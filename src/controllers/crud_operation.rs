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
pub async fn read_an_item(client: &tokio_postgres::Client, id: u32) -> Result<Item, String> {
    println!("Item id: {}", id);

    // Handle the database read operation
    let res = match db_connection::read_data(client, id as i32).await {
        Ok(item) => item,  // If successful, use the item
        Err(e) => { 
            eprintln!("query failed: {}", e); 
            return Err("Query Failed".to_string());  // Return error with a message
        },
    };

    // Return the item wrapped in Ok
    // if let Some(item) = res {
    //     Ok(item);
    // } else {
    //     return "Query failed";
    // }
    if let Some(item) = res {
        return Ok(item);  // Return Ok(item) if the item is found
    } else {
        return Err("Query failed".to_string());  // Return Err with an error message if item is None
    }
}

// Make it generic that any value can be taken care here.
pub async fn update_item(client: &tokio_postgres::Client, item: &mut Item) -> Result<Item, String> {
    let res = match db_connection::update_data(client, item).await {
        Ok(item) => item,  // If successful, use the item
        Err(e) => { 
            eprintln!("query failed: {}", e); 
            return Err("Query Failed".to_string());  // Return error with a message
        },
    };

    if let Some(item) = res {
        return Ok(item);  // Return Ok(item) if the item is found
    } else {
        return Err("Query failed".to_string());  // Return Err with an error message if item is None
    }
}
