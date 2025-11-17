use crate::common::setup_client;
use kalshi_rs::multivariate_collections::models::*;
use std::time::Duration;
use tokio::time::sleep;
#[tokio::test]
async fn test_get_multivariate_event_collections_list() {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let client = setup_client();
    let result = client.get_multivariate_event_collections().await;
    assert!(
        result.is_ok(), "Failed to get multivariate collections: {:?}", result.err()
    );
    let resp = result.unwrap();
    assert!(
        ! resp.multivariate_contracts.is_empty(),
        "should be at least one multivariate collection"
    );
}
#[tokio::test]
async fn test_get_single_multivariate_event_collection() {
    let client = setup_client();
    let list = client
        .get_multivariate_event_collections()
        .await
        .expect("Failed to list multivariate collections");
    if list.multivariate_contracts.is_empty() {
        return;
    }
    sleep(Duration::from_secs(2)).await;
    let ticker = &list.multivariate_contracts[0].collection_ticker;
    let result = client.get_multivariate_event_collection(ticker).await;
    assert!(result.is_ok(), "Failed for MVC {}: {:?}", ticker, result.err());
}
#[tokio::test]
async fn test_multivariate_collections_endpoints_all() {
    let client = setup_client();
    let collections = client
        .get_multivariate_event_collections()
        .await
        .expect("Failed to list multivariate collections");
    sleep(Duration::from_secs(2)).await;
    if let Some(first) = collections.multivariate_contracts.first() {
        let _details = client
            .get_multivariate_event_collection(&first.collection_ticker)
            .await
            .expect("Failed to get collection details");
    }
}
