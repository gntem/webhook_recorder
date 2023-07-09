use tide;
use sqlx;
use dotenv;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
struct State {
    db: sqlx::PgPool,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct WebhookRecording {
    uuid: Uuid,
    url: String,
    method: String,
    body: String,
    headers: String,
}

impl WebhookRecording {
    fn new(url: String, method: String, body: String, headers: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            url,
            method,
            body,
            headers,
        }
    }
}

async fn create_webhook_recording(
    state: &State,
    recording: WebhookRecording,
) -> tide::Result<()> {
    sqlx::query!(
        "INSERT INTO webhook_recordings (url, method, body, headers) VALUES ($1, $2, $3, $4)",
        recording.url,
        recording.method,
        recording.body,
        recording.headers,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = sqlx::PgPool::connect(&db_url).await.unwrap();
    let state = State { db: db_pool };
    let mut app = tide::with_state(state);
    app.at("/sandbox")
        .post(|mut req: tide::Request<State>| async move {
            let url = req.host().unwrap().to_string();
            let method = req.method().to_string();
            let body = req.body_string().await?;
            let headers = req
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\n");
            let recording: WebhookRecording = WebhookRecording::new(url, method, body, headers);
            create_webhook_recording(&req.state(), recording).await?;
            Ok("OK")
        });
    app.listen("127.0.0.1:8080").await.unwrap();
}
