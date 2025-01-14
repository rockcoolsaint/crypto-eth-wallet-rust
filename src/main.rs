use anyhow::Result;

mod eth_wallet;

fn main() -> Result<()>{
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("secret key: {}", hex::encode(&secret_key.as_ref()));
    println!("pub key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_Wallet: {:?}", loaded_wallet);

    Ok(())
}
