table! {
    solana_address (id) {
        id -> Int8,
        block_number -> Int8,
        timestamp -> Int8,
        address -> Text,
        is_new_create -> Bool,
        balance -> Int8,
    }
}

table! {
    solana_block (block_number) {
        block_number -> Int8,
        timestamp -> Int8,
        transaction_number -> Int8,
        sol_transfer -> Int8,
        fee -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    solana_address,
    solana_block,
);
