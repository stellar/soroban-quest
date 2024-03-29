//! # Cross Contract
//!
//! A Soroban smart contract that calls yet another Soroban smart contract.

#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env};

// We import the compiled binary from our DataStore contract in Quest 2,
// allowing us to use that contract's types and client in this contract.
mod storage_contract {
    // We do this inside a `mod{}` block to avoid collisions between type names
    soroban_sdk::contractimport!(file = "./soroban_auth_store_contract.wasm");
}

/// We define a `trait` which can be used to create shared behavior between
/// types, and can be used to group together signatures for related functions.
pub trait StorageCallTrait {
    /// We define in this trait what the `inv_get` function signature must look
    /// like. This function will return the data that was stored, in `Bytes`
    ///
    /// # Arguments
    ///
    /// * `store_address` - the DataStore contract we are going to invoke
    /// * `owner` - the `Address` that was used to `put` data to that contract
    fn inv_get(env: Env, store_address: Address, owner: Address) -> Bytes;
}

#[contract]
pub struct CrossContractCallContract;

/// Our implementation of the `CrossContractCallContract` smart contract.
/// (enough "contracts" for you?!). It implements the trait we defined earlier,
/// and fleshes out what the `inv_get` function should do with its arguments and
/// how it should create the value to return.
#[contractimpl]
impl StorageCallTrait for CrossContractCallContract {
    /// The `inv_get` function will create a new client to the DataStore
    /// contract, and cross-invoke the `get` function, supplying the `owner`
    /// argument we provide when we invoked `inv_get`.
    fn inv_get(env: Env, store_address: Address, owner: Address) -> Bytes {
        // We create a client to interact with the DataStore contract.
        let storage_client = storage_contract::Client::new(&env, &store_address);

        // We invoke the `get` function through the client.
        storage_client.get(&owner)
    }
}

mod test;
