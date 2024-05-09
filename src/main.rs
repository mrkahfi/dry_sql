mod types;

use clap::Parser;
use console::style;
use std::fs;
use std::fs::read_dir;
use tokio_postgres::{Config, Error, NoTls};
use types::{Args, PathParam};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let host = &args.host;
    let database = &args.database;
    let user = &args.user;
    let password = &args.password;
    let port = &args.port;
    let file = &args.file;
    let dir = &args.dir;

    let path = match check_file_or_dir(file, dir) {
        Ok(path) => path,
        Err(_) => PathParam::None,
    };

    let mut config = Config::new();
    config.host(host);
    config.user(user);
    config.password(password);
    config.dbname(database);
    config.port(*port);

    let (mut client, connection) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("{} Connection error: {}", style("✕").red(), e);
        }
    });

    match path {
        PathParam::File(file) => {
            println!("path: {}", file);

            let content = fs::read_to_string(file).expect("Should have been able to read the file");
            let prepared_statement = format!("BEGIN; {} ROLLBACK;", content);
            let transaction = client.transaction().await?;
            let result = transaction.batch_execute(&prepared_statement).await;

            match result {
                Ok(_) => println!("{} Scripts look good!", style("√").green()),
                Err(err) => println!(
                    "{} Scripts contain error(s): {}",
                    style("✕").red(),
                    err.to_string()
                ),
            }
        }
        PathParam::Dir(dir) => {
            // Read directory entries
            let entries = read_dir(dir).unwrap();

            for entry in entries {
                let entry = entry.unwrap();

                if entry.file_type().unwrap().is_file() {
                    let file = entry.path();
                    let content =
                        fs::read_to_string(file).expect("Should have been able to read the file");
                    let prepared_statement = format!("BEGIN; {} ROLLBACK;", content);
                    let transaction = client.transaction().await?;
                    let result = transaction.batch_execute(&prepared_statement).await;

                    match result {
                        Ok(_) => println!("{} Scripts look good!", style("√").green()),
                        Err(err) => println!(
                            "{} Scripts contain error(s): {}",
                            style("✕").red(),
                            err.to_string()
                        ),
                    }
                }
            }
        }
        PathParam::None => (),
    }

    Ok(())
}

fn check_file_or_dir<'a>(file: &'a String, dir: &'a String) -> Result<PathParam<'a>, String> {
    if file.is_empty() && dir.is_empty() {
        return Err(String::from("Both --file or --dir are unset"));
    }

    if !file.is_empty() && !dir.is_empty() {
        return Err(String::from("Provide only --file or --dir"));
    }

    if !file.is_empty() {
        return Ok(PathParam::File(&file));
    } else {
        return Ok(PathParam::Dir(&dir));
    }
}
