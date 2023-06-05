#![no_std]
/// DON'T PANIC!! You don't actually have to change anything here. Yes, you
/// should read this file to understand what's happening, but there's nothing
/// that needs to be changed in the code for today.

/// As of `soroban_sdk` v0.8.x, the `token` spec is included within the SDK
/// itself! No more keeping track of and importing the Stellar Asset Contract's
/// wasm file. Just `use` it directly from the SDK! How cool!?
use soroban_sdk::{contracterror, contractimpl, contracttype, token, Address, Env};

/// An `Error` enum is used to meaningfully and concisely share error
/// information with a contract user.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ContractAlreadyInitialized = 1,
    ContractNotInitialized = 2,
    InvalidAuth = 3,
    ChildAlreadyWithdrawn = 4,
    InvalidInvoker = 5,
    InvalidArguments = 6,
}

/// We are using a `StorageKey` enum to store different bits and types of data,
/// but keying those pieces of data in a centralized place. This aids in
/// manageability and makes it easier to adapt our contract to store additional
/// pieces of data.
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Parent,       // Address
    Child,        // Address
    TokenAddress, // Address
    Amount,       // i128
    Step,         // u64
    Latest,       // u64
}

/// You know what's a pain? Re-declaring or re-calculating the same value over
/// and over again. We're going to use the number of seconds in a year more than
/// once in this contract, so let's use a `const` to declare it once and get it
/// out of the way.
const SECONDS_IN_YEAR: u64 = 365 * 24 * 60 * 60; // = 31,536,000 seconds (fyi)

pub struct AllowanceContract;

/// Seeing a `trait` may feel familiar. We used one in Quest 4, as well. When
/// utilizing a trait each of the functions that exist within your contract
/// implementation must be included in the trait, along with the arguments,
/// their expected types, and the return type of the function.
pub trait AllowanceTrait {
    // When `init`ializing the contract, we must specify some of the data that
    // will be stored (remember the `StorageKey`?) for the contract to reference.
    // We are using an `AccountId` for the `child` to highlight that a transfer
    // from one user to another is the intended use-case of this particular
    // contract. It also makes the Soroban CLI usage a bit cleaner and easier.
    fn init(
        e: Env,
        parent: Address,        // the parent account giving the allowance
        child: Address,         // the child account receiving the allowance
        token_address: Address, // the id of the token being transferred as an allowance
        amount: i128,           // the total allowance amount given for the year
        step: u64,              // how frequently (in seconds) a withdrawal can be made
    ) -> Result<(), Error>;

    // When `withdraw` is invoked, a transfer is made from the `Parent` asset
    // balance to the `Child` asset balance.
    fn withdraw(e: Env, invoker: Address) -> Result<(), Error>;
}

#[contractimpl]
impl AllowanceTrait for AllowanceContract {
    // Remember, before you can invoke `withdraw`, you must invoke `init`
    fn init(
        e: Env,
        parent: Address,
        child: Address,
        token_address: Address,
        amount: i128,
        step: u64,
    ) -> Result<(), Error> {
        // When running `init`, we want to make sure the function hasn't already
        // been invoked. Even though a few different `StorageKey`s are set
        // during init, it's enough to only check for one.
        let token_key = StorageKey::TokenAddress;
        if e.storage().has(&token_key) {
            return Err(Error::ContractAlreadyInitialized);
        }

        parent.require_auth();

        // You can't have a withdraw every 0 seconds. Obviously. Also, you can't
        // divide by 0. So say the calculators, at least.
        if step == 0 {
            return Err(Error::InvalidArguments);
        }

        // A withdrawal should never be `0`. I mean, really. At that point, why
        // even go through the trouble of setting this up?
        if (amount * step as i128) / SECONDS_IN_YEAR as i128 == 0 {
            return Err(Error::InvalidArguments);
        }

        // We are setting up all the data that this contract will store on the
        // ledger here. Nothing fancy here, just the same thing a few times.
        e.storage().set(&token_key, &token_address);
        e.storage().set(&StorageKey::Parent, &parent);
        e.storage().set(&StorageKey::Child, &child);
        e.storage().set(&StorageKey::Amount, &amount);
        e.storage().set(&StorageKey::Step, &step);

        // As an act of goodwill, we set the `Latest` withdraw to be in the past
        // and allow the `Child` to immediately make the first withdrawal. Just
        // to get them started, ya know.
        let current_ts = e.ledger().timestamp();
        e.storage().set(&StorageKey::Latest, &(current_ts - step));
        // This is the first time we've used `Env.ledger()` in these contracts.
        // The Soroban environment, by design, doesn't have a tremendous amount
        // of context about the current state of the Stellar network. One of the
        // few things it does know is the `timestamp()` of the most recently
        // closed ledger on the network. Check in the list of "Further
        // Resources" in the README to learn more about this.

        Ok(())
    }

