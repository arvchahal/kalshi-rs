use crate::common::setup_client;
use kalshi_rs::milestones::models::*;
use std::time::Duration;
use tokio::time::sleep;
#[tokio::test]
async fn test_get_milestones_basic() {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let client = setup_client();
    let result = client.get_milestones(Some(10)).await;
    assert!(result.is_ok(), "Failed to fetch: {:?}", result.err());
    let resp = result.unwrap();
    assert!(! resp.milestones.is_empty(), "Expected at least one milestone to exist");
}
#[tokio::test]
async fn test_get_single_milestone() {
    let client = setup_client();
    let list = client.get_milestones(Some(5)).await.expect("Failed to list milestones");
    if list.milestones.is_empty() {
        return;
    }
    sleep(Duration::from_secs(2)).await;
    let milestone_id = &list.milestones[0].id;
    let result = client.get_milestone(milestone_id).await;
    assert!(
        result.is_ok(), "Failed to get milestone {}: {:?}", milestone_id, result.err()
    );
}
#[tokio::test]
async fn test_milestones_endpoints_all() {
    let client = setup_client();
    let list = client.get_milestones(Some(20)).await.expect("Failed to get milestones");
    sleep(Duration::from_secs(2)).await;
    if let Some(first) = list.milestones.first() {
        let _single = client.get_milestone(&first.id).await.expect("Failed to fetch");
    }
}
