use sea_orm_migration::prelude::*;
use dotenv::dotenv;

#[async_std::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    match std::env::var("DATABASE_URL") {
        Ok(val) => {
            println!("Using DATABASE_URL: {}", val);
        }
        Err(err) => {
            println!("DATABASE_URL not set: {}", err);
        }
    };

    cli::run_cli(migration::Migrator).await;
}
