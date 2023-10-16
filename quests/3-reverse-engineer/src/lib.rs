#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct ReverseEngineerContract;

/// A constant is an **always immutable** value that is valid for the entire
/// time the program runs. When declaring a `const` in Rust, you must explicitly
/// provide a type for it at that time. Here the `SECRET` constant is typed
/// using `&str` and we define what that secret is.
const SECRET: &str = "dancinRaph";

/// The `ReverseEngineerContract` contains only one function: `submit()`.
#[contractimpl]
impl ReverseEngineerContract {
    // The `submit()` function only takes a single argument, `secret` and
    // returns either `true` or `false`, letting you know whether you submitted
    // the correct secret.
    pub fn submit(e: Env, secret: Symbol) -> bool {
        secret == Symbol::new(&e, SECRET)
    }
}

mod test;
