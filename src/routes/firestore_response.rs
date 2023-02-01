use axum::response::{IntoResponse, Response};
use firestore::*;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Data {
    message: String,
    count: i32,
    username: String,
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

pub async fn firestore_response() -> Response {
    let project_id = config_env_var("PROJECT_ID").unwrap();

    // Create an instance
    let db = FirestoreDb::new(project_id).await.unwrap();

    let my_struct = Data {
        message: "hoge".to_string(),
        count: 10,
        username: "hiroism007".to_string(),
    };

    db.fluent()
        .insert()
        .into("tests")
        .document_id(&my_struct.message)
        .object(&my_struct)
        .execute()
        .await
        .unwrap_or_else(|e| panic!("Error: {}", e));

    (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}
