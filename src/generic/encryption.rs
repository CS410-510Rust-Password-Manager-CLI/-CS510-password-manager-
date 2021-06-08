use crate::generic::common::{
    GlobalConfiguration
};
use crate::models::data_model::{
    Entry,
    EntryStore
};

use std::fs::File;
use rsa::{PaddingScheme, PrivateKeyPemEncoding, PublicKey, RSAPrivateKey, RSAPublicKey};
use std::io::{Read, Write};
use rand::rngs::OsRng;
use std::path::Path;

// Creates a new RSA private key for every password entry
// Saves private key to pem file stored in the .keys
// Key name is based on the hashed value of the entry name
pub fn create_new_rsa_private_key(key_name: &str) -> std::io::Result<()> {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let new_key_file_path = format!(
        "{}/{}.pem",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    let mut file = File::create(new_key_file_path)?;
    let key_buf = priv_key.to_pem_pkcs1().unwrap();
    file.write_all(key_buf.as_bytes())?;
    Ok(())
}

pub fn encrypt_data_with_private_key(
    key_name: &str,
    username: &str,
    password: &str,
    hashed_store_name: &str,
    entry_name: &str,
) -> std::io::Result<()> {
    let key_file_path = format!(
        "{}/{}.pem",
        GlobalConfiguration::KeyStoreDir.value().unwrap(),
        key_name
    );
    let mut file = File::open(key_file_path).unwrap();
    let mut priv_key_buf = String::new();
    file.read_to_string(&mut priv_key_buf);

    let der_encoded = priv_key_buf
        .lines()
        .filter(|line| !line.starts_with("-"))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("failed to parse key");
    let pub_key = RSAPublicKey::from(&private_key);

    let mut rng = OsRng;
    let enc_username_data = pub_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            username.as_bytes(),
        )
        .expect("failed to encrypt username");
    let enc_password_data = pub_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            password.as_bytes(),
        )
        .expect("failed to encrypt password");

    // Need to read in current entries
    let mut new_store = EntryStore::new();

    // Create new data entry from encrypted data
    let new_entry = Entry{
        name: entry_name.to_string(),
        username: enc_username_data,
        password: enc_password_data,
    };

    new_store.entries.push(new_entry);

    let serialized_data = serde_json::to_string(&new_store).unwrap();
    let store_path = format!("{0}/{1}.json", GlobalConfiguration::StoreDir.value().unwrap(), hashed_store_name);
    println!("Path: {}", store_path);
    println!("Data: {}", serialized_data);
    let mut store_file = File::create(Path::new(&store_path)).unwrap();
    serde_json::to_writer(store_file, &new_store).unwrap();
    println!("Saved!");
    Ok(())
}