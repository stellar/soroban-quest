//! # Reverse Engineer
//!
//! A Soroban smart contract that checks if a submitted string matches the
//! secret defined in the contract.

#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct ReverseEngineerContract;

// A constant is an **always immutable** value that is valid for the entire
// time the program runs. When declaring a `const` in Rust, you must explicitly
// provide a type for it at that time. Here the `SECRET` constant is typed
// using `&str` and we define what that secret is.
const SECRET: &str = "dancinRaph";

/// Our implementation of the `ReverseEngineerContract` smart contract.
#[contractimpl]
impl ReverseEngineerContract {
    /// Determine whether the submitted secret is correct. Returns `true` if the
    /// secret is correct, or `false` if not.
    ///
    /// # Arguments
    ///
    /// * `secret` - accepts a string that will be checked to see if it matches
    ///   the previously defined `SECRET` constant.
    pub fn submit(e: Env, secret: Symbol) -> bool {
        secret == Symbol::new(&e, SECRET)
    }
}

mod test;
