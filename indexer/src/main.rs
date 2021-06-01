#![allow(non_snake_case)]

// use error_chain::error_chain;
use serde::Deserialize;
use serde_json::json;
//use std::env;
use reqwest::Client;
use std::error::Error;
//use tokio_postgres::{NoTls};
use std::process::Command;
use regex::Regex;

type TypeTimeStamp = u128;
type TypeBlockSlot = u128;
type TypeTokenUnit = u128;
type TypeBlockHash = String;
type TypeInnerInstructions = String;
type TypeTransactionStatusOk = String;
type TypeAccountPublicAddress = String;

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
    err: Option<String>,
    fee: TypeTokenUnit,
    innerInstructions: Vec<TypeInnerInstructions>,
    logMessages: Vec<String>,
    postBalances: Vec<TypeTokenUnit>,
    postTokenBalances: Vec<TypeTokenUnit>,
    preBalances: Vec<TypeTokenUnit>,
    preTokenBalances: Vec<TypeTokenUnit>,
    status: TransactionStatus,

}

#[derive(Deserialize, Debug)]
struct TransactionStatus {
    Ok: Option<TypeTransactionStatusOk>
}



async fn get_block()->Result<Vec<BlockResponse>, Box<dyn Error>>{

    let uri = "https://api.devnet.solana.com/";
    let mut responses = Vec::new();

    // Hard-code the latest block number here
    let start_block = 58538963;
    let block_number = 1;
    for i in start_block..(start_block+block_number) {
        let gist_body = json!({
            "jsonrpc": "2.0",
            "method": "getConfirmedBlock",
            "params": [i, "base64"],
            "id": 1
            });

        let request_url = uri;

        let response = Client::new()
            .post(request_url)
            .json(&gist_body)
            .send().await?;

        let response: BlockResponse = response.json().await?;
        responses.push(response);
    }
    Ok(responses)
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


#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error>> {

    // Get block data
    let responses = get_block().await?;

    // Get address from block data
    for response in responses {
        //print!("{:?}\n",response.result.transactions);
        let transactions = response.result.transactions;

        // Each transaction
        for transaction in transactions{
            // Get encoded transaction and base mode
            let encoded_transaction = &transaction.transaction[0];
            let base_mode = &transaction.transaction[1];

            // Get accounts
            let accounts = get_accounts_from_encoded_transaction(encoded_transaction,base_mode);
            print!("{:?}\n",&accounts);
        }
    }
    Ok(())
}