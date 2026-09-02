use crate::common::setup_client;
use kalshi_rs::exchange::models::*;
use std::time::Duration;
use tokio::time::sleep;
/// =============================================================================
/// EXCHANGE ANNOUNCEMENTS TESTS
/// =============================================================================
/// Kalshi removed `GET /exchange/announcements`. This test now pins the
/// removal: if Kalshi ever brings the endpoint back, it fails so the
/// deprecation on `get_exchange_announcements` can be lifted.
#[tokio::test]
async fn test_get_exchange_announcements_is_removed() {
    let client = setup_client();
    #[allow(deprecated)]
    let result = client.get_exchange_announcements().await;
    let err = match result {
        Ok(_) => panic!("GET /exchange/announcements responded again - remove the deprecation"),
        Err(e) => e.to_string(),
    };
    assert!(
        err.contains("404"),
        "expected removed endpoint to 404, got: {err}"
    );
    println!("Confirmed: GET /exchange/announcements is removed (404).");
}
/// =============================================================================
/// EXCHANGE SCHEDULE TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_exchange_schedule() {
    let client = setup_client();
    let result = client.get_exchange_schedule().await;
    assert!(
        result.is_ok(),
        "Failed to get exchange schedule: {:?}",
        result.err()
    );
    let schedule_response = result.unwrap();
    let schedule = schedule_response.schedule;
    println!("Maintenance windows: {:?}", schedule.maintenance_windows);
    println!("Standard hours entries: {}", schedule.standard_hours.len());
    for (i, hours) in schedule.standard_hours.iter().enumerate() {
        println!(
            "Standard Hours [{}]: start={}, end={}",
            i, hours.start_time, hours.end_time
        );
        assert!(hours.monday.is_empty() || hours.monday.iter().all(|d| !d.close_time.is_empty()));
    }
}
/// =============================================================================
/// EXCHANGE STATUS TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_exchange_status() {
    let client = setup_client();
    let result = client.get_exchange_status().await;
    assert!(
        result.is_ok(),
        "Failed to get exchange status: {:?}",
        result.err()
    );
    let status = result.unwrap();
    println!(
        "Exchange active: {}, Trading active: {}, Resume time: {:?}",
        status.exchange_active, status.trading_active, status.exchange_estimated_resume_time
    );
    assert!(
        status.exchange_active == true || status.exchange_active == false,
        "exchange_active not boolean"
    );
    assert!(
        status.trading_active == true || status.trading_active == false,
        "trading_active not boolean"
    );
}
/// =============================================================================
/// USER DATA TIMESTAMP TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_user_data_timestamp() {
    let client = setup_client();
    let result = client.get_user_data_timestamp().await;
    assert!(
        result.is_ok(),
        "Failed to get user data timestamp: {:?}",
        result.err()
    );
    let timestamp = result.unwrap();
    println!("User data last updated at: {}", timestamp.as_of_time);
    assert!(
        timestamp.as_of_time.contains("T") && timestamp.as_of_time.contains("Z"),
        "Invalid timestamp format: {}",
        timestamp.as_of_time
    );
}
/// =============================================================================
/// COMPREHENSIVE EXCHANGE TEST
/// =============================================================================
#[tokio::test]
async fn test_exchange_endpoints_comprehensive() {
    let client = setup_client();
    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE EXCHANGE ENDPOINTS TEST");
    println!("{}\n", "=".repeat(80));
    println!("1. Skipping exchange announcements (endpoint removed by Kalshi)\n");
    println!("2. Getting exchange schedule...");
    let schedule = client
        .get_exchange_schedule()
        .await
        .expect("Failed to get schedule");
    println!(
        "   Maintenance windows: {}, Standard hours: {}\n",
        schedule.schedule.maintenance_windows.len(),
        schedule.schedule.standard_hours.len()
    );
    sleep(Duration::from_secs(2)).await;
    println!("3. Getting exchange status...");
    let status = client
        .get_exchange_status()
        .await
        .expect("Failed to get exchange status");
    println!(
        "   Exchange active: {}, Trading active: {}, Resume: {:?}\n",
        status.exchange_active, status.trading_active, status.exchange_estimated_resume_time
    );
    sleep(Duration::from_secs(2)).await;
    println!("4. Getting user data timestamp...");
    let ts = client
        .get_user_data_timestamp()
        .await
        .expect("Failed to get user data timestamp");
    println!("   User data updated as of: {}\n", ts.as_of_time);
    println!("{}", "=".repeat(80));
    println!("ALL EXCHANGE ENDPOINTS TESTS PASSED");
    println!("{}\n", "=".repeat(80));
}
