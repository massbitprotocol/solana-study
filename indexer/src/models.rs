use super::schema::solana_block_aggregate;
#[derive(Queryable,Insertable)]
#[table_name="solana_block_aggregate"]
pub struct SolanaBlockAggregate {
    pub block_number: i64,
    pub timestamp: i64,
    pub transaction_number: i64,
    pub sol_transfer: i64,
    pub fee: i64
}

