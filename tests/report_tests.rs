#[tokio::test]
async fn test_ticket_report() {
    let token = "Bearer YOUR_TOKEN_HERE";
    let client = reqwest::Client::new();

    let res = client.get("http://127.0.0.1:3000/reports/overview")
        .bearer_auth(token)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
}
