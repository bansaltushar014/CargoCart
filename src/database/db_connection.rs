use crate::utils::lib::Item;
use chrono::NaiveDateTime;
use dotenv::dotenv;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::env;
use std::fs::File;
use std::io::Read;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::{Config, Error, NoTls};

pub async fn connect_database() -> Result<tokio_postgres::Client, Box<dyn std::error::Error>> {
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

    Ok(client)
}

// async move
// tokio::spawn -> it does not let the connection drop, if drop then immediately informs

pub async fn insert_data(client: &tokio_postgres::Client, item: &mut Item) -> Result<(), Error> {
    // Define the values to insert
    let id: i32 = item.id as i32; // Example ID
    let name: &str = &item.name; // Example Name
    // Insert statement
    let query = "INSERT INTO users (id, name) VALUES ($1, $2)";

    // Execute the query with the provided values
    client
        .execute(query, &[&id, &name])
        .await
        .expect("Failed to insert data");

    println!("Data inserted: id = {}, name = {}", id, name);

    Ok(())
}
