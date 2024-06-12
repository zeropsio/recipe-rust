use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode, error::ResponseError};
use dotenv::dotenv;
use std::{env, fmt};
use tokio_postgres::NoTls;
use uuid::Uuid;
use serde_json::json;
use log::{info, warn, error, debug, trace};
use env_logger;


#[derive(Debug)] // Automatically derive Debug trait for AppError
struct AppError(tokio_postgres::Error);

// Implement std::fmt::Display for AppError to provide human-readable error messages
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error: {}", self.0)
    }
}

// Implement ResponseError for AppError to convert it into an Actix-web error
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

// Convert tokio_postgres::Error into AppError
impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> AppError {
        AppError(err)
    }
}

async fn add_entry() -> Result<HttpResponse, AppError> {
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        db_host, db_port, db_user, db_pass, db_name
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.map_err(AppError::from)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(
        "CREATE TABLE IF NOT EXISTS entries (id SERIAL PRIMARY KEY, data TEXT NOT NULL);",
        &[],
    ).await.map_err(AppError::from)?;

    let data = Uuid::new_v4().to_string();
    client.execute(
        "INSERT INTO entries (data) VALUES ($1);",
        &[&data],
    ).await.map_err(AppError::from)?;

    let row = client.query_one("SELECT COUNT(*) FROM entries;", &[]).await.map_err(AppError::from)?;
    let count: i64 = row.get(0);

    // Log messages at various levels
    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
    debug!("This is a debug message.");
    trace!("This is a trace message.");

    Ok(HttpResponse::Ok().json(json!({
        "message": "This is a simple, basic Rust application running on Zerops.io, each request adds an entry to the PostgreSQL database and returns a count. See the source repository (https://github.com/zeropsio/recipe-rust) for more information.",
        "newEntry": data,
        "count": count
    })))
}

async fn status() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "UP",
    })))
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(add_entry)))
            .service(web::resource("/status").route(web::get().to(status)))
            .default_service(web::route().to(not_found)) // Catch-all for unmatched routes
    })
    .bind("[::]:8080")?
    .run()
    .await
}
