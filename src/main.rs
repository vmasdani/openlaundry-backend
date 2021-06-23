use actix_web::HttpResponse;
use actix_web::{rt::System, web, App, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};
type DbPool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod model;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[derive(Serialize, Deserialize, Debug)]
struct TableJsonPostBody {
    email: Option<String>,
    customers: Option<String>,

    #[serde(rename = "laundryDocuments")]
    laundy_documents: Option<String>,

    #[serde(rename = "laundryRecords")]
    laundy_records: Option<String>,
}

async fn backup_data(data: web::Json<TableJsonPostBody>) -> impl Responder {
    println!("Backup data:");
    println!("{:?}", data);

    match serde_json::to_string(&data.into_inner()) {
        Ok(data_json) => Ok(HttpResponse::Ok().json(data_json)),
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Running on port 8000");
    let manager = ConnectionManager::<SqliteConnection>::new("./openlaundry-backend.sqlite3");
    let pool = diesel::r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");

    let local = tokio::task::LocalSet::new();
    let sys = System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(hello_world))
            .route("/backup", web::post().to(backup_data))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;
    sys.await?;

    Ok(server_res)
}
