mod routes;
mod utils;

use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    axum::Server::bind(
        &format!(
            "0.0.0.0:{port}",
            port = option_env!("PORT").unwrap_or("8080")
        )
        .parse()
        .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
