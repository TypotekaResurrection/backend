use entity::sea_orm;
use sea_orm::DatabaseConnection;
use tokio_postgres::NoTls;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        // println!("{:?}", std::env::var("DATABASE_URL"));
        let connection = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap())
            .await
            .expect("Could not connect to database");

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}