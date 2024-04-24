use std::fs;
use tokio_postgres::{Error, NoTls};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("postgres"))]
    user: String,
    #[arg(long, default_value_t = String::from("postgres"))]
    password: String,
    #[arg(long, default_value_t = String::from("54322"))]
    port: String,
    #[arg(long, default_value_t = String::from(""))]
    file_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let user = &args.user;
    let password = &args.password;
    let port = &args.port;
    let file_path = &args.file_path;

    let config = format!(
        "host=localhost user={} port={} password={}",
        user, port, password
    );
    let config_str = config.as_str();

    let (mut client, connection) = tokio_postgres::connect(config_str, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("file: {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let prepared_statement = format!("BEGIN; {} ROLLBACK;", content);

    let transaction = client.transaction().await?;

    let result = transaction.batch_execute(&prepared_statement).await;

    match result {
        Ok(_) => println!("Successfully executed script"),
        Err(err) => println!("Failed executing script: {}", err.to_string()),
    }

    Ok(())
}
