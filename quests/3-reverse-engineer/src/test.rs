#![cfg(test)]

use crate::{ReverseEngineerContract, ReverseEngineerContractClient, SECRET};
use soroban_sdk::{symbol_short, Env, Symbol};

/// We only have one test this time around. There is a single function in our
/// contract, and it only takes a single argument. So, we are only testing that
/// function.
#[test]
fn test_q3() {
    // Here we register the ReverseEngineer contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_address = env.register(ReverseEngineerContract, ());
    let client = ReverseEngineerContractClient::new(&env, &contract_address);

    // We invoke the ReverseEngineer contract's `submit()` function, providing a
    // value of "wrong" and we expect the contract to return `false`.
    assert!(!client.submit(&symbol_short!("wrong")));

    // We invoke the `submit()` function, with the correct secret word, and we
    // expect the contract to return `true` this time.
    assert!(client.submit(&Symbol::new(&env, SECRET)));
}
