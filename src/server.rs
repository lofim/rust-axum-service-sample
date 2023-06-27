use crate::{
    config::Config,
    handlers::{
        create_todo_handler, delete_todo_handler, get_todo_handler, healthz_handler,
        list_todos_handler, readyz_handler, update_todo_handler,
    },
    use_cases::{TodoInputPortArc, TodoService},
};

use crate::todo_store::sqlite::SqliteTodoStore;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

use hyper::server::conn::AddrIncoming;
use std::{error::Error, sync::Arc};

use axum::{
    routing::{delete, get, post, put, IntoMakeService},
    Extension, Router, Server,
};

use std::net::SocketAddr;

async fn init_sql_client() -> Result<SqlitePool, Box<dyn Error>> {
    let connect_options = SqliteConnectOptions::from_str("sqlite://db/data.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn init_http_server(
    config: &Config,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Box<dyn Error>> {
    // init action layer and it's dependencies
    let todo_store = SqliteTodoStore::new(init_sql_client().await?);

    let todo_use_case = TodoService::new(Arc::new(todo_store));
    let shared_todo_use_case = Arc::new(todo_use_case) as TodoInputPortArc;

    let router = Router::new()
        .route("/healthz", get(healthz_handler))
        .route("/readyz", get(readyz_handler))
        .route("/api/v1/todos", get(list_todos_handler))
        .route("/api/v1/todos/:id", get(get_todo_handler))
        .route("/api/v1/todos", post(create_todo_handler))
        .route("/api/v1/todos/:id", put(update_todo_handler))
        .route("/api/v1/todos/:id", delete(delete_todo_handler))
        .layer(Extension(shared_todo_use_case));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    tracing::info!("listening on {}", addr);

    let server = axum::Server::bind(&addr);

    Ok(server.serve(router.into_make_service()))
}
