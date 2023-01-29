use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let optional_value: Option<&'static str> = option_env!("PORT");

    Json(Data {
        message: optional_value.unwrap_or("8080").to_owned(),
        count: 10,
        username: "hiroism007".to_owned(),
    })
}
