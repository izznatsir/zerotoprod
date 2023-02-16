use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    let origin = spawn_server();
    let client = Client::new();

    let response = client
        .get(format!("{origin}/health-check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_route_returns_a_200_for_valid_form_data() {
    let origin = spawn_server();
    let client = Client::new();

    let response = client
        .post(format!("{origin}/subscribe"))
        .header("content-type", "application/x-www-form-urlencoded")
        .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("Failed to execute reqest.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_route_returns_a_400_for_when_data_is_missing() {
    let origin = spawn_server();
    let client = Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (form_data, error_message) in test_cases {
        let response = client
            .post(format!("{origin}/subscribe"))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(form_data)
            .send()
            .await
            .expect("Failed to execute reqest.");

        assert_eq!(400, response.status().as_u16(), "The `subscribe` API route does not return with 400 status code when the payload is {error_message}.");
    }
}

fn spawn_server() -> String {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind socket address");
    let port = listener.local_addr().unwrap().port();

    let server = zerotoprod::run(listener).expect("Failed to assign listener.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
