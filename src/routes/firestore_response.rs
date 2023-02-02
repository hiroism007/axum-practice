use axum::response::{IntoResponse, Response};
use firestore::*;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Data {
    message: String,
    count: i32,
    username: String,
}

pub fn config_env_var(name: &str) -> Result<String, AppError> {
    std::env::var(name).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{} is not set", name),
        )
    })
}

pub async fn firestore_response() -> Response {
    // 環境変数から firebase の　project id を取得
    let project_id = match config_env_var("PROJECT_ID") {
        Ok(project_id) => project_id.to_string(),
        Err(e) => return e.into_response(),
    };

    // db を作成
    let db = FirestoreDb::new(project_id)
        .await
        .map_err(internal_error)
        .unwrap();

    // データを作成
    let my_struct = Data {
        message: "hoge".to_string(),
        count: 10,
        username: "hiroism007".to_string(),
    };

    // データを firestore に保存
    db.fluent()
        .insert()
        .into("tests")
        .document_id(&my_struct.message)
        .object(&my_struct)
        .execute()
        .await
        .unwrap_or_else(|e| panic!("Error: {}", e));

    // response を作成
    (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> AppError
where
    E: std::error::Error,
{
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
