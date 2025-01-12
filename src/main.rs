mod eth_wallet;

fn main() {
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("secret key: {}", hex::encode(&secret_key.as_ref()));
    println!("pub key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);
}
