use soroban_sdk::{Env, Address};
use super::storage_types::{DataKey};

/// Checks if the invoker is an admin.
pub fn check_admin(env: &Env) -> bool {
    match env.data().get::<Address>(DataKey::Admin) {
        Some(Ok(admin)) => env.invoker() == admin,
        _ => false,
    }
}

/// Retrieves the address of the admin.
pub fn get_admin(env: &Env) -> Result<Address, &'static str> {
    match env.data().get::<Address>(DataKey::Admin) {
        Some(Ok(admin)) => Ok(admin),
        Some(Err(_)) => Err("Failed to convert data to address."),
        None => Err("Admin not set."),
    }
}

/// Sets the address for the admin.
pub fn set_admin(env: &Env, admin: Address) {
    env.data().set(DataKey::Admin, admin)
}
