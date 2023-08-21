use super::storage_types::DataKey;
use soroban_sdk::{Address, Env,};

fn check_admin(env: &Env) -> bool {
    let key = DataKey::Admin;
    match env.storage().instance().get(&key) {
        Some(stored_admin) => &stored_admin == env.invoker(),
        None => false,
    }
}

fn get_admin(env: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    env.storage().instance().get(&key)
}

fn set_admin(env: &Env, admin: Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, admin);
}
