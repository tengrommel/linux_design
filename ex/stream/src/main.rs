use dotenv;
use serde_json::json;
use sqlx::{query, PgPool, Pool};
use tide::http::StatusCode;
use tide::Request;
use tide::Response;
use tide::Server;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::new(&db_url).await.unwrap(); // 必须声明
    let mut app: Server<State> = Server::with_state(State { db_pool });
    app.at("/").get(|req: Request<State>| async move {
        let json = json!([1, 2, 3]);
        Ok(Response::new(StatusCode::Ok).body_json(&json)?)
    });
    app.listen("127.0.0.1:8000").await.unwrap();
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
