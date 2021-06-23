use actix_web::HttpResponse;
use actix_web::{rt::System, web, App, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};
type DbPool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod model;

use model::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::io::Read;

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

    let data = data.into_inner();

    // For testing
    // curl localhost:8000/backup -H 'content-type:application/json' -d '{"email":"valianmasdani@gmail.com", "customers": "H4sIAAAAAAAA/6XQvW4DMQgH8P2eovJ8SDYQc86W54g6+A5QO+RDuWSq+u6xWqXq4CztguAPw0/sh49we9ewDYSaVDwCMRuwk0NVIphiEl82yu4YxpdwrAdr5+vpYPDVt+z8djr+hN9DS6vqxdb1kT/GtlkuVq+mu2vYpozMWTZTmTCN4XbW7uZz/A0VzeQJF9C5RmC1AjUbt6JzLOJzrtKzYh+Lz7X4f65nxDm2bwp6+6siFBGFiB5dilHh3LNS30rPrfRH6zC83gFx+AWdBwIAAA=="}'

    match &data.customers {
        Some(customers_str) => {
            let customers = match base64::decode(customers_str) {
                Ok(customers_bin) => match libflate::gzip::Decoder::new(&customers_bin[..]) {
                    Ok(mut res) => {
                        let mut res_str = Vec::new();
                        res.read_to_end(&mut res_str);

                        let json_str = String::from_utf8_lossy(&res_str);

                        let customers_data = serde_json::from_str::<Vec<CustomerJson>>(&json_str);

                        println!("Decoded JSON: {:?}", json_str);
                        println!("Rust struct: {:?}", customers_data);
                    }
                    Err(e) => {
                        println!("Customer gzip inflat error {:?}", e);
                    }
                },
                Err(e) => {
                    println!("Customer base64 error {:?}", e);
                }
            };
        }
        None => {
            println!("Customer empty");
        }
    }

    match serde_json::to_string(&data) {
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
