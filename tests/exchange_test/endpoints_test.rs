use crate::common::setup_client;
use kalshi_rs::exchange::models::*;
use std::time::Duration;
use tokio::time::sleep;
/// =============================================================================
/// EXCHANGE ANNOUNCEMENTS TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_exchange_announcements() {
    let client = setup_client();
    let result = client.get_exchange_announcements().await;
    assert!(result.is_ok(), "Failed to get exchange announcements: {:?}", result.err());
    let announcements = result.unwrap();
    println!("Exchange announcements count: {}", announcements.announcements.len());
    if announcements.announcements.is_empty() {
        println!("No exchange announcements currently available (OK).");
    } else {
        println!("First announcement: {}", announcements.announcements[0]);
    }
}
/// =============================================================================
/// EXCHANGE SCHEDULE TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_exchange_schedule() {
    let client = setup_client();
    let result = client.get_exchange_schedule().await;
    assert!(result.is_ok(), "Failed to get exchange schedule: {:?}", result.err());
    let schedule_response = result.unwrap();
    let schedule = schedule_response.schedule;
    println!("Maintenance windows: {:?}", schedule.maintenance_windows);
    println!("Standard hours entries: {}", schedule.standard_hours.len());
    for (i, hours) in schedule.standard_hours.iter().enumerate() {
        println!(
            "Standard Hours [{}]: start={}, end={}", i, hours.start_time, hours.end_time
        );
        assert!(
            hours.monday.is_empty() || hours.monday.iter().all(| d | ! d.close_time
            .is_empty())
        );
    }
}
/// =============================================================================
/// EXCHANGE STATUS TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_exchange_status() {
    let client = setup_client();
    let result = client.get_exchange_status().await;
    assert!(result.is_ok(), "Failed to get exchange status: {:?}", result.err());
    let status = result.unwrap();
    println!(
        "Exchange active: {}, Trading active: {}, Resume time: {:?}", status
        .exchange_active, status.trading_active, status.exchange_estimated_resume_time
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
    assert!(result.is_ok(), "Failed to get user data timestamp: {:?}", result.err());
    let timestamp = result.unwrap();
    println!("User data last updated at: {}", timestamp.as_of_time);
    assert!(
        timestamp.as_of_time.contains("T") && timestamp.as_of_time.contains("Z"),
        "Invalid timestamp format: {}", timestamp.as_of_time
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
    println!("1. Getting exchange announcements...");
    let announcements = client
        .get_exchange_announcements()
        .await
        .expect("Failed to get announcements");
    println!("   Announcements: {}\n", announcements.announcements.len());
    sleep(Duration::from_secs(2)).await;
    println!("2. Getting exchange schedule...");
    let schedule = client.get_exchange_schedule().await.expect("Failed to get schedule");
    println!(
        "   Maintenance windows: {}, Standard hours: {}\n", schedule.schedule
        .maintenance_windows.len(), schedule.schedule.standard_hours.len()
    );
    sleep(Duration::from_secs(2)).await;
    println!("3. Getting exchange status...");
    let status = client
        .get_exchange_status()
        .await
        .expect("Failed to get exchange status");
    println!(
        "   Exchange active: {}, Trading active: {}, Resume: {:?}\n", status
        .exchange_active, status.trading_active, status.exchange_estimated_resume_time
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
