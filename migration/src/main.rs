use sea_orm_migration::prelude::*;
use dotenv::dotenv;

#[async_std::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();
    println!("Current dir: {:?}", std::env::current_dir().unwrap());


    match std::env::var("DATABASE_URL") {
        Ok(val) => {
            println!("Using DATABASE_URL: {}", val);
        }
        Err(err) => {
            println!("DATABASE_URL not set: {}", err);
            std::process::exit(1);
        }
    };

    cli::run_cli(migration::Migrator).await;
}
