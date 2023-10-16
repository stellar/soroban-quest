#![cfg(test)]

extern crate std;

use super::*;

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env,
};

/// The first test function, `test_valid_sequence()`, we test the contract
/// running in the sequence that is expected: parent approves on the token
/// contract, parent initializes the AllowanceContract, and child makes some
/// withdraws. Along the way, we check allowances and balances.
#[test]
fn test_valid_sequence() {
    // Just like always, we (say it with me) register the AllowanceContract
    // contract in a default Soroban environment, and build a client that can be
    // used to invoke the contract.
    let env = Env::default();
    let contract_address = env.register_contract(None, AllowanceContract);
    let client = AllowanceContractClient::new(&env, &contract_address);

    // For this contract, we'll need to set some ledger state to test against.
    // If you do the math, you can tell when we wrote this test!
    env.ledger().set(LedgerInfo {
        timestamp: 1669726145,
        ..env.ledger().get()
    });

    // We create two user addresses to test with, `u1` and `u2`
    let u1 = Address::random(&env); // `Parent` address
    let u2 = Address::random(&env); // `Child` address

    // We register a token contract that we can use to test our allowance and
    // payments. For testing purposes, the specific `token_address` we use for
    // this asset contract doesn't matter.
    let token_address = env.register_stellar_asset_contract(u1.clone());

    // We create a client that can be used for our token contract and we invoke
    // the `init` function. Again, in tests, the values we supply here are
    // inconsequential.
    let token_client = token::Client::new(&env, &token_address);
    let token_admin = token::StellarAssetClient::new(&env, &token_address);

    // Disable checks for authentication. See note in quest 2 tests for details.
    env.mock_all_auths();

    // We use the `u1` account to mint 1,000,000,000 stroops of our token to the
    // `Parent` (that is equal to 100 units of the asset).
    token_admin.mint(&u1, &1000000000);

    // We invoke the token contract's `approve` function as the `u1` address,
    // allowing our AllowanceContract to spend tokens out of the `u1` balance.
    // We are giving the contract a 500,000,000 stroop (== 50 units) allowance.
    token_client.approve(
        &u1,
        &contract_address,
        &500000000,
        &(env.ledger().sequence() + LedgerInfo::default().max_entry_expiration),
    );
    // We invoke the token contract's `allowance` function to ensure everything
    // has worked up to this point.
    assert_eq!(token_client.allowance(&u1, &contract_address), 500000000);

    // We invoke the `init` function of the AllowanceContract, providing the
    // starting arguments. These values result in a weekly allowance of
    // 9,615,384 stroops (== 0.9615384 units). Why, you big spender!
    client.init(
        &u1,                 // our `Parent` address
        &u2,                 // our `Child` address
        &token_address,      // our token contract id
        &500000000,          // 500000000 stroops == 50 units allowance for the year
        &(7 * 24 * 60 * 60), // 1 withdraw per week (7 days * 24 hours * 60 minutes * 60 seconds)
    );

    // We set new ledger state to simulate time passing. Here, we have increased
    // the timestamp by one second.
    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1),
        ..env.ledger().get()
    });

    // We invoke the inaugural `withdraw` to get the first allowance paid out.
    // Then, we make sure the `u2` account's token balance has increased to
    // 9,615,384. (Technically, at this point in time, that should be _exactly_
    // their balance, because they didn't have any of the asset minted to them.)
    client.withdraw(&u1);
    assert_eq!(token_client.balance(&u2), 9615384);

    // We (again) set new ledger state to simulate time passing. This time,
    // we've increased the timestamp by one second and one week.
    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1) + (7 * 24 * 60 * 60),
        ..env.ledger().get()
    });

    // We invoke `withdraw` again, and check that the `u2` token balance
    // reflects two allowance transfers.
    client.withdraw(&u2);
    assert_eq!(token_client.balance(&u2), 9615384 * 2);

    // A third time, we set new ledger state to simulate time passing. Here, we
    // skip ahead one second and two weeks from the `init` invocation.
    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1) + (7 * 24 * 60 * 60) + (7 * 24 * 60 * 60),
        ..env.ledger().get()
    });

    // We invoke `withdraw` again, and check that the `u2` token balance now
    // reflects three allowance transfers.
    client.withdraw(&u1);
    assert_eq!(token_client.balance(&u2), 9615384 * 3);
}

