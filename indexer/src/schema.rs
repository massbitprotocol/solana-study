table! {
    solana_block_aggregate (block_number) {
        block_number -> Int8,
        timestamp -> Int8,
        transaction_number -> Int8,
        sol_transfer -> Int8,
        fee -> Int8,
    }
}
