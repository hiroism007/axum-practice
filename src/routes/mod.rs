mod always_error;
mod get_json;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod returns_201;
mod set_middleware_custom_header;
mod validate_with_serde;
use always_error::always_error;
use axum::{extract::FromRef, middleware, routing::get, routing::post, Router};
use get_json::get_json;
use hello_world::hello_world;
use hyper::Method;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use returns_201::returns_201;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use validate_with_serde::validate_with_serde;

use self::mirror_custom_header::custom_header;

#[derive(Clone, FromRef)]
pub struct SharedData {
    uid: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        uid: "fuga".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/user_agent", get(mirror_user_agent))
        .route("/custom_header", get(custom_header))
        .route("/middleware_message", get(middleware_message))
        .with_state(shared_data)
        .layer(cors)
        .route("/always_errors", get(always_error))
        .route("/returns_201", get(returns_201))
        .route("/validate_with_serde", post(validate_with_serde))
        .route("/get_json", get(get_json))
    // 最後にもってくるとそれ以前にある route に効果が及ぶ
    // route の優先度は登録順
}
