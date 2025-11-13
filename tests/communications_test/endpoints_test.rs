use crate::common::setup_client;
use kalshi_rust_sdk::communications::models::*;
use std::time::Duration;
use tokio::time::sleep;

/// =============================================================================
/// COMMUNICATIONS ENDPOINTS TESTS (RFQs + QUOTES)
/// =============================================================================

#[tokio::test]
async fn test_get_rfqs_list() {
    let client = setup_client();

    println!("\n=== COMMUNICATIONS: GET RFQs LIST ===");
    let result = client.get_rfqs().await;
    assert!(result.is_ok(), "Failed to get RFQs: {:?}", result.err());

    let resp = result.unwrap();
    println!("Retrieved {} RFQs", resp.rfqs.len());

    if let Some(cursor) = &resp.cursor {
        println!("Next page cursor: {}", cursor);
    }

    if let Some(rfq) = resp.rfqs.first() {
        println!(
            "Sample RFQ: id={} | market_ticker={} | status={}",
            rfq.id, rfq.market_ticker, rfq.status
        );
    }
}

#[tokio::test]
async fn test_get_quotes_list() {
    let client = setup_client();

    println!("\n=== COMMUNICATIONS: GET QUOTES LIST ===");
    let result = client
        .get_quotes(None, None, None, Some(10), None, None, None, None)
        .await;
    assert!(result.is_ok(), "Failed to get quotes: {:?}", result.err());

    let resp = result.unwrap();
    println!("Retrieved {} quotes", resp.quotes.len());

    if let Some(cursor) = &resp.cursor {
        println!("Next page cursor: {}", cursor);
    }

    if let Some(q) = resp.quotes.first() {
        println!(
            "Sample Quote: id={} | rfq_id={} | status={} | market_ticker={}",
            q.id, q.rfq_id, q.status, q.market_ticker
        );
    }
}

#[tokio::test]
async fn test_get_communications_id() {
    let client = setup_client();

    println!("\n=== COMMUNICATIONS: GET COMMUNICATIONS ID ===");
    let result = client.get_communications_id().await;
    assert!(
        result.is_ok(),
        "Failed to get communications ID: {:?}",
        result.err()
    );

    let resp = result.unwrap();
    println!("Communication ID: {}", resp.communcation_id);
    assert!(
        !resp.communcation_id.is_empty(),
        "Expected non-empty communications ID"
    );
}

#[tokio::test]
async fn test_create_and_delete_rfq_lifecycle() {
    let client = setup_client();

    println!("\n=== COMMUNICATIONS: CREATE + DELETE RFQ ===");

    // ⚙️ Create RFQ
    let rfq_body = CreateRFQRequest {
        market_ticker: "KXMVENFLSINGLEGAME-S2025B3F84FCFC70-DB6D0E930C8".to_string(),
        rest_remainder: false,
        contracts: Some(1),
        target_cost_centi_cents: Some(5000),
        replace_existing: None,
        subtrader_id: None,
    };

    let created = client
        .create_rfq(&rfq_body)
        .await
        .expect("Failed to create RFQ");
    println!("Created RFQ id={}", created.id);

    sleep(Duration::from_secs(2)).await;

    // 🗑️ Delete RFQ
    let deleted = client
        .delete_rfq(&created.id)
        .await
        .expect("Failed to delete RFQ");
    println!("Deleted RFQ id={} response={:?}", created.id, deleted.body);
}

#[tokio::test]
async fn test_create_quote_and_accept_flow() {
    let client = setup_client();

    println!("\n=== COMMUNICATIONS: CREATE QUOTE + ACCEPT FLOW ===");

    // NOTE: RFQ must exist — use one from get_rfqs()
    let rfqs = client.get_rfqs().await.expect("Failed to get RFQs");
    if rfqs.rfqs.is_empty() {
        println!("⚠️ No RFQs available to create quote for — skipping test");
        return;
    }
    let rfq_id = &rfqs.rfqs[0].id;
    println!("Using RFQ id={}", rfq_id);

    // Create quote
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
    println!("Created quote id={}", created.id);

    sleep(Duration::from_secs(2)).await;

    // Accept quote (side = yes)
    let accept_result = client.accept_quote(&created.id, "yes").await;
    match accept_result {
        Ok(resp) => println!("Accepted quote id={} resp={:?}", created.id, resp.body),
        Err(e) => println!("⚠️ Accept failed (expected if not fillable): {:?}", e),
    }

    sleep(Duration::from_secs(2)).await;

    // Confirm quote
    let confirm_result = client.confirm_quote(&created.id).await;
    match confirm_result {
        Ok(resp) => println!("Confirmed quote id={} resp={:?}", created.id, resp.body),
        Err(e) => println!("⚠️ Confirm failed (may be valid): {:?}", e),
    }

    // Delete quote to clean up
    let del_result = client.delete_quote(&created.id).await;
    match del_result {
        Ok(resp) => println!("Deleted quote id={} resp={:?}", created.id, resp.body),
        Err(e) => println!("⚠️ Delete failed (may already be gone): {:?}", e),
    }
}

#[tokio::test]
async fn test_communications_endpoints_comprehensive() {
    let client = setup_client();

    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE COMMUNICATIONS ENDPOINTS TEST");
    println!("{}", "=".repeat(80));

    // 1️⃣ Get communications ID
    let comm_id = client
        .get_communications_id()
        .await
        .expect("Failed to get communications ID");
    println!("Communication ID: {}\n", comm_id.communcation_id);
    sleep(Duration::from_secs(2)).await;

    // 2️⃣ List RFQs
    let rfqs = client.get_rfqs().await.expect("Failed to list RFQs");
    println!("RFQs retrieved: {}\n", rfqs.rfqs.len());
    sleep(Duration::from_secs(2)).await;

    // 3️⃣ List Quotes
    let quotes = client
        .get_quotes(None, None, None, Some(5), None, None, None, None)
        .await
        .expect("Failed to list quotes");
    println!("Quotes retrieved: {}\n", quotes.quotes.len());

    println!("{}", "=".repeat(80));
    println!("ALL COMMUNICATIONS ENDPOINTS TESTS PASSED ✅");
    println!("{}", "=".repeat(80));
}
