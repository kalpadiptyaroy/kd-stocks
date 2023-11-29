use std::env;
use dotenvy::dotenv;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    return MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}