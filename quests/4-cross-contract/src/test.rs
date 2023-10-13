#![cfg(test)]

// We declare the we are planning to `use` the listed `crates` in our test
// (these have been defined in `lib.rs`)
use crate::{storage_contract, CrossContractCallContract, CrossContractCallContractClient};
use soroban_sdk::{bytes, testutils::Address as _, Address, Env};

#[test]
fn get_cross_call() {
    // Create a default Soroban environment.
    let env = Env::default();

    // Here we register the DataStore contract in the Soroban environment, and
    // build a client that can be used to invoke the contract.
    let storage_contract_address = env.register_contract_wasm(None, storage_contract::WASM);
    let storage_contract_client = storage_contract::Client::new(&env, &storage_contract_address);

    // Here we register the CrossContractCall contract in the Soroban
    // environment, and build a client that can be used to invoke the contract.
    let cross_call_contract_address = env.register_contract(None, CrossContractCallContract);
    let cross_call_contract_client =
        CrossContractCallContractClient::new(&env, &cross_call_contract_address);

    // Disable checks for authentication. See note in quest 2 tests for details.
    env.mock_all_auths();

    // We generate a test user, and invoke the `put` function to store some data
    // in the DataStore contract.
    let u1 = Address::random(&env);
    storage_contract_client.put(&u1, &bytes![&env, 0x48656c6c6f20536f726f62616e21]); // This is the hex value for "Hello Soroban!"

    // We invoke the `inv_get` function using our CrossContractCall client,
    // supplying the DataStore contract address and our test user as arguments.
    // We expect it to return the same data we already stored in the DataStore
    // contract.
    assert_eq!(
        cross_call_contract_client.inv_get(&storage_contract_address, &u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}
