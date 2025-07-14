#[tokio::test]
async fn test_create_article() {
    let token = "Bearer YOUR_TOKEN_HERE";
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "title": "Getting Started",
        "content": "How to use the system",
        "tags": ["guide", "start"]
    });

    let res = client.post("http://127.0.0.1:3000/kb")
        .bearer_auth(token)
        .json(&payload)
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
}
