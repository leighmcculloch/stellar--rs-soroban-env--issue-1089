#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Env};

#[contracterror]
#[derive(Copy, Clone, PartialEq)]
pub enum Error {
    // Counter limit, u32 max, is reached.
    LimitReached = 1,
}

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn counter(env: Env) -> i32 {
        env.storage()
            .instance()
            .get(&symbol_short!("counter"))
            .unwrap_or(0)
    }

    pub fn count(env: Env) -> Result<i32, Error> {
        let counter = Self::counter(env.clone());
        let Some(updated_counter) = counter.checked_add(1) else {
            return Err(Error::LimitReached);
        };
        env.storage()
            .instance()
            .set(&symbol_short!("counter"), &updated_counter);
        Ok(updated_counter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Counter);
        let client = CounterClient::new(&env, &contract_id);

        assert_eq!(client.counter(), 0);
        assert_eq!(client.count(), 1);
        assert_eq!(client.count(), 2);
        assert_eq!(client.count(), 3);
        assert_eq!(client.count(), 4);
        assert_eq!(client.count(), 5);
        assert_eq!(client.counter(), 5);
    }
}
