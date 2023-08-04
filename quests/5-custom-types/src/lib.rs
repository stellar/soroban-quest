#![no_std]

/*
We promise: There is nothing to be changed in this file, you should probably
look into `types.rs`, though. There's bound to be some interesting stuff there!
http://bit.ly/3UajKQv
*/

use soroban_sdk::{contractimpl, Env, contract};
use types::*;

#[contract]
pub struct TypesContract;

/// This `TypesContract` contract is really only here to accept a custom type as
/// a parameter, and we will consider it a successful invocation as long as you
/// can submit a valid argument for your custom type.
#[contractimpl]
impl TypesContract {
    // The `_` preceding the arguments in these functions is just how we tell
    // rust that it's expected for those arguments to be unused in the function.
    pub fn c_rect(_env: Env, _rect: Rectangle) {}
    pub fn c_animal(_env: Env, _animal: Animal) {}
    pub fn c_user(_env: Env, _user: User) {}
    pub fn c_rgb(_env: Env, _rgb: RGB) {}
    pub fn c_color(_env: Env, _val: Color) {}
    pub fn c_part(_env: Env, _participant: Participant) {}
    pub fn c_card(_env: Env, _card: RoyalCard) {}
}

mod test;
mod types;