/// The `test_invalid_auth()` function will test that the contract panics when
/// someone who is not the `Parent` or `Child` invokes the `withdraw` function.
/// Again, this contract could be constructed to remove authentication from this
/// function altogether. Pretty neat!
#[test]
#[should_panic(expected = "Error(Contract, #3)")] // We want this test to panic since it is not authorized correctly.
fn test_invalid_auth() {
    // Almost everything in this test is identical to the previous one. We'll
    // drop a comment to let you know when things are getting interesting again.
    let env = Env::default();
    let contract_address = env.register_contract(None, AllowanceContract);
    let client = AllowanceContractClient::new(&env, &contract_address);

    env.ledger().set(LedgerInfo {
        timestamp: 1669726145,
        ..env.ledger().get()
    });

    let u1 = Address::random(&env); // `Parent` address
    let u2 = Address::random(&env); // `Child` address

    let token_address = env.register_stellar_asset_contract(u1.clone());

    let token_client = token::Client::new(&env, &token_address);
    let token_admin = token::StellarAssetClient::new(&env, &token_address);

    env.mock_all_auths();

    token_admin.mint(&u1, &1000000000);
    token_client.approve(
        &u1,
        &contract_address,
        &500000000,
        &(env.ledger().sequence() + LedgerInfo::default().max_entry_expiration),
    );
    assert_eq!(token_client.allowance(&u1, &contract_address), 500000000);

    client.init(&u1, &u2, &token_address, &500000000, &(7 * 24 * 60 * 60));

    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1),
        ..env.ledger().get()
    });

    // Ok, stop here! Instead of invoking as either of the `Parent` or `Child`
    // addresses, we are generating an entirely different address to invoke the
    // `withdraw` function. Since we expect to panic here, we stop.
    let u3 = Address::random(&env);
    client.withdraw(&u3);
}

/// In our next test function, `test_invalid_sequence()`, we are testing the
/// case where things are setup in the same way, but a second `withdraw`
/// invocation is made too quickly.
#[test]
#[should_panic(expected = "Error(Contract, #4)")] // We want this test to panic since it is withdrawing too quickly.
fn test_invalid_sequence() {
    // Almost everything in this test is identical to the previous one. We'll
    // drop a comment to let you know when things are getting interesting again.
    let env = Env::default();

    env.ledger().set(LedgerInfo {
        timestamp: 1669726145,
        ..env.ledger().get()
    });

    let u1 = Address::random(&env); // `Parent` address
    let u2 = Address::random(&env); // `Child` address

    let contract_address = env.register_contract(None, AllowanceContract);
    let client = AllowanceContractClient::new(&env, &contract_address);

    let token_address = env.register_stellar_asset_contract(u1.clone());
    let token_client = token::Client::new(&env, &token_address);
    let token_admin = token::StellarAssetClient::new(&env, &token_address);

    env.mock_all_auths();

    token_admin.mint(&u1, &1000000000);
    token_client.approve(
        &u1,
        &contract_address,
        &500000000,
        &(env.ledger().sequence() + LedgerInfo::default().max_entry_expiration),
    );

    assert_eq!(token_client.allowance(&u1, &contract_address), 500000000);

    client.init(&u1, &u2, &token_address, &500000000, &(7 * 24 * 60 * 60));

    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1),
        ..env.ledger().get()
    });

    client.withdraw(&u2);
    assert_eq!(token_client.balance(&u2), 9615384);

    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1) + (7 * 24 * 60 * 60),
        ..env.ledger().get()
    });

    client.withdraw(&u2);
    assert_eq!(token_client.balance(&u2), 9615384 * 2);

    // Ok, stop here! This time, for our third `withdraw` invocation, we are
    // only adding 20 seconds to the previous invocation. Since we've set up for
    // weekly allowance transfers, this attempt should fail.
    env.ledger().set(LedgerInfo {
        timestamp: (1669726145 + 1) + (7 * 24 * 60 * 60) + 20,
        ..env.ledger().get()
    });

    // We don't need an assertion here, since this invocation should fail and
    // respond with `Status(ContractError(4))`.
    client.withdraw(&u1);
}

