// We don't include the standard library to minimize compiled size.
// We also import a few macros and types we need from the `soroban_sdk`.
#![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec, contract, symbol_short};

#[contract]
pub struct HelloContract;

// Our `HelloContract` implementation contains only one function, `hello()`.
// This function will receive a `to` argument, and return a Vec made up of
// "Hello" and the supplied `to` value.
#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        // We use `Symbol::short` here, since our supplied string is fewer than
        // 10 characters. For strings up to 32 characters, use `Symbol::new`.
        vec![&env, symbol_short!("Hello"), to]
    }
}

// This `mod` declaration inserts the contents of `test.rs` into this file.
mod test;
