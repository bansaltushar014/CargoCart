use crate::utils::lib::Item;
use dotenv::dotenv;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::env;
use std::fs::File;
use std::io::Read;
use tokio_postgres::{Config, Error};

pub async fn connect_database() -> Result<Arc<Mutex<tokio_postgres::Client>>, Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);

    let mut file = File::open("./ca.pem")?;
    let mut cert_data = Vec::new();
    file.read_to_end(&mut cert_data)?;

    let cert = Certificate::from_pem(&cert_data)?;
    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;

    let tls = MakeTlsConnector::new(connector);
    let config: Config = database_url.parse()?;

    let (client, connection) = config.connect(tls).await?;

    // Spawn connection to run in background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(client)))
}

// async move
// tokio::spawn -> it does not let the connection drop, if drop then immediately informs
async fn check_existance(client: &tokio_postgres::Client, id: i32) -> Result<bool, Error> {
    let query = "SELECT * FROM users WHERE id=$1;";

    let rows = client.query(query, &[&id]).await?;

    if let Some(row) = rows.get(0) {
        // The first (and only) row will contain the EXISTS result, which is an integer (1 or 0)
        let exists: i32 = row.get(0); // Access the first column (0) of the row, which is an integer (1 or 0)
        let exists_bool = exists != 0; // 1 means true, 0 means false
        println!("Exists: {}", exists_bool);
        Ok(exists_bool)
    } else {
        println!("No rows found.");
        Ok(false)
    }
}

pub async fn insert_data(client: &tokio_postgres::Client, item: &mut Item) -> Result<(), Error> {
    let id: i32 = item.id as i32;
    let name: &str = &item.name;

    if check_existance(client, id).await? {
        println!("Data with id = {} already exists, no insert needed.", id);
    } else {
        let query = "INSERT INTO users (id, name) VALUES ($1, $2)";

        client
            .execute(query, &[&id, &name])
            .await
            .expect("Failed to insert data");

        println!("Data inserted: id = {}, name = {}", id, name);
    }

    Ok(())
}

pub async fn update_data(client: &tokio_postgres::Client, item: &mut Item) -> Result<Option<Item>, Error> {
    let id: i32 = item.id as i32;
    let name: &str = &item.name;

    let mut resItem: Option<Item> = None;
    if check_existance(client, id).await? {
        let query = "UPDATE users SET name = $2 WHERE id = $1";

        client
            .execute(query, &[&id, &name])
            .await
            .expect("Failed to insert data");

        println!("Data inserted: id = {}, name = {}", id, name);
        resItem = Some(Item {id: item.id, name: item.name.clone()});
    } else {
        println!("Data with id = {} not exists, no update needed.", id);
    }

    Ok(resItem)
}

pub async fn read_data(client: &tokio_postgres::Client, id: i32) -> Result<(Option<Item>), Error> {
    println!("Data read against: id = {}", id);

    let query = "SELECT * FROM users WHERE id=$1;";

    // Execute the query with the provided values
    let rows = client
        .query(query, &[&id])
        .await
        .expect("Failed to insert data");

    let mut user_id: Option<i32> = None;
    let mut name: Option<String> = None;

    for row in rows {
        user_id = Some(row.get(0));
        name = Some(row.get(1));
    }

    let mut item: Option<Item> = None;
    match (user_id, name.clone()) {
        (Some(id), Some(name)) => {
            println!("User ID: {}, Name: {}", id, name);
            item = Some(Item {id: id as u32, name: name});
        }
        _ => {
            println!("No data found.");
        }
    }

    println!("User ID: {:?}, Name: {:?}", user_id, name);

    Ok(item)
}
