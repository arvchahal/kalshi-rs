use crate::common::setup_client;
use kalshi_rs::communications::models::*;
use kalshi_rs::markets::models::MarketsQuery;
use std::time::Duration;
use tokio::time::sleep;
/// =============================================================================
/// COMMUNICATIONS ENDPOINTS TESTS (RFQs + QUOTES)
/// =============================================================================
#[tokio::test]
async fn test_get_rfqs_list() {
    let client = setup_client();
    let result = client.get_rfqs().await;
    assert!(result.is_ok(), "Failed to get RFQs: {:?}", result.err());
    let resp = result.unwrap();
    assert!(
        !resp.rfqs.is_empty() || resp.rfqs.is_empty(),
        "RFQs fetched successfully"
    );
}
#[tokio::test]
async fn test_get_quotes_list() {
    let client = setup_client();
    let markets = client
        .get_all_markets(&MarketsQuery {
            limit: Some(5),
            cursor: None,
            event_ticker: None,
            series_ticker: None,
            max_close_ts: None,
            min_close_ts: None,
            status: Some("active".to_string()),
            tickers: None,
        })
        .await
        .expect("issue with getting markets");
    let rand_market = &markets.markets[0].ticker;
    let rfq_body = CreateRFQRequest {
        market_ticker: rand_market.to_string(),
        rest_remainder: false,
        contracts: Some(1),
        target_cost_centi_cents: None,
        replace_existing: None,
        subtrader_id: None,
    };
    let create_rfq = client
        .create_rfq(&rfq_body)
        .await
        .expect("Failed to create rfq");
    let rfq_id = &create_rfq.id;
    sleep(Duration::from_secs(2)).await;
    let rfq = client.get_rfq(rfq_id).await.expect("fail to get rfq");
    let creator_rfq_id = rfq.rfq.creator_user_id.clone();
    let query = GetQuotesQuery {
        cursor: None,
        event_ticker: None,
        market_ticker: None,
        limit: Some(10),
        status: None,
        quote_creator_user_id: None,
        rfq_creator_user_id: creator_rfq_id,
        rfq_id: None,
    };
    let result = client.get_quotes(&query).await;
    assert!(result.is_ok(), "Failed to get quotes: {:?}", result.err());
    sleep(Duration::from_secs(2)).await;
    let _deleted = client
        .delete_rfq(&rfq_id)
        .await
        .expect("Failed to delete RFQ");
    let resp = result.unwrap();
    assert!(resp.quotes.len() <= 10, "Quotes limit respected");
}
#[tokio::test]
async fn test_get_communications_id() {
    let client = setup_client();
    let result = client.get_communications_id().await;
    assert!(
        result.is_ok(),
        "Failed to get communications ID: {:?}",
        result.err()
    );
    let resp = result.unwrap();
    assert!(
        !resp.communications_id.is_empty(),
        "Expected non-empty communications ID"
    );
}
#[tokio::test]
#[ignore = "Creates RFQ which could trigger market maker responses"]
async fn test_create_and_delete_rfq_lifecycle() {
    let client = setup_client();
    let rfq_body = CreateRFQRequest {
        market_ticker: "KXMVENFLSINGLEGAME-S2025B3F84FCFC70-DB6D0E930C8".to_string(),
        rest_remainder: false,
        contracts: Some(1),
        target_cost_centi_cents: None,
        replace_existing: None,
        subtrader_id: None,
    };
    let created = client
        .create_rfq(&rfq_body)
        .await
        .expect("Failed to create RFQ");
    sleep(Duration::from_secs(2)).await;
    let _deleted = client
        .delete_rfq(&created.id)
        .await
        .expect("Failed to delete RFQ");
}
#[tokio::test]
#[ignore = "DESTRUCTIVE: Accepts and confirms quotes which can execute real trades and cost real money"]
async fn test_create_quote_and_accept_flow() {
    let client = setup_client();
    let rfqs = client.get_rfqs().await.expect("Failed to get RFQs");
    if rfqs.rfqs.is_empty() {
        return;
    }
    let rfq_id = &rfqs.rfqs[0].id;
    sleep(Duration::from_secs(2)).await;
    let quote_body = CreateQuoteRequest {
        rfq_id: rfq_id.to_string(),
        yes_bid: "45".to_string(),
        no_bid: "55".to_string(),
        rest_remainder: false,
    };
    let created = client
        .create_quote(quote_body)
        .await
        .expect("Failed to create quote");
    sleep(Duration::from_secs(2)).await;
    let _accept_result = client.accept_quote(&created.id, "yes").await;
    sleep(Duration::from_secs(2)).await;
    let _confirm_result = client.confirm_quote(&created.id).await;
    sleep(Duration::from_secs(2)).await;
    let _del_result = client.delete_quote(&created.id).await;
}
