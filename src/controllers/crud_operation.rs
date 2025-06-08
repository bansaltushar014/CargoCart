// mod utils;
use crate::database::db_connection;
use crate::utils::lib::Item;
use tokio_postgres::{Error, NoTls};

// We are using crate here as it help us to go to root
// We are not using mod here because crud_operation is child of controllers

pub async fn create_an_item(client: &tokio_postgres::Client, item: &mut Item) {
    // if let Some(client) = db_client {
    //     println!("{:?}", client);
    //     match db_connection::insert_data(&client, &mut item).await {
    //         Ok(()) => {
    //             println!("Query Entered Success!");
    //         },
    //         Err(e) => eprintln!("query failed: {}", e),
    //     }
    // };
    println!("Inside create Item!");
    match db_connection::insert_data(client, item).await {
        Ok(()) => {
            println!("Query Entered Success!");
        }
        Err(e) => eprintln!("query failed: {}", e),
    }
}

// Make it as whole struct could be printed.
pub fn read_an_item(item: &mut Item) {
    println!("Item id: {} , name: {}", item.id, item.name);
}

// Make it generic that any value can be taken care here.
pub fn update_item(_item: &mut Item, _id: u32) {
    _item.id = _id;
}
