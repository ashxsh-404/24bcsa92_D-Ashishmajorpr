#[tokio::test]
async fn test_signup_and_login() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // optional delay

    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "name": "Test Agent",
        "email": "testagent@example.com",
        "password": "test1234"
    });

    let res = client
        .post("http://127.0.0.1:3000/auth/signup") // ✅ use 127.0.0.1
        .json(&payload)
        .send()
        .await
        .expect("Failed to send signup request");

    println!("Signup response = {:?}", res.status());
    assert_eq!(res.status(), 201);

    let login_payload = serde_json::json!({
        "email": "testagent@example.com",
        "password": "test1234"
    });

    let res = client
        .post("http://127.0.0.1:3000/auth/login")
        .json(&login_payload)
        .send()
        .await
        .expect("Failed to send login request");

    println!("Login response = {:?}", res.status());
    assert_eq!(res.status(), 200);
}
