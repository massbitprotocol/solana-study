
use solana_transaction_status::{EncodedTransaction,UiTransactionEncoding};
use solana_sdk;
use solana_sdk::program_utils::limited_deserialize;
fn main(){
    let encode_trans = "AZkhMW7dbIZfg3bvFtC60K9u19pnj8RCvZZTv5C2FHB6pWM2HLKaBk/VjrGAnalO/u26tl85eD7T6n0LxqkimgIBAAEDClrNBKMsMKt9IZ8AiJwWnXOyVj1N+ZkcCh9K8GlRngvQbX6GZxfU+TTnG855FaE8v+EXQfNVQW+lbFylHELYQgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAzhkJF3mQT9yv74RHe6sNTN9GnOgE9r9iaLbKF9z4oBAgIAAQwCAAAAwNRUBwAAAAA=";
    //let encode_trans = "AauO9UickwsDYNWMZH2+DpsvgRXaJ6uYHedM6uMIXvepOmuC/REtcq3yKbDyicCCaOoCQvaITXluMB83Mh52YQEBAAIGdxpKE4kvtsE/HVIYAFjMuYdVmfVSPQsDDyxNrnjnlm790TnNvRMZIWiMfldkdMVvqnPfFAA5+M4xfvuyR/xg3/Gm32WPol4nQOkIkxg3cLzqAY97RTRNCoyLLFuYoeNvDzgJ2D1NxLLY37ddnFlvb8p49JufrVov/d9MuWpR+6GFDy1uAqR6+CTQmradxC1wyyjL+iSft+5XudJWwSdi7wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAeYUXxaZrV1d1+JF9Txvwqnk/wzUGplyS8B/j2O6m1AECBAUBAgMCAgcAAwAAAAEABQIAAAwCAAAAmA8AAAAAAAA=";
    let transaction = EncodedTransaction::Binary(encode_trans.to_string(),
        UiTransactionEncoding::Base64,
    );
    println!("Decode Transaction {:#?}",&transaction.decode().unwrap().message.instructions[0].data);

    let data = &transaction.decode().unwrap().message.instructions[0].data;


    //if program_pubkey == solana_sdk::system_program::id() {
    let system_instruction =
        limited_deserialize::<solana_sdk::system_instruction::SystemInstruction,>(data);
    println!("Decode Intruction data: {:#?}",system_instruction);

}
