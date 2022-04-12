use migration::Migrator;
use sea_schema::migration::*;

#[cfg(debug_assertions)]
use dotenv::dotenv;

#[async_std::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    // let fallback = "postgres://postgres:mao0311@localhost/mydb";
    let fallback = "DATABASE_URL";

    match std::env::var("DATABASE_URL") {
        Ok(val) => {
            println!("Using DATABASE_URL: {}", val);
        }
        Err(_) => {
            std::env::set_var("DATABASE_URL", fallback);
            println!("Set DATABASE_URL: {}", fallback);
        }
    };

    cli::run_cli(Migrator).await;
}