/// In our next test function, `test_invalid_init()`, we test to make sure that
/// invoking the AllowanceContract `init` function with invalid arguments will
/// fail as expected. Specifically, we are passing `0` for the `step` value.
#[test]
#[should_panic(expected = "Error(Contract, #6)")] // We want this test to panic since we are giving an unusable argument.
fn test_invalid_init() {
    // Almost everything in this test is identical to the first one. We'll drop
    // a comment to let you know when things are getting interesting again.
    let env = Env::default();

    env.ledger().set(LedgerInfo {
        timestamp: 1669726145,
        ..env.ledger().get()
    });

    let u1 = Address::random(&env); // `Parent` address
    let u2 = Address::random(&env); // `Child` address

    let contract_address = env.register_contract(None, AllowanceContract);
    let client = AllowanceContractClient::new(&env, &contract_address);

    let token_address = env.register_stellar_asset_contract(u1.clone());
    let token_client = token::Client::new(&env, &token_address);
    let token_admin = token::StellarAssetClient::new(&env, &token_address);

    env.mock_all_auths();

    token_admin.mint(&u1, &1000000000);
    token_client.approve(
        &u1,
        &contract_address,
        &500000000,
        &(env.ledger().sequence() + LedgerInfo::default().max_entry_expiration),
    );

    assert_eq!(token_client.allowance(&u1, &contract_address), 500000000);

    // Ok, stop here! This time, when invoking `init`, we give a `0` for the
    // `step` field. This isn't possible because it would turn the
    // allowance-dripping faucet into a rusted old faucet that has been welded
    // shut. Also, dividing by zero is impossible. So, that's an important
    // consideration, too.
    client.init(
        &u1,            // our `Parent` account
        &u2,            // our `Child` account
        &token_address, // our token contract id
        &500000000,     // 500000000 stroops == 50 units allowance for the year
        &0,             // 0 withdraw per second (why would you even do this?)
    );

    // Again, there's no need for an assertion here, since this invocation
    // should fail and respond with `Status(ContractError(6))`.
}

/// In our final test function, `test_invalid_init_withdrawal()`, we test to
/// make sure that invoking the AllowanceContract `init` function with invalid
/// arguments will fail as expected. Specifically, we are passing the arguments
/// so that over the course of the year's allowance of 1 stroop, the child's
/// withdrawal amount would be impossibly small.
#[test]
#[should_panic(expected = "Error(Contract, #6)")] // We want this test to panic since we are giving an unusable argument.
fn test_invalid_init_withdrawal() {
    // Almost everything in this test is identical to the first one. We'll drop
    // a comment to let you know when things are getting interesting again.
    let env = Env::default();

    env.ledger().set(LedgerInfo {
        timestamp: 1669726145,
        ..env.ledger().get()
    });

    let u1 = Address::random(&env); // `Parent` address
    let u2 = Address::random(&env); // `Child` address

    let contract_address = env.register_contract(None, AllowanceContract);
    let client = AllowanceContractClient::new(&env, &contract_address);

    let token_address = env.register_stellar_asset_contract(u1.clone());
    let token_client = token::Client::new(&env, &token_address);
    let token_admin = token::StellarAssetClient::new(&env, &token_address);

    env.mock_all_auths();

    token_admin.mint(&u1, &1000000000);

    token_client.approve(
        &u1,
        &contract_address,
        &500000000,
        &(env.ledger().sequence() + LedgerInfo::default().max_entry_expiration),
    );

    assert_eq!(token_client.allowance(&u1, &contract_address), 500000000);

    // Ok, stop here! This time, when invoking `init`, we give a `1` for the
    // `amount` field and a `1` for the `step` field. If you've followed along
    // with the math so far, that comes out to 3.1709792e-15 **stroops** per
    // withdraw. That's even more precision than Microsoft Excel can handle!
    client.init(
        &u1,            // our `Parent` account
        &u2,            // our `Child` account
        &token_address, // our token contract id
        &1,             // 1 stroops == 0.0000001 units allowance for the year
        &1,             // 1 withdraw per second
    );

    // Again, there's no need for an assertion here, since this invocation
    // should fail and respond with `Status(ContractError(6))`.
}
