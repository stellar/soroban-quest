#![no_std]
use error::ContractError;
use soroban_sdk::{bytes, contract, contractimpl, panic_with_error, Address, Bytes, Env};

// We will use this `MAX` constant to bump our contract's storage entry
// immediately after we create the ledger entry. We are using the maximum
// lifetime allowed in Soroban, measured in the number of ledgers.
const MAX: u32 = 6_312_000; // This equates to roughly one year (5 seconds per ledger)

#[contract]
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

        // We then use `env.storage().temporary().set()` to store the value that
        // was passed, associating it with the `user` Address. This storage
        // entry is subject to expiration if it is not bumped. Following that,
        // it will need to be re-created before it is available to the contract.
        // We've chosen the `temporary` storage because these entries can be
        // re-created after expiration without needing to get into the weeds of
        // ledger entry restore operations.
        env.storage().temporary().set(&user, &value);
        env.storage().temporary().bump(&user, MAX); // We are bumping the entry by `MAX`.

        Ok(()) // return ok if function call succeeded
    }

    /// The `get()` function takes an `owner` parameter, accepting an Address
    /// object for it. We then use `env.storage().temporary().get()` to
    /// retrieve the value which has been associated with the supplied Address.
    /// If there is no data associated, return Bytes of length 0.
    pub fn get(env: Env, owner: Address) -> Bytes {
        env.storage()
            .temporary()
            .get(&owner)
            .unwrap_or_else(|| bytes!(&env)) // This uses `unwrap_or_else` and closure which only evaluates Bytes(0) when necessary.
    }
}

mod error;
mod test;
