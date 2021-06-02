pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::TimeTransaction;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_record(conn: &PgConnection, block_number: u128, timestamp: u128, transaction_number: u128) -> TimeTransaction {
    use schema::time_transaction;

    let new_record = TimeTransaction {
        block_number: block_number as i64,
        timestamp: timestamp as i64,
        transaction_number: transaction_number as i64,
    };

    diesel::insert_into(time_transaction::table)
        .values(&new_record)
        .get_result(conn)
        .expect("Error saving new record")
}