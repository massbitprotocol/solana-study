use super::schema::solana_block;
use super::schema::solana_address;
#[derive(Queryable,Insertable)]
#[table_name="solana_block"]
pub struct SolanaBlock {
    pub block_number: i64,
    pub timestamp: i64,
    pub transaction_number: i64,
    pub sol_transfer: i64,
    pub fee: i64
}

#[derive(Insertable)]
#[table_name="solana_address"]
pub struct SolanaAddress<'a> {
    pub block_number: i64,
    pub timestamp: i64,
    pub address:&'a str,
    pub is_new_create: bool,
    pub balance: i64
}
