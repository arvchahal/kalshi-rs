use crate::common::setup_client;
use kalshi_rs::api_keys::models::*;
use std::time::Duration;
use tokio::time::sleep;
/// api keys endpoints except the one where you provide your own api key because I was lazy
#[tokio::test]
async fn test_get_api_keys_list() {
    let client = setup_client();
    println!("\n=== API KEYS: GET LIST ===");
    let result = client.get_api_keys().await;
    assert!(result.is_ok(), "Failed to fetch API keys: {:?}", result.err());
    let keys = result.unwrap();
    println!("Retrieved {} API keys.", keys.len());
    for key in &keys {
        println!("• {} — {}", key.api_key_id, key.name);
    }
    assert!(! keys.is_empty(), "Expected at least one API key to be present");
}
#[tokio::test]
async fn test_generate_and_delete_api_key() {
    let client = setup_client();
    println!("\n API KEYS: GENERATE + DELETE");
    let new_key_name = format!("SDK_Test_Key_{}", chrono::Utc::now().timestamp());
    let result = client.generate_api_key(&new_key_name).await;
    assert!(result.is_ok(), "Failed to generate API key: {:?}", result.err());
    let created = result.unwrap();
    println!(
        "Created new key: id={} | first line of private_key={:?}", created.api_key_id,
        created.private_key.lines().next().unwrap_or("n/a")
    );
    sleep(Duration::from_secs(1)).await;
    let delete_result = client.delete_api_key(&created.api_key_id).await;
    match delete_result {
        Ok(resp) => {
            println!("Deleted key {} successfully.", created.api_key_id);
            if let Some(body) = resp.body {
                println!("Delete body: {}", body);
            }
        }
        Err(e) => {
            panic!("Failed to delete key {}", created.api_key_id);
        }
    }
}
#[tokio::test]
async fn test_api_keys_endpoints_comprehensive() {
    let client = setup_client();
    println!("Listing existing API keys...");
    let keys = client.get_api_keys().await.expect("Failed to list API keys");
    println!("   Retrieved {} keys", keys.len());
    sleep(Duration::from_secs(2)).await;
    println!("Generating a temporary test key...");
    let key_name = format!("SDK_Comprehensive_{}", chrono::Utc::now().timestamp());
    let new_key = client
        .generate_api_key(&key_name)
        .await
        .expect("Failed to generate new key");
    println!("   New key created: {}", new_key.api_key_id);
    sleep(Duration::from_secs(2)).await;
    let _ = client
        .delete_api_key(&new_key.api_key_id)
        .await
        .expect("Failed to delete key");
    println!("   Successfully deleted key {}", new_key.api_key_id);
}
