use axum::{response::Json, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Json<JsonAPI> {
    let client = reqwest::Client::new();
    let res: JsonAPI = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await
        .expect("unable to call jsonplaceholder")
        .json()
        .await
        .expect("unable to deserialize json");

    Json(res)
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct JsonAPI {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}
