#![no_std]
use error::ContractError;
use soroban_sdk::{bytes, contractimpl, panic_with_error, Address, Bytes, Env};

pub struct DataStoreContract;

/// The `DataStoreContract` contains both functions our contract can run when it
/// is invoked: `put()` and `get()`
#[contractimpl]
impl DataStoreContract {
    /// The `put()` function takes two parameters:
    /// `user` - accepts an `Address` object that will "own" the data being
    /// stored. The `Address` type serves as an opaque identifier for both
    /// accounts and contracts.
    /// `value` - accepts a `Bytes` object to store, which can be supplied as an
    /// array of `u8` values, an integer, or a hex-encoded string.
    pub fn put(env: Env, user: Address, value: Bytes) -> Result<(), ContractError> {
        // By calling `user.require_auth()`, we are ensuring the owner-to-be of
        // the stored data has given appropriate authorization to associate with
        // this data.
        user.require_auth();

        // We are ensuring the provided Bytes value length is at least 11 since
        // we want users to perform the String to Bytes conversion on their own,
        // without passing simple values like Bytes(7). We also want to
        // highlight some differences between Bytes and symbols (which must be
        // fewer than 10 characters, in many cases).
        if value.len() <= 10 {
            panic_with_error!(&env, ContractError::InputValueTooShort)
        }

        // We then use `env.storage().set()` to store the value that was passed,
        // associating it with the `user` Address.
        env.storage().set(&user, &value);

        Ok(()) // return ok if function call succeeded
    }

    /// The `get()` function takes an `owner` parameter, accepting an Address
    /// object for it. We then use `env.storage().get()` to retrieve the value
    /// which has been associated with the supplied Address. If there is no
    /// data associated, return Bytes of length 0.
    pub fn get(env: Env, owner: Address) -> Bytes {
        env.storage()
            .get(&owner)
            .unwrap_or_else(|| Ok(bytes!(&env))) // This uses `unwrap_or_else` and closure which only evaluates Bytes(0) when necessary.
            .unwrap()
    }
}

mod error;
mod test;
