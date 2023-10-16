use std::net::SocketAddr;

use axum::{Router, routing::{get, post}, Json, http::StatusCode, extract::State};

use serde::{Deserialize, Serialize};

use sqlx::{postgres::PgPoolOptions, pool, Postgres, Pool, PgPool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();

    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://dev:abcde@localhost/test").await?;
    
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .with_state(pool);


    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>) -> Result<String, (StatusCode, String)>{
    sqlx::query("INSERT INTO users(username) VALUES ($1)").bind(payload.username).execute(&pool).await.map_err(internal_error)?;

    Ok(String::from("User created successfully"))
}

#[derive(Deserialize)]
struct CreateUser{
    username: String,
}

#[derive(Serialize)]
struct User{
    id: i32,
    username: String
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}