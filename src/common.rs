extern crate home;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

// Global Configurations for the password manager
pub enum GlobalConfiguration{
    HomeDir,
    StoreDir,
}

// Function to pass back home dir and store dir path locations
impl GlobalConfiguration {
    pub fn value(&self) -> super::errors::Result<String>{
        let hdir = home::home_dir();
        match hdir {
            Some(path) => {
                //a home env variable exists
                let mut hdirfinal = path.display().to_string();
                hdirfinal.push_str("/.passmanager");
                match *self {
                    GlobalConfiguration::HomeDir => Ok(hdirfinal),
                    GlobalConfiguration::StoreDir => {
                        hdirfinal.push_str("/.store");
                        Ok(hdirfinal)
                    },
                }
            }
            None => {
                return Err(super::errors::PasswordStoreError::HomeDirError)
            }
        }
    }
}

// Enum class for message templates for user message
pub enum UserMessage<'a>{
    // Inform user that they are creating a new store
    CreatingNewStore(&'a str),
    // Inform user that they store creation was successful
    StoreCreationSuccessful,
    // Inform user that base directory has been created
    CreatedBaseDir,
    // Inform user that store directory has been created
    CreatedStoreDir,
}

impl UserMessage<'_>{
    pub fn value(&self) -> String {
        let mut message = String::new();
        match *self {
            UserMessage::CreatingNewStore(store_name) => {
                message.push_str(&format!("Creating store with name: {}", store_name));
                message

            },
            UserMessage::StoreCreationSuccessful => "Store created successfully!".to_string(),
            UserMessage::CreatedBaseDir => "Base dir created!".to_string(),
            UserMessage::CreatedStoreDir => "Base store dir created!".to_string(),
        }
    }
}

// Hashes name input string
// Returns str reference to hashed str name
pub fn calculate_store_name_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn base_dir_exist() -> bool{
    match GlobalConfiguration::HomeDir.value(){
        Ok(path) => Path::new(&path).is_dir(),
        Err(e) => false
    }
}
