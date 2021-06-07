CREATE TABLE solana_block_aggregate (
                                        block_number BIGINT PRIMARY KEY,
                                        timestamp BIGINT NOT NULL,
                                        transaction_number BIGINT NOT NULL,
                                        sol_transfer BIGINT NOT NULL,
                                        fee BIGINT NOT NULL
)