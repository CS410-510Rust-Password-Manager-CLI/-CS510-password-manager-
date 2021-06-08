use crate::generic::common::{
    get_all_secrets, does_store_exist, get_index, calculate_store_name_hash, write_to_file};
use crate::generic::errors::{
    Result,
    PasswordStoreError
};

use crate::operations::delete::delete_entry;

use crate::operations::create::{
    get_password,
    get_username,
    add_to_store
};
use crate::models::data_model::{Entry, EntryStore};

// Modify a data entry
pub fn modify_entry<'a>(store_name: &str, entry_name: &str) -> Result<'a, ()>{
    // Password protection around rotation
    if !does_store_exist(store_name) {
        // Throw error if the requested store does not exist
        return Err(PasswordStoreError::ErrorStoreDoesNotExist);
    }

    let failback_copy = get_all_secrets(store_name);
    match delete_entry(store_name, entry_name){
        Ok(()) => {
            let username: String = *get_username();
            let password: String = *(get_password().unwrap());
            let keyname = calculate_store_name_hash(store_name).to_string();

            match add_to_store(&keyname, &username, &password, store_name, entry_name){
                Ok(()) => Ok(()),
                Err(e) => {

                    // Fail back if error occurs with modifying password after
                    // the entry has already been removed
                    write_to_file(&(*failback_copy.unwrap()),
                                  &calculate_store_name_hash(entry_name).to_string());
                    Err(e)
                }
            }
        },
        Err(e) => Err(e)
    };
    Ok(())
}