#![allow(non_snake_case)]

// use error_chain::error_chain;
use serde::Deserialize;
use serde_json::{json, Value};
//use std::env;
use reqwest::Client;
use reqwest::Response;
use std::error::Error;
//use tokio_postgres::{NoTls};
use std::process::Command;
use regex::Regex;
//use tokio_postgres::{NoTls};
use indexer::{establish_connection, create_block_record,create_address_record,};
use diesel::PgConnection;
use std::time::Duration;
use std::thread;
use futures::future::try_join_all;
use indexer::decode_solana::{decode_transaction,decode_instruction_data,get_sol_transfer_in_transaction};
use solana_sdk::{
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    program_utils::limited_deserialize,
};
//use indexer::schema::solana_address::columns::{is_new_create, address};
use itertools::izip;

type TypeTimeStamp = u128;
type TypeBlockSlot = u128;
type TypeTokenUnit = u128;
type TypeBlockHash = String;
type TypeTransactionStatusOk = String;
type TypeAccountPublicAddress = Pubkey;
type TypeInnerInstructions = serde_json::Value;
type TypeErr = Option<serde_json::Value>;
type TypePostTokenBalances = serde_json::Value;
type TypePreTokenBalances = serde_json::Value;


// Response of getBlock RPC
#[derive(Deserialize, Debug)]
pub struct BlockResponse {
    id: u64,
    jsonrpc: String,
    result: BlockResult,
}

#[derive(Deserialize, Debug)]
struct BlockResult {
    blockTime: TypeTimeStamp,
    blockhash: TypeBlockHash,
    parentSlot: TypeBlockSlot,
    previousBlockhash: TypeBlockHash,
    rewards: Vec<BlockReward>,
    transactions: Vec<BlockTransaction>,

}

#[derive(Deserialize, Debug)]
struct BlockReward {
    lamports: TypeTokenUnit,
    postBalance: TypeTokenUnit,
    pubkey: TypeBlockHash,
    rewardType: String,
}

#[derive(Deserialize, Debug)]
struct BlockTransaction {
    meta:TransactionMeta,
    transaction:[String;2],
}

#[derive(Deserialize, Debug)]
struct TransactionMeta {
    err: TypeErr,
    fee: TypeTokenUnit,
    innerInstructions: Vec<TypeInnerInstructions>,
    logMessages: Vec<String>,
    postBalances: Vec<TypeTokenUnit>,
    postTokenBalances: TypePostTokenBalances,
    preBalances: Vec<TypeTokenUnit>,
    preTokenBalances: TypePreTokenBalances,
    status: TransactionStatus,

}

#[derive(Deserialize, Debug)]
struct TransactionStatus {
    Ok: Option<TypeTransactionStatusOk>
}

#[derive(Deserialize, Debug)]
struct EpochInfoResponse {
    jsonrpc: String,
    result: EpochInfo,
    id: u64
}

#[derive(Deserialize, Debug)]
struct EpochInfo {
    absoluteSlot: u64,
    blockHeight: u64,
    epoch: u64,
    slotIndex: u64,
    slotsInEpoch: u64,
    transactionCount: u64
}

#[derive(Deserialize, Debug)]
struct Account {
    address: Pubkey,
    balance: u64,
    is_new_create: bool,
}


async fn call_rpc(gist_body: &Value)->Result<Response, Box<dyn Error>> {
    //let uri = "https://api.devnet.solana.com/";
    let uri = "https://api.mainnet-beta.solana.com/";
    let response = Client::new()
        .post(uri)
        .json(gist_body)
        .send().await?;
    Ok(response)
}


// Get the info of current epoch
async fn get_current_epoch_info()->Result<EpochInfoResponse, Box<dyn Error>>{
    let gist_body = json!({
            "jsonrpc": "2.0",
            "method": "getEpochInfo",
            "id": 1
            });
    let response = call_rpc(&gist_body).await?;

    //println!("response: {:?}",response);
    let response: EpochInfoResponse = response.json().await?;
    Ok(response)
}

async fn get_block(block_height: u64)->Result<BlockResponse, Box<dyn Error>>{
    let gist_body = json!({
        "jsonrpc": "2.0",
        "method": "getConfirmedBlock",
        "params": [block_height, "base64"],
        "id": 1
        });

    // Call RPC
    let response = call_rpc(&gist_body).await?;
    //println!("getConfirmedBlock response: {:?}",&response.text().await);

    // Parse the response
    let result: BlockResponse = response.json().await?;
    Ok(result)
}

// fn get_account_from_string(line: &str) -> Option<TypeAccountPublicAddress>{
//     // Example
//     // Account 0: srw- dv1LfzJvDF7S1fBKpFgKoKXK5yoSosmkAdfbxBo1GqJ (fee payer)
//     // Account 1: -rw- 5MMCR4NbTZqjthjLGywmeT66iwE9J9f7kjtxzJjwfUx2
//     let mut acc = None;
//     let re = Regex::new(r"Account \d+: [srwx-]{4} (\S+)").unwrap();
//     for cap in re.captures_iter(line) {
//         //println!("***Acc: {}\n", &cap[1]);
//         acc = Some(cap[1].to_string());
//     }
//     acc
//
// }

