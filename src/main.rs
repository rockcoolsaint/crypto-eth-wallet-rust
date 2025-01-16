use std::env;
// use std::str::FromStr;

use anyhow::Result;
use web3::types::Address;

mod eth_wallet;
mod utils;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv::dotenv().ok();
    // let (secret_key, pub_key) = eth_wallet::generate_keypair();

    // println!("secret key: {}", hex::encode(&secret_key.as_ref()));
    // println!("pub key: {}", &pub_key.to_string());

    // let pub_address = eth_wallet::public_key_address(&pub_key);
    // println!("public address: {:?}", pub_address);

    // let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    // println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    // crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_Wallet: {:?}", loaded_wallet);

    let endpoint = env::var("SEPOLIA_URL")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("Wallet balance: {} eth", &balance);

    let address_str = "0xBF4643138f6F060eaD73C2Cea529dD9257d2c3E3";
    let address_bytes = hex::decode(&address_str[2..]).expect("Invalid hex address"); // Remove "0x" prefix
    // let transaction = eth_wallet::create_eth_transaction(Address::from_slice(b"0xBF4643138f6F060eaD73C2Cea529dD9257d2c3E3"), 0.01);
    let transaction = eth_wallet::create_eth_transaction(Address::from_slice(&address_bytes), 0.01);
    let transact_hash = eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;

    println!("transaction hash: {:?}", transact_hash);

    Ok(())
}
