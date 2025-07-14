#[tokio::test]
async fn test_create_ticket() {
    let token = "Bearer YOUR_TOKEN_HERE";
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "title": "Test Ticket",
        "description": "This is a test",
        "status": "open",
        "priority": "high"
    });

    let res = client.post("http://127.0.0.1:3000/tickets")
        .bearer_auth(token)
        .json(&payload)
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
}
