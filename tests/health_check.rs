use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    return format!("127.0.0.1:{}", port);
}

fn addr_string_to_endpoint(addr: &String, endpt: String) -> String {
    return format!("http://{}/{}", addr, endpt);
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app().await;
    let endpoint = addr_string_to_endpoint(&addr, "healthcheck".to_string());
    let client = reqwest::Client::new();

    let response = client
        .get(&endpoint)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_200_on_success() {
    let addr = spawn_app().await;
    let endpoint = addr_string_to_endpoint(&addr, "subscriptions".to_string());
    let client = reqwest::Client::new();

    let body = "name=le20%guin&mail=le_guin40%gmail.com";
    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_400_on_error() {
    let addr = spawn_app().await;
    let endpoint = addr_string_to_endpoint(&addr, "subscriptions".to_string());
    let client = reqwest::Client::new();

    let failed_cases = vec![
        ("name=le20%guin", "missing the email"),
        ("mail=le_guin40%gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, failed_response) in failed_cases {
        let response = client
            .post(&endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail when it should fail w/ {}",
            failed_response
        );
    }
}
