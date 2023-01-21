use zerotoprod::app;

#[tokio::main]
async fn main() {
    let app = app();

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