// fn get_accounts_from_encoded_transaction(encoded_transaction: &String, base_mode: &String)-> Vec<TypeAccountPublicAddress>{
//     let mut accounts = Vec::new();
//     //let encoded_transaction = "AfmhCkp72aPvZR5xdRUIVNqR76711XZvwx0fTQvrHVmpS+Cz9PByQTAT95BzN0Fe8oglreNP9QPac7iEZpNXKwcBAAMFCXTX0M+X39utFbKwA9FQQ4tt6jzejcK2FqmPryDOZ+1ApTNTQmIsWCfxeddcdNN1OYwAeR1nPurxgVFgOFiEkwan1RcZLwqvxvJl4/t3zHragsUp0L47E24tAFUgAAAABqfVFxjHdMkoVmOYaR1etoteuKObS21cc1VbIQAAAAAHYUgdNXR0u3xNdiTr072z2DVec9EQQ/wNo1OAAAAAALy4mTmcCr96b/vHCOUBuJ/6fHpQIyw8wous4pPg3B5wAQQEAQIDAD0CAAAAAQAAAAAAAADXHUIDAAAAACTEC3xYed21Qche1aM9Rn5KibhEE6ue0hDW5Uzy9s/eAdd5lmAAAAAA";
//     // Run decode-transaction CLI for decode transaction
//     let result = Command::new("solana")
//         .arg("decode-transaction")
//         .arg(encoded_transaction)
//         .arg(base_mode)
//         .output()
//         .expect("ls command failed to start");
//     let stdout = String::from_utf8_lossy(&result.stdout);
//     print!("{}\n",&stdout);
//     let stdout = stdout.split("\n");
//
//     //println!("status: {}", result.status);
//
//
//     for s in stdout{
//         //println!("stdout: {}", s);
//         let acc = get_account_from_string(s);
//
//         match acc {
//             Some(acc) => {
//                 accounts.push(acc.clone());
//             },
//             None => (),
//         }
//     }
//     accounts
//     //print!("{:?}",accounts);
//
// }

/// Using solana SDK for decode transaction
fn get_address_from_encoded_transaction(encoded_transaction: &String, base_mode: &str) -> Vec<Pubkey> {
    let transaction = decode_transaction(encoded_transaction,base_mode).unwrap();
    transaction.message.account_keys
}


// fn get_accounts_in_block(br: BlockResponse) -> Vec<Vec<TypeAccountPublicAddress>>{
//     let transactions = br.result.transactions;
//
//     let mut accounts_in_block = Vec::new();
//
//     // Each transaction
//     for transaction in transactions{
//         // Get encoded transaction and base mode
//         let encoded_transaction = &transaction.transaction[0];
//         let base_mode = &transaction.transaction[1];
//
//         // Get accounts for each transaction
//         let accounts = get_accounts_from_encoded_transaction(encoded_transaction,base_mode);
//         print!("{:?}\n",&accounts);
//         accounts_in_block.push(accounts);
//     }
//     accounts_in_block
// }

// fn get_sol_transfer(block_response: BlockResponse) -> u128{
//     let accounts = get_accounts_in_block(block_response).iter();
//     let transactions = block_response.result.transactions.iter();
//     let system_program_addr = "11111111111111111111111111111111";
//     accounts
//         .zip(transactions)
//         .map(|(acc , trans)| {
//             if acc.contains(system_program_addr){
//                 trans.meta.postBalances[0]-
//             }
//         })
//
//     for transaction in block_response.result.transactions{
//         transaction.transaction
//     }
//
// }




fn store_into_solana_block(connection: &PgConnection, responses: &Vec<BlockResponse>){
    for response in responses {
        /// Prepare metrics
        let transaction_number = response.result.transactions.len();
        let timestamp = response.result.blockTime;
        let block_number = response.result.parentSlot + 1;
        let sol_transfer= get_sol_transfer_in_block(&response);
        println!("Sum sol transfer:{}",sol_transfer);
        let fee = get_fee_in_block(&response);


        let post = create_block_record(
        connection,
            block_number,
            timestamp,
            transaction_number as u128,
            sol_transfer,
            fee);
        println!("\nSaved block {} timestamp {} with transaction number {}",post.block_number, post.timestamp, post.transaction_number);
    }

}

fn get_accounts_from_encoded_transaction(encoded_transaction: &String, base_mode: &str, postBalances: &Vec<TypeTokenUnit>, preBalances: &Vec<TypeTokenUnit>) -> Vec<Account>{
    let mut accounts = Vec::new();
    let transaction = decode_transaction(encoded_transaction,base_mode).unwrap();
    let addresses = transaction.message.account_keys;
    for (address,post_balance,pre_balance) in izip!(addresses,postBalances,preBalances){
        let is_new_create = (*pre_balance == 0u128);
        let account = Account{
            address,
            balance: *post_balance as u64,
            is_new_create,
        };
        accounts.push(account);
    }

    accounts
}

