#[tokio::test]
async fn test_add_note() {
    let token = "Bearer YOUR_TOKEN_HERE";
    let ticket_id = "UUID_OF_TICKET";
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "content": "Test note for ticket"
    });

    let res = client.post(&format!("http://127.0.0.1:3000/notes/{}", ticket_id))
        .bearer_auth(token)
        .json(&payload)
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
}
