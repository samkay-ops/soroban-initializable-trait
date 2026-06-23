// shared/initializable.rs
use soroban_sdk::{contracttype, Env, Address};

#[contracttype]
pub enum DataKey {
    Initialized,
}

pub trait Initializable {
    fn is_initialized(env: &Env) -> bool;
    fn mark_initialized(env: &Env);
}

pub fn initialize_guard(env: &Env) {
    if is_initialized(env) {
        panic!("Contract is already initialized");
    }
    mark_initialized(env);
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Initialized)
}

fn mark_initialized(env: &Env) {
    env.storage().instance().set(&DataKey::Initialized, &true);
}
