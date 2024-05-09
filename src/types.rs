use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = String::from("localhost"))]
    pub host: String,
    #[arg(long, default_value_t = String::from("postgres"))]
    pub database: String,
    #[arg(long, default_value_t = String::from("postgres"))]
    pub user: String,
    #[arg(long, default_value_t = String::from("postgres"))]
    pub password: String,
    #[arg(long, default_value_t = 54322)]
    pub port: u16,
    #[arg(long, default_value_t = String::from(""))]
    pub file: String,
    #[arg(long, default_value_t = String::from(""))]
    pub dir: String,
}

pub enum PathParam<'a> {
    File(&'a str),
    Dir(&'a str),
    None,
}
