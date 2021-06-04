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
use indexer::{establish_connection, create_record};
use diesel::PgConnection;
use std::time::Duration;
use std::thread;
use futures::future::try_join_all;

type TypeTimeStamp = u128;
type TypeBlockSlot = u128;
type TypeTokenUnit = u128;
type TypeBlockHash = String;
type TypeTransactionStatusOk = String;
type TypeAccountPublicAddress = String;
type TypeInnerInstructions = serde_json::Value;
type TypeErr = Option<serde_json::Value>;
type TypePostTokenBalances = serde_json::Value;
type TypePreTokenBalances = serde_json::Value;


// Response of getBlock RPC
#[derive(Deserialize, Debug)]
struct BlockResponse {
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

async fn call_rpc(gist_body: &Value)->Result<Response, Box<dyn Error>> {
    let uri = "https://api.devnet.solana.com/";
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

async fn get_blocks(end_block: u64,block_number: u64)->Result<Vec<BlockResponse>, Box<dyn Error>>{

    let mut responses = Vec::new();

    // Hard-code the latest block number here
    //let start_block = 58538963;
    //let block_number = 10;

    // Get block_number blocks from start_block
    for i in (end_block-block_number)..end_block {
        let gist_body = json!({
            "jsonrpc": "2.0",
            "method": "getConfirmedBlock",
            "params": [i, "base64"],
            "id": 1
            });

        // Call RPC
        let response_rpc = call_rpc(&gist_body).await?;
        //println!("getConfirmedBlock response: {:?}",&response_rpc.text().await);

        // Parse the response
        let result = response_rpc.json().await;

        let response: BlockResponse;
        match result {
            Ok(re) => {
                response = re;
                //println!("response: {:?}",response);
                responses.push(response);
            },
            Err(error) => println!("Block: {}, Error: {:?}!",i,error),
        }

    }

    Ok(responses)
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

fn get_account_from_string(line: &str) -> Option<TypeAccountPublicAddress>{
    // Example
    // Account 0: srw- dv1LfzJvDF7S1fBKpFgKoKXK5yoSosmkAdfbxBo1GqJ (fee payer)
    // Account 1: -rw- 5MMCR4NbTZqjthjLGywmeT66iwE9J9f7kjtxzJjwfUx2
    let mut acc = None;
    let re = Regex::new(r"Account \d+: [srwx-]{4} (\S+)").unwrap();
    for cap in re.captures_iter(line) {
        //println!("***Acc: {}\n", &cap[1]);
        acc = Some(cap[1].to_string());
    }
    acc

}

fn get_accounts_from_encoded_transaction(encoded_transaction: &String, base_mode: &String)-> Vec<TypeAccountPublicAddress>{
    let mut accounts = Vec::new();
    //let encoded_transaction = "AfmhCkp72aPvZR5xdRUIVNqR76711XZvwx0fTQvrHVmpS+Cz9PByQTAT95BzN0Fe8oglreNP9QPac7iEZpNXKwcBAAMFCXTX0M+X39utFbKwA9FQQ4tt6jzejcK2FqmPryDOZ+1ApTNTQmIsWCfxeddcdNN1OYwAeR1nPurxgVFgOFiEkwan1RcZLwqvxvJl4/t3zHragsUp0L47E24tAFUgAAAABqfVFxjHdMkoVmOYaR1etoteuKObS21cc1VbIQAAAAAHYUgdNXR0u3xNdiTr072z2DVec9EQQ/wNo1OAAAAAALy4mTmcCr96b/vHCOUBuJ/6fHpQIyw8wous4pPg3B5wAQQEAQIDAD0CAAAAAQAAAAAAAADXHUIDAAAAACTEC3xYed21Qche1aM9Rn5KibhEE6ue0hDW5Uzy9s/eAdd5lmAAAAAA";
    // Run decode-transaction CLI for decode transaction
    let result = Command::new("solana")
        .arg("decode-transaction")
        .arg(encoded_transaction)
        .arg(base_mode)
        .output()
        .expect("ls command failed to start");
    let stdout = String::from_utf8_lossy(&result.stdout);
    print!("{}\n",&stdout);
    let stdout = stdout.split("\n");

    //println!("status: {}", result.status);


    for s in stdout{
        //println!("stdout: {}", s);
        let acc = get_account_from_string(s);

        match acc {
            Some(acc) => {
                accounts.push(acc.clone());
            },
            None => (),
        }
    }
    accounts
    //print!("{:?}",accounts);

}

fn store_time_transaction(connection: &PgConnection, block_number: u128, timestamp: TypeTimeStamp,transaction_number:u128){
    let post = create_record(connection, block_number, timestamp, transaction_number);
    println!("\nSaved record timestamp {} with transaction number {}", post.timestamp, post.transaction_number);
}

fn get_accounts_in_block(br: BlockResponse) -> Vec<Vec<TypeAccountPublicAddress>>{
    let transactions = br.result.transactions;

    let mut accounts_in_block = Vec::new();

    // Each transaction
    for transaction in transactions{
        // Get encoded transaction and base mode
        let encoded_transaction = &transaction.transaction[0];
        let base_mode = &transaction.transaction[1];

        // Get accounts for each transaction
        let accounts = get_accounts_from_encoded_transaction(encoded_transaction,base_mode);
        print!("{:?}\n",&accounts);
        accounts_in_block.push(accounts);
    }
    accounts_in_block
}

fn store_into_time_transaction(connection: &PgConnection, responses: Vec<BlockResponse>){
    for response in responses {
        let transaction_number = response.result.transactions.len();
        let timestamp = response.result.blockTime;
        let block_number = response.result.parentSlot + 1;
        store_time_transaction(connection,block_number, timestamp,transaction_number as u128);
    }

}

async fn get_record_and_store_time_transaction(connection: &PgConnection,current_block_height: u64) -> Result<(), Box<dyn Error>>{
    let response  = get_block(current_block_height).await?;
    //println!("Current block: {:?}",response);
    store_into_time_transaction(&connection, Vec::from([response]));
    Ok(())
}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error>> {
    /// Number of block before current block will be indexed
    let block_number = 10;

    /// Connect to DB
    let connection = establish_connection();

    /// Get current epoch info
    let epoch_info = get_current_epoch_info().await?;
    let mut current_block_height = epoch_info.result.blockHeight;

    println!("block height: {}", current_block_height);


    /// Get block_number block data before current time
    // let responses = get_blocks(current_block_height,block_number).await?;
    // store_into_time_transaction(&connection, responses);
    let mut next_block_height = current_block_height;
    let max_RPC_call = 10;
    /// Get block from current time
    loop {
        /// Get current block height
        let epoch_info = get_current_epoch_info().await;
        match epoch_info {
            Ok(epoch_info) => current_block_height = epoch_info.result.blockHeight,
            Err(error) => {
                println!("Error cannot get epoch_info: {}",error);
                continue;
            },
        }


        /// Check if enough data is available
        if next_block_height + max_RPC_call < current_block_height {
            println!("Data available!");
            let mut threads:Vec<_> = Vec::new();
            for i in 0..max_RPC_call{
                let thread = get_record_and_store_time_transaction(&connection, next_block_height);
                threads.push(thread);
                next_block_height += 1;
            }
            try_join_all(threads).await;
        }
        else{
            println!("No new data!")
        }
        thread::sleep(Duration::from_millis(1000));

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