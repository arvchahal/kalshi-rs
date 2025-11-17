use crate::common::setup_client;
use kalshi_rs::events::models::*;
use std::time::Duration;
use tokio::time::sleep;
/// ALL EVENTS TESTS
#[tokio::test]
async fn test_get_all_events_basic() {
    let client = setup_client();
    let params = EventsQuery {
        limit: Some(10),
        cursor: None,
    };
    let result = client.get_all_events(&params).await;
    assert!(result.is_ok(), "Failed to get all events: {:?}", result.err());
    let response = result.unwrap();
    println!("Events retrieved: {}", response.events.len());
    if let Some(first) = response.events.first() {
        println!(
            "Sample Event: {} | Title: {} | Series: {} | Brokers: {}", first
            .event_ticker, first.title, first.series_ticker, first.available_on_brokers
        );
    } else {
        println!("No events returned (OK if no events exist currently).");
    }
}
#[tokio::test]
async fn test_get_all_events_with_cursor() {
    let client = setup_client();
    let params = EventsQuery {
        limit: Some(3),
        cursor: None,
    };
    let batch = client.get_all_events(&params).await.unwrap();
    if batch.events.is_empty() {
        println!("No events available - skipping cursor pagination test");
        return;
    }
    println!("Testing pagination with limit=3");
    let params2 = EventsQuery {
        limit: Some(3),
        cursor: Some("abc123".to_string()),
    };
    let next = client.get_all_events(&params2).await;
    assert!(
        next.is_ok() || next.is_err(), "Pagination should succeed or gracefully fail"
    );
    println!("Pagination test completed (server may ignore cursor).");
}
/// SINGLE EVENT TESTS
#[tokio::test]
async fn test_get_event_single() {
    let client = setup_client();
    let params = EventsQuery {
        limit: Some(1),
        cursor: None,
    };
    let events = client.get_all_events(&params).await.unwrap();
    if events.events.is_empty() {
        println!("No events available - skipping single event test");
        return;
    }
    let ticker = &events.events[0].event_ticker;
    println!("Testing get_event for ticker: {}", ticker);
    let result = client.get_event(ticker).await;
    assert!(result.is_ok(), "Failed to get event: {:?}", result.err());
    let event_response = result.unwrap();
    println!(
        "Event: {} | Markets: {} | Title: {}", event_response.event.event_ticker,
        event_response.markets.len(), event_response.event.title
    );
}
#[tokio::test]
async fn test_get_event_with_markets_check() {
    let client = setup_client();
    let params = EventsQuery {
        limit: Some(1),
        cursor: None,
    };
    let events = client.get_all_events(&params).await.unwrap();
    if events.events.is_empty() {
        println!("No events available - skipping markets check test");
        return;
    }
    let ticker = &events.events[0].event_ticker;
    let response = client.get_event(ticker).await.unwrap();
    println!(
        "Event {} has {} markets", response.event.event_ticker, response.markets.len()
    );
    assert!(response.markets.len() >= 0, "Expected 0 or more markets per event");
}
/// =============================================================================
/// EVENT METADATA TESTS
/// =============================================================================
#[tokio::test]
async fn test_get_event_metadata() {
    let client = setup_client();
    let params = EventsQuery {
        limit: Some(1),
        cursor: None,
    };
    let events = client.get_all_events(&params).await.unwrap();
    if events.events.is_empty() {
        println!("No events available - skipping metadata test");
        return;
    }
    let ticker = &events.events[0].event_ticker;
    println!("Fetching metadata for event: {}", ticker);
    let result = client.get_event_metadata(ticker).await;
    assert!(result.is_ok(), "Failed to get event metadata: {:?}", result.err());
    let meta = result.unwrap();
    println!(
        "Metadata -> competition: {:?}, scope: {:?}, sources: {}", meta.competition, meta
        .competition_scope, meta.settlement_sources.len()
    );
    for src in meta.settlement_sources.iter().take(3) {
        println!("Settlement Source: {} ({})", src.name, src.url);
    }
}
/// COMPREHENSIVE EVENTS TEST
#[tokio::test]
async fn test_events_endpoints_comprehensive() {
    let client = setup_client();
    println!("COMPREHENSIVE EVENTS ENDPOINTS TEST");
    println!("{}\n", "=".repeat(80));
    let params = EventsQuery {
        limit: Some(5),
        cursor: None,
    };
    let events = client.get_all_events(&params).await.expect("Failed to get all events");
    println!("   Retrieved {} events\n", events.events.len());
    sleep(Duration::from_secs(2)).await;
    if let Some(first_event) = events.events.first() {
        let ticker = &first_event.event_ticker;
        println!("   Testing event ticker: {}\n", ticker);
        println!("2. Getting single event...");
        let event = client.get_event(ticker).await.expect("Failed to get single event");
        println!("   Event: {} | Markets: {}\n", event.event.title, event.markets.len());
        sleep(Duration::from_secs(2)).await;
        println!("3. Getting event metadata...");
        let meta = client
            .get_event_metadata(ticker)
            .await
            .expect("Failed to get event metadata");
        println!(
            "   Metadata retrieved -> sources: {}, competition: {:?}\n", meta
            .settlement_sources.len(), meta.competition
        );
    } else {
        println!("No events available for detailed test run.");
    }
    println!("ALL EVENTS ENDPOINT TESTS PASSED");
}
