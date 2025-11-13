use crate::common::setup_client;
use kalshi_rust_sdk::multivariate_collections::models::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_get_multivariate_event_collections_list() {
    let client = setup_client();

    let result = client.get_multivariate_event_collections().await;
    assert!(
        result.is_ok(),
        "Failed to get multivariate collections: {:?}",
        result.err()
    );

    let resp = result.unwrap();
    println!("Retrieved {} multivariate collections", resp.multivariate_contracts.len());

    if let Some(cursor) = &resp.cursor {
        println!("Next page cursor: {}", cursor);
    }

    if let Some(contract) = resp.multivariate_contracts.first() {
        println!(
            "Sample Contract: {} | Title: {} | Markets: {} events",
            contract.collection_ticker,
            contract.title,
            contract.associated_events.len()
        );
    }

    assert!(
        !resp.multivariate_contracts.is_empty(),
        "should be at least one multivariate collection"
    );
}

#[tokio::test]
async fn test_get_single_multivariate_event_collection() {
    let client = setup_client();

    println!("\n SINGLE MVE");

    // Fetch list first
    let list = client
        .get_multivariate_event_collections()
        .await
        .expect("Failed to list multivariate collections");

    if list.multivariate_contracts.is_empty() {
        println!(" No MV Coll available — skipping");
        return;
    }

    let ticker = &list.multivariate_contracts[0].collection_ticker;
    println!("details for ticker: {}", ticker);

    let result = client.get_multivariate_event_collection(ticker).await;
    assert!(
        result.is_ok(),
        "Fail for MVC {}: {:?}",
        ticker,
        result.err()
    );

    let resp = result.unwrap();
    let contract = &resp.multivariate_contract;
    println!(
        "Fetched : {} | Title: {} | Functional Description: {}",
        contract.collection_ticker, contract.title, contract.functional_description
    );

    println!(
        "Associated Events: {} | Ordered: {} | AllYes: {}",
        contract.associated_events.len(),
        contract.is_ordered,
        contract.is_all_yes
    );
}

#[tokio::test]
async fn test_multivariate_collections_endpoints_all() {
    let client = setup_client();


    let collections = client
        .get_multivariate_event_collections()
        .await
        .expect("Failed to list multivariate collections");
    println!("Retrieved {} total collections", collections.multivariate_contracts.len());
    sleep(Duration::from_secs(2)).await;

    if let Some(first) = collections.multivariate_contracts.first() {
        println!("Getting details for collection: {}", first.collection_ticker);
        let details = client
            .get_multivariate_event_collection(&first.collection_ticker)
            .await
            .expect("Failed to get collection details");
        println!(
            "Collection '{}' has {} associated events",
            details.multivariate_contract.collection_ticker,
            details.multivariate_contract.associated_events.len()
        );
    } else {
        println!("NOne available.");
    }

}
