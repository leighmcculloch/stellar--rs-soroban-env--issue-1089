#![no_std]
#![cfg(test)]

use soroban_sdk::{Address, Env, TryIntoVal};

mod installer {
    soroban_sdk::contractimport!(file = "../out/installer.wasm");
}

mod deployer {
    soroban_sdk::contractimport!(file = "../out/deployer.wasm");
}

mod counter {
    soroban_sdk::contractimport!(file = "../out/counter.wasm");
}

#[test]
fn test() {
    let env = Env::default();

    let contract_id = env.register_contract_wasm(None, installer::WASM);
    let client = installer::Client::new(&env, &contract_id);
    env.to_snapshot_file("snapshot1.json");
    client.install();

    env.to_snapshot_file("snapshot2.json");

    let contract_id: soroban_sdk::xdr::ScAddress = contract_id.try_into().unwrap();
    let env = Env::from_snapshot(env.to_snapshot());
    let contract_id: Address = contract_id.try_into_val(&env).unwrap();
    let client = deployer::Client::new(&env, &contract_id);

    let counter_addr = client.deploy();
    let counter_client = counter::Client::new(&env, &counter_addr);
    assert_eq!(counter_client.counter(), 0);
    assert_eq!(counter_client.count(), 1);
    assert_eq!(counter_client.count(), 2);
    assert_eq!(counter_client.count(), 3);
    assert_eq!(counter_client.count(), 4);
    assert_eq!(counter_client.count(), 5);
    assert_eq!(counter_client.counter(), 5);
}