fn get_accounts_in_block(br: &BlockResponse) -> Vec<Vec<Account>>{
    let transactions = &br.result.transactions;

    let mut accounts_in_block = Vec::new();

    // Each transaction
    for transaction in transactions{
        // Get encoded transaction and base mode
        let encoded_transaction = &transaction.transaction[0];
        let base_mode = &transaction.transaction[1];
        let postBalances = &transaction.meta.postBalances;
        let preBalances = &transaction.meta.preBalances;

        // Get accounts for each transaction
        let accounts = get_accounts_from_encoded_transaction(encoded_transaction, base_mode,postBalances,preBalances);
        print!("{:?}\n",&accounts);
        accounts_in_block.push(accounts);
    }
    accounts_in_block
}

fn store_into_solana_address(connection: &PgConnection, responses: &Vec<BlockResponse>){
    for response in responses {
        /// Prepare metrics
        let timestamp = response.result.blockTime;
        let block_number = response.result.parentSlot + 1;
        let accounts_in_block = get_accounts_in_block(response);
        for accounts_in_transaction in accounts_in_block{
            for account in accounts_in_transaction{
                let address = "".to_string();

                let post = create_address_record(
                    connection,
                    block_number,
                    timestamp,
                    &account.address.to_string(),
                    account.is_new_create,
                    account.balance as u128);
                println!("\nSaved block {} timestamp {} with address {}, balance {}, is_new {}",
                         block_number,
                         timestamp,
                         &account.address.to_string(),
                         account.balance,
                         account.is_new_create
                );
            }
        }


    }

}


async fn get_record_and_store_solana_db(connection: &PgConnection, current_block_height: u64) -> Result<(), Box<dyn Error>>{
    let response  = get_block(current_block_height).await?;
    //println!("Current block: {:?}",response);
    let responses = Vec::from([response]);
    store_into_solana_block(&connection, &responses);
    store_into_solana_address(&connection, &responses);
    Ok(())
}

pub fn get_sol_transfer_in_block(response: &BlockResponse) -> u128{
    /// Sum all sol in transactions
    let mut sol_transfer = 0u128;
    for transaction in &response.result.transactions{
        let blob = &transaction.transaction[0];
        let base_mode =  &transaction.transaction[1];
        /// Decode transaction
        let decoded_trans = decode_transaction(blob,base_mode);
        if let Some(decoded_trans) = decoded_trans {
            sol_transfer += get_sol_transfer_in_transaction(decoded_trans);
        }
    }

    sol_transfer
}

pub fn get_fee_in_block(response: &BlockResponse) -> u128{
    /// Sum all sol in transactions
    let mut fee = 0u128;
    for transaction in &response.result.transactions{
        fee += transaction.meta.fee;
    }

    fee
}


#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error>> {
    /// Number of block before current block will be indexed
    let block_number = 10;

    /// Connect to DB
    let connection = establish_connection();

    /// Get current epoch info
    let epoch_info = get_current_epoch_info().await?;
    let mut current_block_height = epoch_info.result.absoluteSlot;

    /// For debug only
    // let mut current_block_height = 60422367;
    // let response  = get_block(current_block_height).await.unwrap();
    // store_into_solana_block(&connection, Vec::from([response]));


    println!("block height: {}", current_block_height);


    /// Get block_number block data before current time
    // let responses = get_blocks(current_block_height,block_number).await?;
    // store_into_solana_block(&connection, responses);
    let mut next_block_height = current_block_height;
    let
    let limit_single_RPC_per_10sec = 40;
    let max_RPC_call = 8;
    let margin_call = 1;
    /// Get block from current time
    loop {
        /// Get current block height
        let epoch_info = get_current_epoch_info().await;
        match epoch_info {
            Ok(epoch_info) => current_block_height = epoch_info.result.absoluteSlot,
            Err(error) => {
                println!("Error cannot get epoch_info: {}",error);
                continue;
            },
        }


        /// Check if enough data is available
        if next_block_height + max_RPC_call + margin_call < current_block_height{
            println!("Data available! Indexed block {}, on-chain confirmed block {}",next_block_height,current_block_height);
            let mut threads:Vec<_> = Vec::new();
            for i in 0..max_RPC_call{
                let thread = get_record_and_store_solana_db(&connection, next_block_height);
                threads.push(thread);
                next_block_height += 1;
            }
            try_join_all(threads).await;
            /// To avoid limit single RPC call per 10 sec
            thread::sleep(Duration::from_millis((10000+1000)/limit_single_RPC_per_10sec*max_RPC_call));
        }
        else{
            println!("No new data!");
            thread::sleep(Duration::from_millis(300));
        }


    }

    /// Get address from block data
    /// Todo: use these accounts infos
    // for response in responses {
    //     //print!("{:?}\n",response.result.transactions);
    //     let accounts_in_block = get_accounts_in_block(br: BlockResponse);
    // }
    //print!("{:?}",responses);

    Ok(())
}