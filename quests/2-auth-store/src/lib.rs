//! # Auth Store
//!
//! A Soroban smart contract that stores data entries keyed by user address.

#![no_std]
use error::ContractError;
use soroban_sdk::{bytes, contract, contractimpl, panic_with_error, Address, Bytes, Env};

// We will use this `MAX` constant to bump our contract's storage entry
// immediately after we create the ledger entry. We are using the maximum
// lifetime allowed in Soroban, measured in the number of ledgers.
const HIGH_TTL: u32 = 483_840; // Our "upper bounds" is roughly 28 days (5 seconds per ledger)
const LOW_TTL: u32 = HIGH_TTL / 2; // Our "lower bounds" is roughly 14 days (5 seconds per ledger)

#[contract]
pub struct DataStoreContract;

/// Our implementation of the `DataStoreContract` smart contract.
#[contractimpl]
impl DataStoreContract {
    /// Create and store a provided value in a temporary ledger entry. This
    /// `Bytes` value will be keyed by the provided user `Address`.
    ///
    /// # Arguments
    ///
    /// * `user` - accepts an `Address` object that will "own" the data being
    ///   stored. In Soroban, the `Address` type serves as an opaque identifier
    ///   for both accounts and contracts.
    /// * `value` - accepts a `Bytes` object to store, which can be supplied as
    ///   an array of `u8` values, an integer, or a hex-encoded string.
    ///
    /// # Panics
    ///
    /// If the Bytes argument provided is shorter than 11 bytes long.
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
        // entry is subject to archival if its time to live (TTL) is not
        // extended. Following that, it will need to be re-created before it is
        // available to the contract. We've chosen to use `temporary` storage
        // here because these entries can be re-created after archival without
        // needing to get into the weeds of ledger entry restore operations.
        env.storage().temporary().set(&user, &value);

        // Immediately after we create the storage entry, we are extending its
        // archival TTL so it will live on the ledger longer than the default
        // amount (hopefully long enough for you to complete the quest). We
        // provide two values here because we're essentially setting up a range
        // of acceptable lifetimes for our storage entry. If the `extend_ttl`
        // function were to discover that the TTL ledger is already greater than
        // our lower bounds, it would save us the fee and not bump the entry.
        // If, however, the entry's TTL ledger is closer to now than our lower
        // threshold, it will bump the TTL of the storage entry until `HIGH_TTL`
        // ledgers from now. (This strategy doesn't accomplish much in this
        // scenario, where the temporary entry was just created and will
        // **definitely** have a shorter lifetime than `LOW_TTL`, but it's a
        // useful opportunity to demonstrate how and why the `extend_ttl`
        // function operates this way.)
        env.storage()
            .temporary()
            .extend_ttl(&user, LOW_TTL, HIGH_TTL); // We are bumping the entry by `HIGH_TTL`.

        Ok(()) // return ok if function call succeeded
    }

    /// Use `env.storage().temporary().get()` to retrieve a value which has been
    /// associated with the supplied Address. If there is no data associated,
    /// return Bytes of length 0.
    ///
    /// # Arguments
    ///
    /// * `owner` - the Address to lookup a corresponding storage entry for.
    pub fn get(env: Env, owner: Address) -> Bytes {
        env.storage()
            .temporary()
            .get(&owner)
            .unwrap_or_else(|| bytes!(&env)) // This uses `unwrap_or_else` and closure which only evaluates Bytes(0) when necessary.
    }
}

mod error;
mod test;
