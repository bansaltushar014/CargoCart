mod utils;
use utils::helper::input_values;
use utils::lib::Item;

mod controllers;
use controllers::crud_operation;

mod database;
use database::db_connection;

#[tokio::main]
async fn main() {
    // let client = db_connection::connect_database().await?;
    // println!("{:?}", client);
    // let mut db_client: Option<tokio_postgres::Client> = None;
    let mut db_client = match db_connection::connect_database().await {
        Ok(client) => {
            println!("Connected successfully!");
            Some(client)
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            None
        }
    };

    let (id, name) = input_values();
    let mut item = Item { id: id, name: name };

    if let Some(client) = db_client {
        println!("Client-  {:?}", client);
        // crud_operation::create_an_item(&client, &mut item).await;
        match db_connection::insert_data(&client, &mut item).await {
            Ok(()) => {
                println!("Query Entered Success!");
            }
            Err(e) => eprintln!("query failed: {}", e),
        }
    };

    // crud_operation::read_an_item(&mut item);
    // crud_operation::update_item(&mut item, 12);
    // crud_operation::read_an_item(&mut item);
}
