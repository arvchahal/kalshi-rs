use crate::common::setup_client;
use kalshi_rust_sdk::milestones::models::*;
use std::time::Duration;
use tokio::time::sleep;


#[tokio::test]
async fn test_get_milestones_basic() {
    let client = setup_client();

    let result = client.get_milestones(Some(10)).await;
    assert!(result.is_ok(), "Failed to fetch: {:?}", result.err());

    let resp = result.unwrap();
    println!("Retrieved {} milestones", resp.milestones.len());

    if let Some(first) = resp.milestones.first() {
        println!("sample id: {}", first.id);
    } else {
        println!("not foundd");
    }

    // Expecting non-empty list for a healthy environment
    assert!(
        !resp.milestones.is_empty(),
        "Expected at least one milestone to exist"
    );
}

#[tokio::test]
async fn test_get_single_milestone() {
    let client = setup_client();


    // Fetch the list first
    let list = client
        .get_milestones(Some(5))
        .await
        .expect("Failed to list milestones");

    if list.milestones.is_empty() {
        println!("Not available skipping");
        return;
    }

    let milestone_id = &list.milestones[0].id;
    println!("Fetching milestone details for ID: {}", milestone_id);

    let result = client.get_milestone(milestone_id).await;
    assert!(
        result.is_ok(),
        "Failed to get milestone {}: {:?}",
        milestone_id,
        result.err()
    );

    let resp = result.unwrap();
    println!("Milestone retrieved successfully: id={}", resp.milestone.id);
}

#[tokio::test]
async fn test_milestones_endpoints_all() {
    let client = setup_client();


    let list = client
        .get_milestones(Some(20))
        .await
        .expect("Failed to get milestones");
    println!("Retrieved {} total milestones", list.milestones.len());
    sleep(Duration::from_secs(2)).await;

    if let Some(first) = list.milestones.first() {
        println!("Fetching milestone for id={}", first.id);
        let single = client
            .get_milestone(&first.id)
            .await
            .expect("Failed to fetch");
        println!("Milestone  fetched: {}", single.milestone.id);
    } else {
        println!("No milesotnes valailable");
    }

}
