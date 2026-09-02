//! Client-side rate limiting for the Kalshi API.
//!
//! Kalshi meters requests with a per-second token budget that is tracked
//! separately for reads and writes. Most endpoints cost 10 tokens, so a Basic
//! tier key (200 read tokens/sec) sustains roughly 20 reads/sec. Exceeding the
//! budget returns `HTTP 429` with no `Retry-After` header, so clients are
//! expected to pace themselves and back off on their own.
//!
//! [`RateLimiter`] implements the same token-bucket model locally: requests
//! wait for capacity instead of being rejected by the server.
//!
//! ```no_run
//! use kalshi_rs::{Account, KalshiClient, RateLimitTier};
//!
//! # fn example(account: Account) {
//! let client = KalshiClient::new(account).with_rate_limit(RateLimitTier::Basic);
//! # }
//! ```

use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// Default token cost of a single request.
///
/// Kalshi charges 10 tokens for most endpoints; a few cost more. Using the
/// default keeps the limiter conservative for the common case.
pub const DEFAULT_REQUEST_COST: u32 = 10;

/// Published per-second token budgets for each Kalshi access tier.
///
/// Values are tokens per second, as documented under "Rate Limits".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitTier {
    Basic,
    Advanced,
    Expert,
    Premier,
    Paragon,
    Prime,
    Prestige,
}

impl RateLimitTier {
    /// Returns `(read_tokens_per_sec, write_tokens_per_sec)` for the tier.
    pub fn budgets(self) -> (u32, u32) {
        match self {
            RateLimitTier::Basic => (200, 100),
            RateLimitTier::Advanced => (300, 300),
            RateLimitTier::Expert => (600, 600),
            RateLimitTier::Premier => (1_000, 1_000),
            RateLimitTier::Paragon => (2_000, 2_000),
            RateLimitTier::Prime => (4_000, 4_000),
            RateLimitTier::Prestige => (10_000, 8_000),
        }
    }
}

/// Whether a request draws from the read or the write budget.
///
/// Kalshi meters `GET` against the read budget and mutating verbs against the
/// write budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestKind {
    Read,
    Write,
}

impl RequestKind {
    /// Classify an HTTP method into the budget it draws from.
    pub fn from_method(method: &str) -> RequestKind {
        if method.eq_ignore_ascii_case("GET") {
            RequestKind::Read
        } else {
            RequestKind::Write
        }
    }
}

/// A continuously refilling token bucket.
struct Bucket {
    /// Tokens currently available. Fractional so slow refills still accrue.
    tokens: f64,
    /// Maximum tokens the bucket holds, i.e. the burst allowance.
    capacity: f64,
    /// Tokens added per second.
    refill_per_sec: f64,
    /// When `tokens` was last brought up to date.
    last_refill: Instant,
}

impl Bucket {
    fn new(per_sec: u32) -> Bucket {
        let per_sec = per_sec.max(1) as f64;
        Bucket {
            tokens: per_sec,
            capacity: per_sec,
            refill_per_sec: per_sec,
            last_refill: Instant::now(),
        }
    }

    /// Credit tokens for the time elapsed since the last update.
    fn refill(&mut self, now: Instant) {
        let elapsed = now.saturating_duration_since(self.last_refill).as_secs_f64();
        if elapsed > 0.0 {
            self.tokens = (self.tokens + elapsed * self.refill_per_sec).min(self.capacity);
            self.last_refill = now;
        }
    }

    /// Take `cost` tokens, returning how long the caller must wait first.
    ///
    /// The tokens are always deducted; a positive return value means the
    /// caller has borrowed against future refill and must sleep before
    /// sending. Deducting up front is what serialises concurrent callers.
    fn take(&mut self, cost: f64) -> Duration {
        self.refill(Instant::now());
        self.tokens -= cost;
        if self.tokens >= 0.0 {
            Duration::ZERO
        } else {
            Duration::from_secs_f64(-self.tokens / self.refill_per_sec)
        }
    }
}

