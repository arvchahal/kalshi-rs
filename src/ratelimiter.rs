use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Jitter, Quota, RateLimiter,
};
use std::sync::Arc;
use std::{num::NonZeroU32, time::Duration};

/// Represents the Kalshi API Tiers and their associated rate limits.
/// Limits are expressed in Requests Per Second (RPS).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitTier {
    /// Basic: 20 Read, 10 Write
    Basic,
    /// Advanced: 30 Read, 30 Write
    Advanced,
    /// Premier: 100 Read, 100 Write
    Premier,
    /// Prime: 400 Read, 400 Write
    Prime,
    /// Custom: Manually specify (Read, Write) limits
    Custom { read_rps: u32, write_rps: u32 },
}

impl RateLimitTier {
    /// Returns (Read_RPS, Write_RPS)
    pub fn get_limits(&self) -> (u32, u32) {
        match *self {
            Self::Basic => (20, 10),
            Self::Advanced => (30, 30),
            Self::Premier => (100, 100),
            Self::Prime => (400, 400),
            Self::Custom {
                read_rps,
                write_rps,
            } => (read_rps, write_rps),
        }
    }
}

impl Default for RateLimitTier {
    fn default() -> Self {
        Self::Basic
    }
}

type InMemorySingleNodeRatelimiter =
    RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

#[derive(Clone, Debug)]
pub struct KalshiLimiter {
    read_bucket: Arc<InMemorySingleNodeRatelimiter>,
    write_bucket: Arc<InMemorySingleNodeRatelimiter>,
    jitter: Jitter,
}

impl KalshiLimiter {
    pub fn new(tier: RateLimitTier, config: RateLimiterConfig) -> Self {
        let (read_rps, write_rps) = tier.get_limits();

        let make_quota = |rps: u32| {
            let nz = NonZeroU32::new(rps).unwrap_or(NonZeroU32::new(1).unwrap());
            let burst = config.burst.unwrap_or(NonZeroU32::new(1).unwrap());
            Quota::per_second(nz).allow_burst(burst)
        };

        Self {
            read_bucket: Arc::new(RateLimiter::direct(make_quota(read_rps))),
            write_bucket: Arc::new(RateLimiter::direct(make_quota(write_rps))),
            jitter: Jitter::up_to(config.jitter.unwrap_or(Duration::from_millis(10))),
        }
    }

    /// Suspends the current task until a READ token is available.
    pub async fn wait_read(&self) {
        self.read_bucket.until_ready_with_jitter(self.jitter).await;
    }

    /// Suspends the current task until a WRITE token is available.
    pub async fn wait_write(&self) {
        self.write_bucket.until_ready_with_jitter(self.jitter).await;
    }
}

/// Configuration for the internal rate limiter behavior.
/// Useful for fine-tuning burstiness and thread contention jitter.
#[derive(Debug, Default)]
pub struct RateLimiterConfig {
    /// How many requests can be sent instantly before the limit kicks in.
    /// Defaults to 1 (Strict Pacing) if None.
    pub burst: Option<NonZeroU32>,
    /// Randomized delay added to wait times to prevent Thundering Herd.
    /// Defaults to 10ms if None.
    pub jitter: Option<Duration>,
}

impl RateLimiterConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_burst(mut self, burst: NonZeroU32) -> Self {
        self.burst = Some(burst);
        self
    }

    pub fn with_jitter(mut self, jitter: Duration) -> Self {
        self.jitter = Some(jitter);
        self
    }
}
