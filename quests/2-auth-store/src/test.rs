#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Address as _, Address, Env};

/// These tests are a a lot more interesting and much more involved than the
/// first quest, so let's dive into them a bit deeper. We are testing a few
/// different scenarios with invocations as both a "user" address and as a
/// "contract" address.

/// The first function, `test_store()`, will test the values that are being
/// stored by our contract. This is accomplished by generating a couple user
/// addresses, storing data as those users, and ensuring retrieved data matches
/// what we would expect. We are also checking against a keypair that hasn't
/// stored any data, ensuring we receive Bytes of length 0 in return.
#[test]
fn test_store() {
    // Here we register the DataStore contract in a default Soroban environment,
    // and build a client that can be used to invoke the contract.
    let env = Env::default();

    // The `mock_all_auths()` function will treat any further contract
    // invocations as if they had succeeded. This feature is only available as a
    // testing tool, but makes it easier to write tests when you're not as
    // concerned about valid authorization.
    env.mock_all_auths();

    let contract_address = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_address);

    // We're generating two test addresses, `u1` and `u2` that will be the
    // invokers of the contract functions.
    let u1 = Address::random(&env);
    let u2 = Address::random(&env);

    // For our `u1` address, we store the `Bytes` represetation of "Hello
    // Soroban!" using the contract's `put()` function. We then use the
    // contracts `get()` function to ensure we receive back the expected value.
    client.put(&u1, &bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // This is the hex value for "Hello Soroban!"

    // We then use the contract's `get()` function to ensure we receive back the
    // expected value.
    assert_eq!(
        client.get(&u1),
        bytes!(&env, 0x48656c6c6f20536f726f62616e21)
    );

    // Before storing any value as the `u2` address, we check to ensure `get()`
    // returns 0 Bytes (i.e. the address has no data to get).
    assert_eq!(client.get(&u2).len(), 0);
    // Now, as `u2`, we invoke the `put()` function, storing the `Bytes`
    // representation of "Soroban Quest 2".
    client.put(&u2, &bytes![&env, 0x536f726f62616e2051756573742032]); // This is the hex value for "Soroban Quest 2"

    // We now assert that `get()` should return the same value back to us.
    assert_eq!(
        client.get(&u2),
        bytes![&env, 0x536f726f62616e2051756573742032]
    );

    // Of course, we expect that the data for `u1` has not been overwritten by
    // `u2` invoking the `put()` function.
    assert_eq!(
        client.get(&u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

/// The `test_store_value_too_short()` function will attempt to store a value
/// that is shorter than the required 11 bytes long. We expect that this will
/// end in a panic, with the relevant error code from `error.rs`.
#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_store_value_too_short() {
    // Here we register the DataStore contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();

    // Disable checks for authentication. See note in previous test for more.
    env.mock_all_auths();

    let contract_address = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_address);

    // We're generating a single test address, `u1`, which will be the invoker
    // of the contract's `put()` function.
    let u1 = Address::random(&env);

    // For our `u1` address, we attempt to store `Bytes(0, 7)` using the
    // contract's `put()` function. We stop there, since we're just expecting
    // the contract to panic with the argument that's too short.
    client.put(&u1, &bytes![&env, 0x007]);
}

/// For the next few tests, we are going to test our DataStore contract's
/// behavior when it is invoked from another contract. So, we are creating a
/// very simple Smart Contract here, that we can use in them. It's quite simple,
/// and only exists as a client to invoke the main contract's `put()` and
/// `get()` functions.
#[contract]
pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    // This function passes our supplied `data` argument to the DataStore
    // contract `put()` function, associating it the the `contract_id` Address.
    pub fn try_put(env: Env, contract_address: Address, user: Address, data: Bytes) {
        let cli = DataStoreContractClient::new(&env, &contract_address);
        cli.put(&user, &data);
    }

    // This function invokes the `get()` function from the DataStore contract,
    // passing along an `owner` argument containing an Address.
    pub fn try_get(env: Env, contract_address: Address, owner: Address) -> Bytes {
        let cli = DataStoreContractClient::new(&env, &contract_address);
        cli.get(&owner)
    }
}

/// The `test_contract_store()` function will test whether a contract can invoke
/// the `put()` function in our DataStore contract to store some data, and make
/// sure it's correctly storing the data by reading from the DataStore contract.
#[test]
fn test_contract_store() {
    // Similar to all Soroban tests, we create an environment, and register the
    // DataStore contract, and build a client to invoke this contract later in
    // the test.
    let env = Env::default();

    // Disable checks for authentication. See note in previous test for more.
    env.mock_all_auths();

    let contract_address_data_store = env.register_contract(None, DataStoreContract);
    let data_store_client = DataStoreContractClient::new(&env, &contract_address_data_store);

    // We take an extra step to register our Caller contract in the environment,
    // so we can test the cross-contract calls, using its client. Additionally,
    // the Address of the Caller contract will be the "owner" of the stored
    // data.
    let contract_address_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, &contract_address_caller);

    // We are invoking the the DataStore contract's `put()` function using our
    // Caller contract's `try_put()` function.
    caller_client.try_put(
        // The address of the DataStore contract where we are storing our data
        &contract_address_data_store,
        // The address to be associated with the stored data
        &contract_address_caller,
        // This is the hex value for "Hello Soroban!"
        &bytes![&env, 0x48656c6c6f20536f726f62616e21],
    );

    // We are double-checking our stored data using the DataStore client, to
    // make sure the data has been correctly associated with the correct
    // contract's Address.
    assert_eq!(
        data_store_client.get(&contract_address_caller),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

/// This `test_contract_get()` function will attempt to invoke the `get()`
/// function of the DataStore contract, as another smart contract.
#[test]
fn test_contract_get() {
    // Similar to all Soroban tests, we create an environment, and register the
    // DataStore contract, and build a client to invoke this contract later in
    // the test.
    let env = Env::default();

    // Disable checks for authentication. See note in previous test for more.
    env.mock_all_auths();

    let contract_address_data_store = env.register_contract(None, DataStoreContract);
    let client_data_store = DataStoreContractClient::new(&env, &contract_address_data_store);

    // We take an extra step to register our Caller contract in the environment,
    // so we can test the cross-contract calls, using its client.
    let contract_address_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, &contract_address_caller);

    // We create an Address, `u1`, so we can invoke the `put()` function, and
    // test against the value we store, when called from our contract later.
    let u1 = Address::random(&env);
    client_data_store.put(&u1, &bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // This is the hex value for "Hello Soroban!"

    // We are invoking the the DataStore contract's `get()` function by using
    // the `try_get()` function in the Caller contract. We expect our returned
    // value to match the value we `put` before.
    let value = caller_client.try_get(&contract_address_data_store, &u1);
    assert_eq!(value, bytes!(&env, 0x48656c6c6f20536f726f62616e21));
}
