mod utils;
use utils::helper;
use utils::lib::{Item, Item_Id};

mod controllers;
use controllers::crud_operation;

mod database;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use database::db_connection;
use env_logger;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

#[derive(Deserialize)]
struct Person {
    name: String,
}

#[derive(Serialize)]
struct Greeting {
    message: String,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn create_record(db: web::Data<Arc<Mutex<Client>>>, item: web::Json<Item>) -> impl Responder {
    let message = format!("Create Received Id, {} and Name {}!", item.id, item.name);
    let client = db.lock().await;

    let mut item = item;
    crud_operation::create_an_item(&client, &mut item).await;
    HttpResponse::Ok().json(Greeting { message })
}

async fn read_record(
    db: web::Data<Arc<Mutex<Client>>>,
    item_id: web::Json<Item_Id>,
) -> impl Responder {
    let client = db.lock().await;
    let res = crud_operation::read_an_item(&client, item_id.id).await;
    match res {
        Ok(item) => {
            let message = format!("Received Id, {} & Name, {}!", item.id, item.name);
            HttpResponse::Ok().json(Greeting { message })
        }
        Err(e) => {
            let error_message = format!("Failed to retrieve item: {}", e);
            HttpResponse::InternalServerError().json(Greeting { message: error_message })
        }
    }
}

async fn update_record(db: web::Data<Arc<Mutex<Client>>>, item: web::Json<Item>) -> impl Responder {
    let client = db.lock().await;
    let mut item = item;
    let res = crud_operation::update_item(&client, &mut item).await;
    match res {
        Ok(resItem) => {
            let message = format!("Update Received Id, {} and Name {}!", resItem.id, resItem.name);
            HttpResponse::Ok().json(Greeting { message })
        },
        Err(e) => {
            let message = format!("Failed to update item: {}!", e);
            HttpResponse::InternalServerError().json(Greeting { message })
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug"); // Set log level to debug
    }
    env_logger::init();

    let db_client = db_connection::connect_database()
        .await
        .expect("Failed to connect to the database");

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone())) // Share the pool with handlers
            .route("/", web::get().to(greet)) // Handle GET requests at "/"
            .route("/create", web::post().to(create_record)) // Handle POST requests at "/create"
            .route("/read", web::post().to(read_record)) // Handle POST requests at "/read"
            .route("/update", web::post().to(update_record)) // Handle POST requests at "/update"
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost:8080
    .run()
    .await
}
