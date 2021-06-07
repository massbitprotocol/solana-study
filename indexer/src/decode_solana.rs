use solana_transaction_status::{EncodedTransaction,UiTransactionEncoding};
use solana_sdk;
use solana_sdk::{
    program_utils::limited_deserialize,
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    instruction::CompiledInstruction,
};


//use regex::Error;
use solana_sdk::instruction::InstructionError;
//use core::panicking::assert_failed;


pub fn decode_transaction(blob: &str, base_mode: &str) ->  Option<Transaction>{
    let transaction = match base_mode {
        "base64" => {
            EncodedTransaction::Binary(
                blob.to_string(),
                UiTransactionEncoding::Base64,
            )
        }
        _ => panic!("not support other base")
    };
    transaction.decode()
}

pub fn decode_instruction_data(instruction: CompiledInstruction, account_keys: Vec<Pubkey>) ->  Option<Result<SystemInstruction, InstructionError>> {
    let id:usize = instruction.program_id_index as usize;
    let program_pubkey =  account_keys[id];
    let si;
    if program_pubkey == solana_sdk::system_program::id() {
        si = Some(limited_deserialize::<SystemInstruction,>(&instruction.data));
        si
    }
    else{
        None
    }
}

pub fn get_sol_transfer_in_transaction(transaction: Transaction) -> u128{
    let mut sol_transfer = 0u128;
    let message = &transaction.message;

    /// Loop over each instruction
    for (instruction_index, instruction) in message.instructions.iter().enumerate() {
        let program_pubkey = message.account_keys[instruction.program_id_index as usize];
        let system_program_id = solana_sdk::system_program::id();
        //println!("program_pubkey: {:?}, system_program_id: {:?}",program_pubkey,system_program_id);

        /// Todo: Implement to get transfer in the Inner Instructions
        if program_pubkey==system_program_id {
            /// If this instruction is run by system program
            //println!("program_pubkey: {:?}, system_program_id: {:?}",program_pubkey,system_program_id);
            let data = &message.instructions[0].data;
            /// Decode instruction data
            let system_instruction =
                limited_deserialize::<solana_sdk::system_instruction::SystemInstruction, >(data);
            match system_instruction {
                Ok(si) => {
                    match si {
                        SystemInstruction::Transfer { lamports } => {
                            println!("Add lamports: {}, to sol_transfer: {}", lamports, sol_transfer);
                            sol_transfer += lamports as u128;
                        },
                        SystemInstruction::TransferWithSeed { lamports,from_seed,from_owner } => {
                            println!("Add lamports: {}, to sol_transfer: {}", lamports, sol_transfer);
                            sol_transfer += lamports as u128;
                        },
                        _ => continue,
                    }
                },
                _ => continue
            };
        }

    }

    sol_transfer
}









#[cfg(test)]
mod tests {
    #[test]
    fn test_decode_transfer() {
        let encode_trans = "AZkhMW7dbIZfg3bvFtC60K9u19pnj8RCvZZTv5C2FHB6pWM2HLKaBk/VjrGAnalO/u26tl85eD7T6n0LxqkimgIBAAEDClrNBKMsMKt9IZ8AiJwWnXOyVj1N+ZkcCh9K8GlRngvQbX6GZxfU+TTnG855FaE8v+EXQfNVQW+lbFylHELYQgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAzhkJF3mQT9yv74RHe6sNTN9GnOgE9r9iaLbKF9z4oBAgIAAQwCAAAAwNRUBwAAAAA=";
        //let encode_trans = "AauO9UickwsDYNWMZH2+DpsvgRXaJ6uYHedM6uMIXvepOmuC/REtcq3yKbDyicCCaOoCQvaITXluMB83Mh52YQEBAAIGdxpKE4kvtsE/HVIYAFjMuYdVmfVSPQsDDyxNrnjnlm790TnNvRMZIWiMfldkdMVvqnPfFAA5+M4xfvuyR/xg3/Gm32WPol4nQOkIkxg3cLzqAY97RTRNCoyLLFuYoeNvDzgJ2D1NxLLY37ddnFlvb8p49JufrVov/d9MuWpR+6GFDy1uAqR6+CTQmradxC1wyyjL+iSft+5XudJWwSdi7wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAeYUXxaZrV1d1+JF9Txvwqnk/wzUGplyS8B/j2O6m1AECBAUBAgMCAgcAAwAAAAEABQIAAAwCAAAAmA8AAAAAAAA=";
        let transaction = EncodedTransaction::Binary(encode_trans.to_string(),
                                                     UiTransactionEncoding::Base64,
        );
        println!("Decode Transaction {:#?}", &transaction.decode().unwrap().message.instructions[0].data);

        let data = &transaction.decode().unwrap().message.instructions[0].data;

        //if program_pubkey == solana_sdk::system_program::id() {
        let system_instruction =
            limited_deserialize::<solana_sdk::system_instruction::SystemInstruction, >(data);
        println!("Decode Intruction data: {:#?}", system_instruction);
        match system_instruction {
            Ok(si) => {
                match si {
                    SystemInstruction::Transfer { lamports } => assert_eq!(lamports, 123000000),
                    SystemInstruction::TransferWithSeed { lamports,from_seed,from_owner } => panic!("Not Transfer as expected"),
                    _ => panic!("Not Transfer as expected"),
                }
            },
            _ => panic!("Cannot decode into system instruction")
        };
    }
}
