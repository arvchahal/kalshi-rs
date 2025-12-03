use crate::common::setup_client;
use kalshi_rs::structured_targets::models::*;
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn test_get_all_structured_targets() {
    let client = setup_client();
    let result = client.get_all_structured_targets(Some(10), None).await;
    assert!(result.is_ok(), "Failed to get all structured targets: {:?}", result.err());
    let resp = result.unwrap();
    println!("Retrieved {} structured targets", resp.structured_targets.len());
    if !resp.structured_targets.is_empty() {
        let first = &resp.structured_targets[0];
        println!(
            "First target: ID={}, Name={}, Type={}", first.id, first.name, first.r#type
        );
    }
}
#[tokio::test]
async fn test_get_all_structured_targets_with_limit() {
    let client = setup_client();
    let result = client.get_all_structured_targets(Some(5), None).await;
    assert!(result.is_ok(), "Failed to get structured targets with limit");
    let resp = result.unwrap();
    println!("Requested limit: 5, received: {} targets", resp.structured_targets.len());
    assert!(! resp.structured_targets.is_empty(), "Expected at least some targets");
}
#[tokio::test]
async fn test_get_single_structured_target() {
    let client = setup_client();
    let list = client
        .get_all_structured_targets(Some(5), None)
        .await
        .expect("Failed to fetch structured targets list");
    if list.structured_targets.is_empty() {
        println!("No structured targets available, skipping test");
        return;
    }
    sleep(Duration::from_secs(2)).await;
    let target_id = &list.structured_targets[0].id;
    println!("Testing with structured target ID: {}", target_id);
    let result = client.get_structured_target(target_id).await;
    assert!(
        result.is_ok(), "Failed to get structured target {}: {:?}", target_id, result
        .err()
    );
    let target = result.unwrap();
    println!("Retrieved target: {}", target.structured_target);
    assert_eq!(& target.structured_target.id, target_id, "Target ID mismatch");
}
#[tokio::test]
async fn test_structured_targets_endpoints_comprehensive() {
    let client = setup_client();
    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE STRUCTURED TARGETS TEST");
    println!("{}\n", "=".repeat(80));
    println!("1. Getting all structured targets...");
    let list = client
        .get_all_structured_targets(Some(10), None)
        .await
        .expect("Failed to get structured targets");
    println!("   Found {} targets", list.structured_targets.len());
    if let Some(cursor) = &list.cursor {
        println!("   Cursor: {}", cursor);
    }
    if list.structured_targets.is_empty() {
        println!("   No structured targets available, ending test");
        return;
    }
    sleep(Duration::from_secs(2)).await;
    println!("\n2. Getting details for first target...");
    let first_target = &list.structured_targets[0];
    println!("   Target ID: {}", first_target.id);
    println!("   Name: {}", first_target.name);
    println!("   Type: {}", first_target.r#type);
    sleep(Duration::from_secs(2)).await;
    let details = client
        .get_structured_target(&first_target.id)
        .await
        .expect("Failed to get target details");
    println!("   Retrieved details: {}", details.structured_target);
    println!("\n3. Verifying basic structure of targets...");
    for target in &list.structured_targets {
        assert!(! target.id.is_empty(), "Target has empty ID");
        assert!(! target.r#type.is_empty(), "Target has empty type");
    }
    println!("   All {} targets have valid structure", list.structured_targets.len());
    println!("\n{}", "=".repeat(80));
    println!("ALL STRUCTURED TARGETS TESTS PASSED");
    println!("{}\n", "=".repeat(80));
}
#[tokio::test]
async fn test_structured_targets_pagination() {
    let client = setup_client();
    let page1 = client
        .get_all_structured_targets(Some(3), None)
        .await
        .expect("Failed to get first page");
    println!("Page 1: {} targets", page1.structured_targets.len());
    if let Some(cursor) = &page1.cursor {
        println!("Cursor available for pagination: {}", cursor);
        sleep(Duration::from_secs(2)).await;
        let page2 = client
            .get_all_structured_targets(Some(3), Some(cursor))
            .await
            .expect("Failed to get second page");
        println!("Page 2: {} targets", page2.structured_targets.len());
        let page1_ids: Vec<&str> = page1
            .structured_targets
            .iter()
            .map(|t| t.id.as_str())
            .collect();
        let page2_ids: Vec<&str> = page2
            .structured_targets
            .iter()
            .map(|t| t.id.as_str())
            .collect();
        assert!(! page2.structured_targets.is_empty(), "Page 2 should have targets");
        if !page1_ids.is_empty() && !page2_ids.is_empty() {
            assert_ne!(
                page1_ids[0], page2_ids[0],
                "Page 2 should have different targets than page 1"
            );
        }
    } else {
        println!("No cursor returned - all results fit in one page");
    }
}
