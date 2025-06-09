// mod utils;
use crate::database::db_connection;
use crate::utils::lib::Item;

// We are using crate here as it help us to go to root
// We are not using mod here because crud_operation is child of controllers
pub async fn create_an_item(client: &tokio_postgres::Client, item: &mut Item) {
    match db_connection::insert_data(client, item).await {
        Ok(()) => {
            println!("Query Entered Success!");
        }
        Err(e) => eprintln!("query failed: {}", e),
    }
}

// Make it as whole struct could be printed.
pub async fn read_an_item(client: &tokio_postgres::Client, id: u32) {
    println!("Item id: {}", id);
    match db_connection::read_data(client, id as i32).await {
        Ok(()) => {
            println!("Query Entered Success!");
        }
        Err(e) => eprintln!("query failed: {}", e),
    }
}

// Make it generic that any value can be taken care here.
pub async fn update_item(client: &tokio_postgres::Client, item: &mut Item) {
    match db_connection::update_data(client, item).await {
        Ok(()) => {
            println!("Query Updated Success!");
        }
        Err(e) => eprintln!("query failed: {}", e),
    }
}
