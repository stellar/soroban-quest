#![no_std]
use soroban_sdk::{contractimpl, Address, Bytes, Env, contract};

/// We import the compiled binary from our auth store contract in Quest 2,
/// allowing us to use that contract's types and client in this contract.
mod storage_contract {
    // We do this inside a `mod{}` block to avoid collisions between type names
    soroban_sdk::contractimport!(file = "./soroban_auth_store_contract.wasm"); // TODO: `message: not yet implemented`?!
}

/// We define a `trait` which can be used to create shared behavior between
/// types, and can be used to group together signatures for related functions.
pub trait StorageCallTrait {
    // We define in this trait that the `inv_get()` function will accept:
    // - `store_address`: the other contract we are going to invoke
    // - `owner`: the `Address` that was used to `put` data to that contract
    // This function will return the data that was stored, in `Bytes`
    fn inv_get(env: Env, store_address: Address, owner: Address) -> Bytes;
}

#[contract]
pub struct CrossContractCallContract;

/// The `CrossContractCall` contract implements the trait we defined earlier,
/// and fleshes out what the `inv_get()` function should do with it arguments
/// and how it should create the value to return.
#[contractimpl]
impl StorageCallTrait for CrossContractCallContract {
    /// The `inv_get()` function will create a new client to the auth store
    /// contract, and cross-invoke the `get` function, supplying the `owner`
    /// argument we provide when we invoked `inv_get`.
    fn inv_get(env: Env, store_address: Address, owner: Address) -> Bytes {
        let storage_client = storage_contract::Client::new(&env, &store_address);
        storage_client.get(&owner)
    }
}

mod test;
