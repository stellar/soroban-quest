//! # Custom Types
//!
//! A Soroban smart contract that requires the user to write and pass as
//! arguments custom Rust types.
//!
//! We promise: There is nothing to be changed in this file, you should probably
//! look into `types.rs`, though. There's bound to be some interesting stuff
//! there! http://bit.ly/3UajKQv

#![no_std]

use soroban_sdk::{contract, contractimpl, Env};
// The `use` keyword here allows us to _directly_ use the types we write in the
// `types.rs` file.
use types::*;

#[contract]
pub struct TypesContract;

/// Our implementation of the `TypesContract` smart contract. This contract is
/// really only here to accept a custom type as a parameter, and we will
/// consider it a successful invocation as long as you can submit a valid
/// argument for your custom type.
#[contractimpl]
impl TypesContract {
    // The `_` preceding the arguments in these functions is just how we tell
    // rust that it's expected for those arguments to be unused in the function.

    /// Create a rectangle type argument
    ///
    /// # Arguments
    ///
    /// * `_rect` - a `types::Rectangle` object
    pub fn c_rect(_env: Env, _rect: Rectangle) {}

    /// Create an animal type argument
    ///
    /// # Arguments
    ///
    /// * `_animal` - a `types::Animal` object
    pub fn c_animal(_env: Env, _animal: Animal) {}

    /// Create a user type argument
    ///
    /// # Arguments
    ///
    /// * `_user` - a `types::User` object
    pub fn c_user(_env: Env, _user: User) {}

    /// Create an RGB type argument
    ///
    /// # Arguments
    ///
    /// * `_rgb` - a `types::RGB` object
    pub fn c_rgb(_env: Env, _rgb: RGB) {}

    /// Create a color type argument
    ///
    /// # Arguments
    ///
    /// * `_color` - a `types::Color` object
    pub fn c_color(_env: Env, _val: Color) {}

    /// Create a participant type argument
    ///
    /// # Arguments
    ///
    /// * `_participant` - a `types::Participant` object
    pub fn c_part(_env: Env, _participant: Participant) {}

    /// Create a royal card type argument
    ///
    /// # Arguments
    ///
    /// * `_card` - a `types::RoyalCard` object
    pub fn c_card(_env: Env, _card: RoyalCard) {}
}

mod test;
mod types;
