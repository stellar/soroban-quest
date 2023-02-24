# Quest 2 - Auth Store <!-- omit in toc -->

## TL;DR

Do you consider yourself "advanced"? Do you think you can skip the high-level
theory and get away with rushing into this quest? Do you feel comfortable
completely mucking up your account before you even understand the assignment?

**In this quest, you will deploy the quest 2 Auth Store contract to the Stellar
Futurenet. Then you must successfully invoke the `put()` function to store some
data on the ledger, and then successfully invoke the `get()` function to
retrieve that same data.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Explore the Contract Code](#explore-the-contract-code)
  - [Storing Data](#storing-data)
  - [Retrieving Data](#retrieving-data)
  - [Simple Authentication](#simple-authentication)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quest, or you just need a refresher, we have
some (pretty extensive) instructions for the _mechanics_ of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need to use those
instructions again.

## The Task at Hand

So, down to brass tacks! Let's figure out how this quest works. A Soroban
contract can store arbitrary data in the ledger, and this data can then be
retrieved at a later time or in different contexts. Your job today is to both
store, and subsequently retrieve data by invoking a pair of functions in your
contract. (After it's been deployed to the Futurenet, of course!)

### Explore the Contract Code

Just like all the quests, this quest's [`lib.rs`](src/lib.rs) and
[`test.rs`](src/test.rs) files are commented with handy documentation and
explanations of what is happening. Be sure to check them out and read through
what the contract is up to.

**Important**: This quest has very complete comments and documentation
(particularly in the tests) in those two files. It will go **miles** to help
your understanding of Soroban, if you read through those files, and take the
time to understand what's happening.

### Storing Data

Soroban uses the `Env.storage().set()` function to store data in a contract's
ledger entries. You can think of these ledger entries as key-value storage that
can only be accessed through the contract that owns it. You can construct a
contract's ledger entries in many different ways. They could be made up of very
simple elements like a symbol or number. Or, they can also be made from very
complex vectors or maps.

The ledger entries for this quest will store a supplied `value` alongside an
`Address`. Using Soroban's `Address::require_auth()` function gives us a simple
method of authenticating a user. Only someone who could successfully sign for an
address (and, thus, invoke the contract from the address) is permitted to store
data in this contract as that address.

_Invoke the contract's `put()` function to store some data into the contract's
ledger entry for your quest account._

### Retrieving Data

To retrieve data from within a contract's ledger entries, the
`Env.storage().get()` function is available to us. When called with a `key` that
corresponds to data the contract has previously stored, we get the `value`
stored alongside it in return.

The contract's `get()` function will retrieve stored data associated with any
address. When supplied with an `Address` as an argument, this function will
search for stored data corresponding to that address.

_Invoke the contract's `get()` function to retrieve contract data associated
with your quest account._

### Simple Authentication

OK, so what's the point of all this? Sure, it's pretty neat to be able to store
and retrieve data from the smart contract network. But, is there anything
more... "useful" about this?!

Well, sure! For starters, you can control _who_ is allowed to set _which_ of the
contract's data keys. The way we've coded this contract, **only** an invocation
with a valid authorization can store or modify data associated with a given
Address. The `Address::require_auth()` function can be a very powerful
authentication method that can be made both quite simple and quite complex.

## Further Reading

- Check out the [storing data][data-example] example contract for some further
  discussion about this method of storing and retrieving data.
- A discussion of more advanced authentication methods can be found in the
  [auth][auth-example] how-to guide.
- You can learn more about [persisting data][persist-data] in the "Learn"
  section of the Soroban documentation.

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[data-example]: https://soroban.stellar.org/docs/getting-started/storing-data
[auth-example]: https://soroban.stellar.org/docs/how-to-guides/auth
[persist-data]: https://soroban.stellar.org/docs/learn/persisting-data