    fn withdraw(e: Env, invoker: Address) -> Result<(), Error> {
        // Conversely from `init`, we want to make sure the contract _has_ been
        // initialized before a withdraw can be made.
        let token_key = StorageKey::TokenAddress;
        if !e.storage().has(&token_key) {
            return Err(Error::ContractNotInitialized);
        }

        let child: Address = e.storage().get(&StorageKey::Child).unwrap().unwrap();
        let parent: Address = e.storage().get(&StorageKey::Parent).unwrap().unwrap();

        // This part is one of the contract's really nifty tricks. We are using
        // `require_auth()` in this contract _only_ to make quest verification
        // simpler and more straight-forward. However, the `withdraw` function
        // doesn't _need_ to be written this way at all. By storing the `Child`
        // in our contract data at initialization, we can ensure they are
        // _always_ the beneficiary of the withdrawal. No matter who actually
        // makes the call to the contract, the child is always taken care of.

        // In that case, technically speaking, **anybody** would be able to
        // invoke the `withdraw` function in the contract (yes, even your cousin
        // Josh). In practice, for today's quest, the function **must** be
        // invoked by either the `Parent` or the `Child` address.
        if invoker != child && invoker != parent {
            return Err(Error::InvalidAuth);
        }
        invoker.require_auth();

        // We create a client to the token contract that we'll be able to use to
        // make the transfer later on. This should look familiar to Quest 4.
        let token_address: Address = e.storage().get(&token_key).unwrap().unwrap();
        let client = token::Client::new(&e, &token_address);

        // We do some really quick maths to figure out a couple things:
        // - `iterations` - the number of withdraws that can be made in a year
        // - `withdraw_amount` - the amount withdrawn for every iteration
        let step: u64 = e.storage().get(&StorageKey::Step).unwrap().unwrap();
        let iterations = SECONDS_IN_YEAR / step;
        let amount: i128 = e.storage().get(&StorageKey::Amount).unwrap().unwrap();
        let withdraw_amount = amount / iterations as i128;

        // Some more quick maths to make sure the `Latest` withdraw occurred _at
        // least_ `step` seconds ago. We don't want them draining the piggy bank
        // all at once, after all.
        let latest: u64 = e.storage().get(&StorageKey::Latest).unwrap().unwrap();
        if latest + step > e.ledger().timestamp() {
            return Err(Error::ChildAlreadyWithdrawn);
        }

        // This is where the magic happens! We use the client we set up for our
        // token contract earlier to invoke the `transfer_from` function. We're
        // using _this contract's_ allowance to spend the asset balance of the
        // `Parent` account and transfer funds _directly_ from the `Parent` to
        // the `Child`. That's amazing! Think of the implications and
        // possibilities! They're (and I mean this quite literally) endless!
        client.transfer_from(
            &e.current_contract_address(),
            &parent,
            &child,
            &withdraw_amount,
        );

        // We set a new `Latest` in our contract data to reflect that another
        // withdraw has taken place. The astute among you may notice this isn't
        // based off the ledger's `timestamp()`, but rather the latest withdraw.
        // This allows the child to "catch up" on any missed withdrawals. Very
        // kind of you. You're such a good parent!
        let new_latest = latest + step;
        e.storage().set(&StorageKey::Latest, &new_latest);

        Ok(())
    }
}

mod test;
