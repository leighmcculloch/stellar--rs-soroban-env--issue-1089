#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env};

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(env: Env) -> Address {
        let hash: BytesN<32> = unsafe {
            env.storage()
                .instance()
                .get(&symbol_short!("counter"))
                .unwrap_unchecked()
        };
        let salt = BytesN::from_array(&env, &[0; 32]);
        env.deployer().with_current_contract(salt).deploy(hash)
    }
}
