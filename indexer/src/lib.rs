pub mod schema;
pub mod models;
pub mod decode_solana;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::SolanaBlockAggregate;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_record(conn: &PgConnection, block_number: u128, timestamp: u128, transaction_number: u128, sol_transfer: u128, fee: u128) -> SolanaBlockAggregate {
    use schema::solana_block_aggregate;

    /// Todo: use all var in u128!
    let new_record = SolanaBlockAggregate {
        block_number: block_number as i64,
        timestamp: timestamp as i64,
        transaction_number: transaction_number as i64,
        sol_transfer: sol_transfer as i64,
        fee: fee as i64,
    };

    diesel::insert_into(solana_block_aggregate::table)
        .values(&new_record)
        .get_result(conn)
        .expect("Error saving new record")
}