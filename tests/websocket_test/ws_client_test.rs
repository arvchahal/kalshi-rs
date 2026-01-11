use crate::common::setup_ws_client;

#[tokio::test]
async fn test_connect() {
    let client = setup_ws_client();
    // test connect works
    client.connect().await.unwrap();
    // send req to test sender
    client.list_subscriptions().await.unwrap();
    // if we recieve a message we know we have a connection
    client.next_message().await.unwrap();
}

#[tokio::test]
async fn test_send_message() {
    let client = setup_ws_client();
    client
        .connect()
        .await
        .expect("Failed to connect to WS client. send_message not tested");
    client.send_message(String::from("contents")).await.unwrap();
}

#[tokio::test]
async fn test_build_promotion_request() {
    let client = setup_ws_client();
    client.build_promotion_request().unwrap();
}
