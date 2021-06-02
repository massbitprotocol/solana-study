use super::schema::time_transaction;
#[derive(Queryable,Insertable)]
#[table_name="time_transaction"]
pub struct TimeTransaction {
    pub block_number: i64,
    pub timestamp: i64,
    pub transaction_number: i64,
}

