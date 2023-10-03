#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

mod deployer {
    soroban_sdk::contractimport!(file = "../out/deployer.wasm");
}

mod counter {
    soroban_sdk::contractimport!(file = "../out/counter.wasm");
}

#[contract]
pub struct Installer;

#[contractimpl]
impl Installer {
    pub fn install(env: Env) {
        let counter = env.deployer().upload_contract_wasm(counter::WASM);
        env.storage()
            .instance()
            .set(&symbol_short!("counter"), &counter);
        let deployer = env.deployer().upload_contract_wasm(deployer::WASM);
        env.deployer().update_current_contract_wasm(deployer);
    }
}
