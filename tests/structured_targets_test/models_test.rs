use kalshi_rs::structured_targets::models::*;
#[test]


fn test_structured_target_deserialization() {
    let json = r#"{
        "id": "test-id-123",
        "name": "Test Target",
        "type": "basketball_team",
        "details": {
            "team_name": "Lakers",
            "league": "NBA"
        },
        "source_id": "source-123",
        "last_updated_ts": "2024-01-15T10:30:00Z"
    }"#;
    let target: StructuredTarget = serde_json::from_str(json).unwrap();
    assert_eq!(target.id, "test-id-123");
    assert_eq!(target.name, "Test Target");
    assert_eq!(target.r#type, "basketball_team");
    assert_eq!(target.source_id, Some("source-123".to_string()));
    assert_eq!(target.last_updated_ts, "2024-01-15T10:30:00Z");
    assert!(target.details.is_object());
}
#[test]


fn test_get_structured_targets_response_deserialization() {
    let json = r#"{
        "structured_targets": [
            {
                "id": "target-1",
                "name": "Target One",
                "type": "team",
                "details": {},
                "source_id": "src-1",
                "last_updated_ts": "2024-01-01T00:00:00Z"
            },
            {
                "id": "target-2",
                "name": "Target Two",
                "type": "player",
                "details": {},
                "source_id": "src-2",
                "last_updated_ts": "2024-01-02T00:00:00Z"
            }
        ],
        "cursor": "next-page-cursor"
    }"#;
    let response: GetStructuredTargetsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.structured_targets.len(), 2);
    assert_eq!(response.cursor, Some("next-page-cursor".to_string()));
    assert_eq!(response.structured_targets[0].id, "target-1");
    assert_eq!(response.structured_targets[1].id, "target-2");
}
#[test]


fn test_get_structured_targets_response_no_cursor() {
    let json = r#"{
        "structured_targets": [
            {
                "id": "target-1",
                "name": "Target One",
                "type": "team",
                "details": {},
                "source_id": "src-1",
                "last_updated_ts": "2024-01-01T00:00:00Z"
            }
        ],
        "cursor": null
    }"#;
    let response: GetStructuredTargetsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.structured_targets.len(), 1);
    assert_eq!(response.cursor, None);
}
#[test]


fn test_get_structured_target_response_deserialization() {
    let json = r#"{
        "structured_target": {
            "id": "single-target",
            "name": "Single Target",
            "type": "venue",
            "details": {
                "location": "New York",
                "capacity": 20000
            },
            "source_id": "venue-src",
            "last_updated_ts": "2024-03-15T14:30:00Z"
        }
    }"#;
    let response: GetStructuredTargetResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.structured_target.id, "single-target");
    assert_eq!(response.structured_target.name, "Single Target");
    assert_eq!(response.structured_target.r#type, "venue");
    assert_eq!(response.structured_target.source_id, Some("venue-src".to_string()));
}
#[test]


fn test_structured_targets_query_serialization() {
    let query = StructuredTargetsQuery {
        limit: Some(10),
        cursor: None,
    };
    let serialized = serde_urlencoded::to_string(&query).unwrap();
    assert_eq!(serialized, "limit=10");
}
#[test]


fn test_structured_targets_query_no_limit() {
    let query = StructuredTargetsQuery {
        limit: None,
        cursor: None,
    };
    let serialized = serde_urlencoded::to_string(&query).unwrap();
    assert_eq!(serialized, "");
}
#[test]


fn test_structured_targets_query_with_cursor() {
    let query = StructuredTargetsQuery {
        limit: Some(5),
        cursor: Some("test-cursor-123".to_string()),
    };
    let serialized = serde_urlencoded::to_string(&query).unwrap();
    assert_eq!(serialized, "limit=5&cursor=test-cursor-123");
}
#[test]


fn test_structured_targets_query_cursor_only() {
    let query = StructuredTargetsQuery {
        limit: None,
        cursor: Some("cursor-abc".to_string()),
    };
    let serialized = serde_urlencoded::to_string(&query).unwrap();
    assert_eq!(serialized, "cursor=cursor-abc");
}
#[test]


fn test_structured_target_display() {
    let target = StructuredTarget {
        id: "test-id".to_string(),
        name: "Test Name".to_string(),
        r#type: "test_type".to_string(),
        details: serde_json::json!({}),
        source_id: Some("src-id".to_string()),
        last_updated_ts: "2024-01-01T00:00:00Z".to_string(),
    };
    let display = format!("{}", target);
    assert!(display.contains("test-id"));
    assert!(display.contains("Test Name"));
    assert!(display.contains("test_type"));
    assert!(display.contains("src-id"));
}
#[test]


fn test_structured_target_with_complex_details() {
    let json = r#"{
        "id": "complex-target",
        "name": "Complex Target",
        "type": "basketball_team",
        "details": {
            "team_name": "Golden State Warriors",
            "conference": "Western",
            "division": "Pacific",
            "stats": {
                "wins": 45,
                "losses": 20
            }
        },
        "source_id": "nba-api",
        "last_updated_ts": "2024-11-14T00:00:00Z"
    }"#;
    let target: StructuredTarget = serde_json::from_str(json).unwrap();
    assert_eq!(target.id, "complex-target");
    let details = &target.details;
    assert_eq!(details["team_name"], "Golden State Warriors");
    assert_eq!(details["stats"] ["wins"], 45);
}
#[test]


fn test_empty_structured_targets_list() {
    let json = r#"{
        "structured_targets": [],
        "cursor": null
    }"#;
    let response: GetStructuredTargetsResponse = serde_json::from_str(json).unwrap();
    assert!(response.structured_targets.is_empty());
    assert_eq!(response.cursor, None);
}
#[test]


fn test_structured_target_missing_source_id() {
    let json = r#"{
        "id": "test-id-123",
        "name": "Test Target",
        "type": "basketball_team",
        "details": {},
        "last_updated_ts": "2024-01-15T10:30:00Z"
    }"#;
    let target: StructuredTarget = serde_json::from_str(json).unwrap();
    assert_eq!(target.id, "test-id-123");
    assert_eq!(target.source_id, None);
}
