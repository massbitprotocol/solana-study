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
use self::models::SolanaBlock;
use self::models::SolanaAddress;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_block_record(conn: &PgConnection, block_number: u128, timestamp: u128, transaction_number: u128, sol_transfer: u128, fee: u128) -> SolanaBlock {
    use schema::solana_block;

    /// Todo: use all var in u128!
    let new_record = SolanaBlock {
        block_number: block_number as i64,
        timestamp: timestamp as i64,
        transaction_number: transaction_number as i64,
        sol_transfer: sol_transfer as i64,
        fee: fee as i64,
    };

    diesel::insert_into(solana_block::table)
        .values(&new_record)
        .get_result(conn)
        .expect("Error saving new record")
}

pub fn create_address_record<'a>(conn: &PgConnection, block_number: u128, timestamp: u128, address: &'a str, is_new_create: bool, balance: u128) -> usize {
    use schema::solana_address;

    /// Todo: use all var in u128!
    let new_record = SolanaAddress {
        block_number: block_number as i64,
        timestamp: timestamp as i64,
        address: address,
        is_new_create: is_new_create,
        balance: balance as i64,
    };

    diesel::insert_into(solana_address::table)
        .values(&new_record)
        .execute(conn)
        .expect("Error saving new record")
}