/// Paces outbound requests to stay within Kalshi's token budget.
///
/// Holds an independent bucket for reads and writes, mirroring how Kalshi
/// meters them. Cheap to clone behind an `Arc`; safe to share across tasks.
pub struct RateLimiter {
    read: Mutex<Bucket>,
    write: Mutex<Bucket>,
    /// How many times a 429 is retried before the error is surfaced.
    max_retries: u32,
}

impl RateLimiter {
    /// Build a limiter from Kalshi's published budgets for `tier`.
    pub fn new(tier: RateLimitTier) -> RateLimiter {
        let (read, write) = tier.budgets();
        RateLimiter::with_budgets(read, write)
    }

    /// Build a limiter from explicit per-second token budgets.
    pub fn with_budgets(read_per_sec: u32, write_per_sec: u32) -> RateLimiter {
        RateLimiter {
            read: Mutex::new(Bucket::new(read_per_sec)),
            write: Mutex::new(Bucket::new(write_per_sec)),
            max_retries: 5,
        }
    }

    /// Set how many times a `429` response is retried. Default is 5.
    pub fn with_max_retries(mut self, max_retries: u32) -> RateLimiter {
        self.max_retries = max_retries;
        self
    }

    /// Number of 429 retries this limiter permits.
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Wait until `cost` tokens are available for a request of `kind`.
    pub async fn acquire(&self, kind: RequestKind, cost: u32) {
        let bucket = match kind {
            RequestKind::Read => &self.read,
            RequestKind::Write => &self.write,
        };
        // Deduct under the lock, then sleep with the lock released so other
        // callers can queue behind us rather than serialising on the sleep.
        let wait = {
            let mut b = bucket.lock().await;
            b.take(cost as f64)
        };
        if !wait.is_zero() {
            tokio::time::sleep(wait).await;
        }
    }

    /// Backoff delay before retry `attempt` (0-based) of a 429'd request.
    ///
    /// Exponential with a deterministic jitter offset, capped at 8s. Kalshi's
    /// bucket refills continuously, so a short backoff is usually enough.
    pub fn backoff(&self, attempt: u32) -> Duration {
        let base = Duration::from_millis(250 * 2u64.saturating_pow(attempt.min(5)));
        let jitter = Duration::from_millis((attempt as u64 * 37) % 100);
        (base + jitter).min(Duration::from_secs(8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_budgets_match_documented_values() {
        assert_eq!(RateLimitTier::Basic.budgets(), (200, 100));
        assert_eq!(RateLimitTier::Prestige.budgets(), (10_000, 8_000));
    }

    #[test]
    fn get_is_a_read_everything_else_is_a_write() {
        assert_eq!(RequestKind::from_method("GET"), RequestKind::Read);
        assert_eq!(RequestKind::from_method("get"), RequestKind::Read);
        assert_eq!(RequestKind::from_method("POST"), RequestKind::Write);
        assert_eq!(RequestKind::from_method("DELETE"), RequestKind::Write);
    }

    #[test]
    fn bucket_grants_burst_then_makes_callers_wait() {
        // 200 tokens/sec, 10 per request => 20 requests of burst.
        let mut b = Bucket::new(200);
        for _ in 0..20 {
            assert!(b.take(10.0).is_zero(), "burst capacity should not wait");
        }
        assert!(!b.take(10.0).is_zero(), "should wait once burst is spent");
    }

    #[tokio::test]
    async fn acquire_paces_requests_beyond_the_burst() {
        // 20 tokens/sec with cost 10 => 2 requests immediately, then ~500ms each.
        let limiter = RateLimiter::with_budgets(20, 20);
        let start = Instant::now();
        for _ in 0..4 {
            limiter.acquire(RequestKind::Read, 10).await;
        }
        // 2 free, then 2 paced at ~500ms => at least ~1s total.
        assert!(
            start.elapsed() >= Duration::from_millis(900),
            "expected pacing, took {:?}",
            start.elapsed()
        );
    }

    #[test]
    fn backoff_grows_and_is_capped() {
        let l = RateLimiter::with_budgets(200, 100);
        assert!(l.backoff(0) < l.backoff(3));
        assert!(l.backoff(20) <= Duration::from_secs(8));
    }
}
