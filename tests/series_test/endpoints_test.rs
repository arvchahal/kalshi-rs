use crate::common::setup_client;
use kalshi_rust_sdk::series::models::*;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_get_all_series() {
    let client = setup_client();

    let result = client.get_all_series(Some(10), None).await;
    assert!(result.is_ok(), "fail to get all series: {:?}", result.err());

    let resp = result.unwrap();
    println!("fetched {} series", resp.series.len());

    if let Some(first) = resp.series.first() {
        println!("sample series: {} ({})", first.ticker, first.title);
    }

    assert!(
        !resp.series.is_empty(),
        " expected at >=1 series to be returned"
    );
}

#[tokio::test]
async fn test_get_single_series() {
    let client = setup_client();

    let list = client
        .get_all_series(Some(5), None)
        .await
        .expect("Failed to fetch series list");

    if list.series.is_empty() {
        println!("No series available; skipping single series test");
        return;
    }

    let ticker = &list.series[0].ticker;
    let result = client.get_series_by_ticker(ticker).await;
    assert!(
        result.is_ok(),
        "fail to get series by ticker {}: {:?}",
        ticker,
        result.err()
    );

    let resp = result.unwrap();
    println!(
        "found series {} (category: {})",
        resp.series.ticker, resp.series.category
    );
}

#[tokio::test]
async fn test_series_endpoints_all() {
    let client = setup_client();

    let list = client
        .get_all_series(Some(10), None)
        .await
        .expect("Failed to list series");

    println!("Retrieved {} total series", list.series.len());
    sleep(Duration::from_secs(2)).await;

    if let Some(first) = list.series.first() {
        let ticker = &first.ticker;
        println!("Fetching series details for ticker: {}", ticker);

        let details = client
            .get_series_by_ticker(ticker)
            .await
            .expect("Failed to get series details");

        println!("Series title: {}", details.series.title);
        println!("Category: {}", details.series.category);
        println!("Tags: {:?}", details.series.tags);
    } else {
        println!("None found");
    }
}
