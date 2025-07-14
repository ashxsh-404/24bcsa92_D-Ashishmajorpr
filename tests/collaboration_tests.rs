#[tokio::test]
async fn test_post_message() {
    let token = "Bearer YOUR_TOKEN_HERE";
    let ticket_id = "UUID_OF_TICKET";
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "content": "Test message for team"
    });

    let res = client.post(&format!("http://127.0.0.1:3000/collab/{}", ticket_id))
        .bearer_auth(token)
        .json(&payload)
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
}
