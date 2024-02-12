#![cfg(test)]
extern crate std;

use super::*;

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Bytes, Env, IntoVal, Symbol,
};

/// These tests are a a lot more interesting and much more involved than the
/// first quest, so let's dive into them a bit deeper. We are testing a few
/// different scenarios with invocations as both a "user" address and as a
/// "contract" address.

/// The first function, `test_store`, will test the values that are being stored
/// by our contract. This is accomplished by generating a couple user addresses,
/// storing data as those users, and ensuring retrieved data matches what we
/// would expect. We are also checking against a keypair that hasn't stored any
/// data, ensuring we receive Bytes of length 0 in return.
#[test]
fn test_store() {
    // Here we register the DataStore contract in a default Soroban environment,
    // and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_address = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_address);

    // The `mock_all_auths()` function will treat any further contract
    // invocations as if they had succeeded. This feature is only available as a
    // testing tool, but makes it easier to write tests so you can be sure the
    // authorization of a contract is working properly.
    env.mock_all_auths();

    // We're generating two test addresses, `u1` and `u2` that will be the
    // invokers of the contract functions.
    let u1 = Address::generate(&env);
    let u2 = Address::generate(&env);

    // For our `u1` address, we store the `Bytes` representation of "Hello
    // Soroban!" using the contract's `put` function.
    client.put(&u1, &bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // This is the hex value for "Hello Soroban!"

    // We take this opportunity to make sure the invocation of the `put`
    // function was authenticated (mocked, at least) the way we expect. We are
    // asserting that `env.auths()` (where the authentication is "recorded") is
    // equal to a vector of the user address, and an authorized invocation of
    // the function with its name and all its arguments. Notice we assert this
    // immediately after the `put` function is invoked and before anything else
    // is done with the `client`.
    assert_eq!(
        env.auths(),
        std::vec![(
            u1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_address.clone(),
                    Symbol::new(&env, "put"),
                    (
                        u1.clone(),
                        Bytes::from_slice(
                            &env,
                            &[72, 101, 108, 108, 111, 32, 83, 111, 114, 111, 98, 97, 110, 33] // This is the u8 Bytes array for "Hello Soroban!"
                        )
                    )
                        .into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // We then use the contract's `get` function to ensure we receive back the
    // expected value.
    assert_eq!(
        client.get(&u1),
        bytes!(&env, 0x48656c6c6f20536f726f62616e21)
    );

    // Before storing any value as the `u2` address, we check to ensure `get`
    // returns 0 Bytes (i.e. the address has no data to get).
    assert_eq!(client.get(&u2).len(), 0);

    // Now, as `u2`, we invoke the `put` function, storing the `Bytes`
    // representation of "Soroban Quest 2".
    client.put(&u2, &bytes![&env, 0x536f726f62616e2051756573742032]); // This is the hex value for "Soroban Quest 2"

    // Again, we assert that the mock authorization is what we expect.
    assert_eq!(
        env.auths(),
        std::vec![(
            u2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_address.clone(),
                    Symbol::new(&env, "put"),
                    (
                        u2.clone(),
                        Bytes::from_slice(
                            &env,
                            &[83, 111, 114, 111, 98, 97, 110, 32, 81, 117, 101, 115, 116, 32, 50] // This is the u8 Bytes array for "Soroban Quest 2"
                        )
                    )
                        .into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // We now assert that `get` should return the same value back to us.
    assert_eq!(
        client.get(&u2),
        bytes![&env, 0x536f726f62616e2051756573742032]
    );

    // Of course, we expect that the data for `u1` has not been overwritten by
    // `u2` invoking the `put` function.
    assert_eq!(
        client.get(&u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

/// The `test_store_value_too_short` function will attempt to store a value that
/// is shorter than the required 11 bytes long. We expect that this will end in
/// a panic, with the relevant error code from `error.rs`.
#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_store_value_too_short() {
    // Here we register the DataStore contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_address = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_address);

    // Mock authentication checks. See note in first test for more info.
    env.mock_all_auths();

    // We're generating a single test address, `u1`, which will be the invoker
    // of the contract's `put` function.
    let u1 = Address::generate(&env);

    // For our `u1` address, we attempt to store `Bytes(0, 7)` using the
    // contract's `put` function. We stop there, since we're just expecting the
    // contract to panic because the argument is too short.
    client.put(&u1, &bytes![&env, 0x007]);
}

/// For the next few tests, we are going to test our DataStore contract's
/// behavior when it is invoked from another contract. So, we are creating a
/// very simple smart contract here, that we can use for that purpose. It's
/// quite simple, and only exists as a client to invoke the main contract's
/// `put` and `get` functions.
#[contract]
pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    /// This function passes our supplied `data` argument to the DataStore
    /// contract's `put` function, associating it with this contract's Address.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - address of a DataStoreContract to invoke.
    /// * `user` - address of a user to store as the data key.
    /// * `data` - data to store alongside the user address.
    pub fn try_put(env: Env, contract_address: Address, user: Address, data: Bytes) {
        let client = DataStoreContractClient::new(&env, &contract_address);
        client.put(&user, &data);
    }

    /// This function invokes the `get` function from the DataStore contract,
    /// passing along an `owner` argument containing an Address.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - address of a DataStoreContract to invoke.
    /// * `owner` - address of a user to retrieve corresponding data for.
    pub fn try_get(env: Env, contract_address: Address, owner: Address) -> Bytes {
        let client = DataStoreContractClient::new(&env, &contract_address);
        client.get(&owner)
    }
}

/// The `test_contract_store` function will test whether a contract can invoke
/// the `put` function in our DataStore contract to store some data, and make
/// sure it's correctly storing the data by reading from the DataStore contract.
#[test]
fn test_contract_store() {
    // Similar to all Soroban tests, we create an environment, and register the
    // DataStore contract, and build a client to invoke this contract later in
    // the test.
    let env = Env::default();
    let data_store_contract_address = env.register_contract(None, DataStoreContract);
    let data_store_client = DataStoreContractClient::new(&env, &data_store_contract_address);

    // Mock authentication checks. See note in first test for more info.
    env.mock_all_auths();

    // We take an extra step to register our Caller contract in the Soroban
    // environment we've created, so we can test the cross-contract calls, using
    // its client. Additionally, the Address of the Caller contract will be the
    // "owner" of the stored data.
    let caller_contract_address = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, &caller_contract_address);

    // We are invoking the the DataStore contract's `put` function using our
    // Caller contract's `try_put` function.
    caller_client.try_put(
        // The address of the DataStore contract where we are storing our data
        &data_store_contract_address,
        // The address to be associated with the stored data
        &caller_contract_address,
        // This is the hex value for "Hello Soroban!"
        &bytes![&env, 0x48656c6c6f20536f726f62616e21],
    );

    // We are double-checking our stored data using the DataStore client, to
    // make sure the data has been correctly associated with the correct
    // contract's Address.
    assert_eq!(
        data_store_client.get(&caller_contract_address),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

/// This `test_contract_get` function will attempt to invoke the `get` function
/// of the DataStore contract, as another smart contract.
#[test]
fn test_contract_get() {
    // Similar to all Soroban tests, we create an environment, and register the
    // DataStore contract, and build a client to invoke this contract later in
    // the test.
    let env = Env::default();
    let data_store_contract_address = env.register_contract(None, DataStoreContract);
    let data_store_client = DataStoreContractClient::new(&env, &data_store_contract_address);

    // Mock authentication checks. See note in first test for more info.
    env.mock_all_auths();

    // We take an extra step to register our Caller contract in the Soroban
    // environment we've created, so we can test the cross-contract calls, using
    // its client.
    let caller_contract_address = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, &caller_contract_address);

    // We create an Address, `u1`, so we can invoke the `put` function, and
    // test against the value we store, when called from our contract later.
    let u1 = Address::generate(&env);
    data_store_client.put(&u1, &bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // This is the hex value for "Hello Soroban!"

    // We are invoking the the DataStore contract's `get` function by using
    // the `try_get` function in the Caller contract. We expect our returned
    // value to match the value we `put` before.
    let value = caller_client.try_get(&data_store_contract_address, &u1);
    assert_eq!(value, bytes!(&env, 0x48656c6c6f20536f726f62616e21));
}
