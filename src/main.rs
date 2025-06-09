mod utils;
use utils::helper;
use utils::lib::Item;

mod controllers;
use controllers::crud_operation;

mod database;
use database::db_connection;

#[tokio::main]
async fn main() {
    let db_client = match db_connection::connect_database().await {
        Ok(client) => {
            println!("Connected successfully!");
            Some(client)
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            None
        }
    };

    if let Some(client) = db_client {
        println!("Create item!");
        let (id, name) = helper::input_values();
        let mut item = Item { id: id, name: name };
        crud_operation::create_an_item(&client, &mut item).await;

        println!("Read item!");
        let id = helper::input_id();
        crud_operation::read_an_item(&client, id).await;

        println!("Update item!");
        let (id, name) = helper::input_values();
        let mut item = Item { id: id, name: name };
        crud_operation::update_item(&client, &mut item).await;
    };
}
