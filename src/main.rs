use actix_web::http::ContentEncoding;
use actix_web::{middleware, HttpResponse};
use actix_web::{rt::System, web, App, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use libflate::gzip::Encoder;
use model::BaseModel;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
type DbPool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod model;
pub mod schema;

// use model::*;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::run_migrations;

extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::io::Read;
use std::{env, io};

use crate::model::{BackupRecord, CustomerJson};

embed_migrations!();

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TableJsonPostBody {
    email: Option<String>,
    customers: Option<String>,

    #[serde(rename = "laundryDocuments")]
    laundy_documents: Option<String>,

    #[serde(rename = "laundryRecords")]
    laundy_records: Option<String>,
}

fn decode_and_backup<T: DeserializeOwned + std::fmt::Debug + BaseModel + Serialize + Clone>(
    backup_record_str: String,
    db_str: String,
) -> Option<String> {
    println!("In DB: {:?}", db_str);

    // First, decode DB
    let mut db_vec = match base64::decode(db_str) {
        Ok(db_bin) => match libflate::gzip::Decoder::new(&db_bin[..]) {
            Ok(mut db_res) => {
                let mut res_str = Vec::new();
                db_res.read_to_end(&mut res_str);

                let json_str = String::from_utf8_lossy(&res_str);

                println!("Decoded DB: {:?}", json_str.len());

                Some(serde_json::from_str::<Vec<T>>(&json_str).unwrap_or_default())
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        },
        Err(e) => {
            println!("{:?}", e);
            None
        }
    };

    match base64::decode(backup_record_str) {
        Ok(customers_bin) => match libflate::gzip::Decoder::new(&customers_bin[..]) {
            Ok(mut res) => {
                let mut res_str = Vec::new();
                res.read_to_end(&mut res_str);

                let json_str = String::from_utf8_lossy(&res_str);

                match serde_json::from_str::<Vec<T>>(&json_str) {
                    Ok(items_res) => {
                        println!("Decoded JSON: {:?}", json_str.len());
                        println!("Rust struct: {:?}", items_res);

                        items_res.into_iter().for_each(|item| {
                            println!("Item: {:?}", item.uid());

                            match &mut db_vec {
                                Some(db_vec) => {
                                    // Find occurrences

                                    println!("\nFinding occurrences...\n");

                                    let db_vec_clone = db_vec.clone();

                                    if db_vec_clone
                                        .iter()
                                        .filter(|item_x| {
                                            item_x
                                                .uid()
                                                .unwrap_or_default()
                                                .eq(&item.uid().unwrap_or_default())
                                        })
                                        .collect::<Vec<_>>()
                                        .len()
                                        > 0
                                    {
                                        println!("Found UID for {:?}", &item.uid());
                                    } else {
                                        println!("Pushing {:?}", &item.uid());
                                        db_vec.push(item.clone())
                                    }
                                }
                                None => {
                                    println!("No occurrences found for {:?}", &item.uid());
                                }
                            }
                        });
                    }
                    Err(e) => {
                        println!("Decoding generic JSON STR error {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Customer gzip inflat error {:?}", e);
            }
        },
        Err(e) => {
            println!("Customer base64 error {:?}", e);
        }
    };

    // Gzip then convert to base64
    match serde_json::to_string(&db_vec.unwrap_or_default()) {
        Ok(json_str) => {
            let mut encoder = Encoder::new(Vec::new()).unwrap();
            io::copy(&mut json_str.as_bytes(), &mut encoder).unwrap();

            Some(base64::encode(
                encoder.finish().into_result().unwrap_or_default(),
            ))
        }
        Err(e) => {
            println!("{:?}", e);

            None
        }
    }
}

async fn backup_data(
    data: web::Json<TableJsonPostBody>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    println!("Backup data:");
    println!("{:?}", data);

    let data = data.into_inner();

    // For testing
    // 3 record cust: curl localhost:8000/backup -H 'content-type:application/json' -d '{"email":"valianmasdani@gmail.com", "customers": "H4sIAAAAAAAA/6XQvW4DMQgH8P2eovJ8SDYQc86W54g6+A5QO+RDuWSq+u6xWqXq4CztguAPw0/sh49we9ewDYSaVDwCMRuwk0NVIphiEl82yu4YxpdwrAdr5+vpYPDVt+z8djr+hN9DS6vqxdb1kT/GtlkuVq+mu2vYpozMWTZTmTCN4XbW7uZz/A0VzeQJF9C5RmC1AjUbt6JzLOJzrtKzYh+Lz7X4f65nxDm2bwp6+6siFBGFiB5dilHh3LNS30rPrfRH6zC83gFx+AWdBwIAAA=="}'
    // 5 record cust: curl localhost:8000/backup -H 'content-type:application/json' -d '{"email":"valianmasdani@gmail.com", "customers": "H4sIAAAAAAAA/6XSvU4DMQwH8P2eAmWupcRx4ks3ngMxJGdbMPRDvXZCvDsRqIght9Alsv/O8JPll+nD3d7F7V1ECcLmIRIpkEWDKjHC7APbkoTM0O2e3LEetH9fTweF77pn57fT8Tf8aXpaRS66rvf83vbJctF6VXm+un3ISJQ5zWXGsHO3swwnn7u/UJYcLeAC0qoHEi1Qs1J/pPnC1nLlkRXHWNzW4uNcy4jN920yWt+rIBRmAY/mjYvGQnlkjWNr3LbGh63sG5mmDJaYgBJGKMUzBJsjBdVCrY6sNLbStpUeti7SQkEjUKYKxFm7dUbwmfsJaJpzGN5rGlvTtjX90zpN0+sXB0Cn+GADAAA="}'

    // Search
    let data_clone = data.clone();
    let email_clone = data_clone.email.clone();

    let mut backup_record = web::block(move || match pool.get() {
        Ok(conn) => {
            use crate::schema::backup_records::dsl::*;

            let found_backup_record = match backup_records
                .filter(email.eq(&email_clone))
                .first::<BackupRecord>(&conn)
            {
                Ok(backup_record) => Some(backup_record),
                Err(e) => {
                    println!("Backup record not found {:?}", e);

                    let mut encoder = Encoder::new(Vec::new()).unwrap();
                    io::copy(&mut &b"[]"[..], &mut encoder).unwrap();
                    let empty_arr = base64::encode(encoder.finish().into_result().unwrap());

                    println!("Empty arr {:?}", empty_arr);

                    let email_clone_param = &email_clone.clone().unwrap_or_default();

                    let new_backup_record = BackupRecord {
                        id: None,
                        created_at: None,
                        updated_at: None,
                        customers: Some(empty_arr.clone()),
                        laundry_records: Some(empty_arr.clone()),
                        laundry_documents: Some(empty_arr.clone()),
                        email: Some(email_clone_param.to_string()),
                    };

                    println!("New backup record: {:?}", new_backup_record);

                    diesel::replace_into(backup_records)
                        .values(&new_backup_record)
                        .execute(&conn);

                    match backup_records
                        .filter(
                            id.eq(diesel::select(last_insert_rowid)
                                .get_result::<i32>(&conn)
                                .unwrap_or_default()),
                        )
                        .first::<BackupRecord>(&conn)
                    {
                        Ok(backup_record) => Some(backup_record),
                        Err(e) => None,
                    }
                }
            };

            println!(
                "Found backup record for {:?}: {:?}",
                &email_clone, found_backup_record
            );

            match found_backup_record {
                Some(mut backup_record) => {
                    println!("Backup record OK");

                    // Backup customers
                    match data_clone.customers {
                        Some(customers_str) => {
                            let customers_res = decode_and_backup::<CustomerJson>(
                                customers_str.to_string(),
                                backup_record
                                    .customers
                                    .clone()
                                    .unwrap_or_default()
                                    .to_string(),
                            );

                            match customers_res {
                                Some(customers_json) => {
                                    println!("Customers res str: {:?}", customers_json.len());

                                    println!("{:?}", customers_json);

                                    backup_record.customers = Some(customers_json);
                                }
                                None => {
                                    println!("No customers to be put on record.");
                                }
                            }
                        }
                        None => {
                            println!("Customer empty");
                        }
                    }

                    // TODO: backup laundry records

                    // TODO: backup laundry documents

                    use schema::backup_records::dsl::*;

                    diesel::replace_into(backup_records)
                        .values(&backup_record)
                        .execute(&conn);

                    Ok(Some(backup_record))
                }

                None => {
                    println!("No backup record. Error");
                    Ok(None)
                }
            }
        }
        Err(e) => Err(e),
    })
    .await;

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

    // This will run the necessary migrations.
    match pool.get() {
        Ok(conn) => {
            embedded_migrations::run(&conn);
        }
        Err(e) => {
            println!("Error getting pool {:?}", e);
        }
    }

    let local = tokio::task::LocalSet::new();
    let sys = System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
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
