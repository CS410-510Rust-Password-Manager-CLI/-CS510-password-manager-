use crate::generic::errors::{
    Result,
    PasswordStoreError
};
use crate::generic::common::{
    GlobalConfiguration,
    UserMessage,
    calculate_store_name_hash,
    does_store_exist,
    get_all_secrets,

};
use std::fs;
use std::io;
use std::io::Write;

// Deletes a secret from a secret store
// When deleting secret store, verify the store name before deleting
pub fn delete_secret_store(store_name: &str) -> Result<'static, ()>{
    // Hash store name, find file, verify, delete
    // Throw error if user inputs store name wrong in verify
    // Print UserMessage on successful deletion

    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist)
    }

    //hash the store name
    let hash_store_name = calculate_store_name_hash(store_name);

    //get file path of hashed store name
    let file_path = format!("{}/{}.json", GlobalConfiguration::StoreDir.value().unwrap(), hash_store_name);

    //verify store name
    let mut buffer = String::new();
    print!("\nEnter storename again to verify: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("could not read input");

    let verified = buffer.trim();
    if verified != store_name {
        return Err(PasswordStoreError::ErrorMisMatchStoreName)
    }

    //delete the file
    match fs::remove_file(file_path) {
        Err(_e) => return Err(PasswordStoreError::ErrorStoreDoesNotExist),
        Ok(_a) => {
            println!("{}", UserMessage::DeletedEntrySuccessfully.value());
            Ok(())
        }
    }
    
}

// Deletes a secret store
pub fn delete_secret(store_name: &str, entry_name: &str) -> Result<'static, ()>{
    // Check if this store exists, if not return error *
    // Get all secrets from this specific store
    // Make a copy
    // Iterate through and find the specific entry we want to delete
    //      ex: look at get_raw_secret() in get.rs
    // Delete
    // Once we have the correct secret, delete it from the EntryStore data object
    // -------------------------------------------------
    // Write the EntryStore data object back to the disk

    if !does_store_exist(store_name) {
        return Err(PasswordStoreError::ErrorStoreDoesNotExist)
    }

    let all_secrets = get_all_secrets(store_name).unwrap();
    let mut all_secrets_final = get_all_secrets(store_name).unwrap();

    let mut count = 0;
    let mut found = false;

    //loop through entries to find matching name
    for entry in all_secrets.entries {
        if entry.name == entry_name {
            found = true;
            break;
        }
        count += 1;
    }

    //remove entry from entries vector where name matches, if match found
    if found {
        all_secrets_final.entries.remove(count);
    } else {
        return Err(PasswordStoreError::ErrorNoEntryNameMatch);
    }

    //TODO: write back to disk
    Ok(())
}