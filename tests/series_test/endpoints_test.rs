use crate::common::setup_client;
use kalshi_rs::series::models::*;
use tokio::time::{sleep, Duration};
#[tokio::test]
async fn test_get_all_series() {
    let client = setup_client();
    let result = client.get_all_series(Some(10), None).await;
    assert!(result.is_ok(), "fail to get all series: {:?}", result.err());
    let resp = result.unwrap();
    assert!(! resp.series.is_empty(), "expected at least 1 series to be returned");
}
#[tokio::test]
async fn test_get_single_series() {
    let client = setup_client();
    let list = client
        .get_all_series(Some(5), None)
        .await
        .expect("Failed to fetch series list");
    if list.series.is_empty() {
        return;
    }
    sleep(Duration::from_secs(2)).await;
    let ticker = "KXATTYGENID";
    let result = client.get_series_by_ticker(ticker).await;
    assert!(
        result.is_ok(), "fail to get series by ticker {}: {:?}", ticker, result.err()
    );
}
#[tokio::test]
async fn test_series_endpoints_all() {
    let client = setup_client();
    let list = client
        .get_all_series(Some(10), None)
        .await
        .expect("Failed to list series");
    sleep(Duration::from_secs(2)).await;
    if let Some(first) = list.series.first() {
        let ticker = &first.ticker;
        let _details = client
            .get_series_by_ticker(ticker)
            .await
            .expect("Failed to get series details");
    }
}
