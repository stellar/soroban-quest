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
  - [Expiration and Storage Types](#expiration-and-storage-types)
    - [Temporary Storage](#temporary-storage)
    - [Persistent Storage](#persistent-storage)
    - [Instance Storage](#instance-storage)
  - [Storing Data](#storing-data)
  - [Retrieving Data](#retrieving-data)
  - [Bumping Lifetimes](#bumping-lifetimes)
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

### Expiration and Storage Types

> **Note**: Preview 10 saw significant development and breaking changes due to
> the initial implementations of state expiration and a tiered set of storage
> types. We've updated this quest to include those changes. Although changes may
> occur in future releases, this document is current at the time of writing.

Do you know what is _incredibly unique_ and _amazingly powerful_ about Soroban?
The answer might surprise you: [**State expiration**][state-expiration]. A
real-world problem within the blockchain space is that of "ledger bloat" (i.e.,
the indefinite storage of enormous amounts of data that are supposed to live on
the blockchain "forever.") Simply put, there is no reasonable way for users or
developers to make a **one-time** payment, and have their data live on the
blockchain until the sun burns out. Whether you think about it from an economic
standpoint, through a sustainability lens, or just are concerned about making
the network operate in a healthy manner: there _has_ to be some mechanism to
"prune" stale entries on the blockchain.

Soroban is tackling this problem head-on with it's novel system of "rent" for
ledger entries. Each ledger entry gets to live on the blockchain as long as it
has a sufficient rent balance. Even if it does run out of rent, the persistent
data entry can be restored! This means the ledger data acts _as if_ it is stored
in the ledger forever, without requiring nearly as much long-term overhead to
keep the network operating. Of course, if you don't want your ledger entries to
expire, you can "bump" them at any time to keep the rent paid up.

Soroban has three types of Storage: `Temporary`, `Persistent`, and `Instance`.

#### Temporary Storage

- This is the cheapest of all storage types.
- An unlimited amount of temporary storage is available to each contract.
- It's designed for data that may only need to exist for a limited time. (e.g.,
  price oracles, signatures, etc.)
- The default "lifetime" for temporary storage is **16 ledgers** (with an
  estimated 5 seconds per ledger, that lifetime is roughly 80 seconds).
- The entry will expire once the lifetime ends (unless a `bump` has been invoked
  on it). Once expired, the the ledger entry is **permanently** deleted.
- Later on, the entry _can_ be re-created, but it _cannot_ be restored.

#### Persistent Storage

- This is the most expensive storage type (same price as `Instance`).
- An unlimited amount of persistent storage is available to each contract.
- It's designed for data that is unique among each contract user, and is not
  suitable to store temporarily. (e.g., user balances)
- The default "lifetime" for persistent storage is **86400 ledgers** (with an
  estimated 5 seconds per ledger, that lifetime is roughly 5 days)
- The entry will expire once the lifetime ends (unless a `bump` has been invoked
  on it). Once expired (which can happen without the contract instance
  expiring), the ledger entry becomes inaccessible to the network.
- Later on, the entry _cannot_ be re-created, but it _can_ be restored (this
  prevents over-writing a restorable ledger entry with different data).

#### Instance Storage

- This is the most expensive storage type (same price as `Persistent`).
- A limited amount of instance storage is available to each contract.
- It's designed for "global" data that is common and shared across all
  invocations of the contract. (e.g., admin accounts, contract metadata, etc.)
- Shares the "lifetime" of the contract instance. If the contract instance has
  not yet expired, the instance storage data is guaranteed to be not expired.
  (Contract instances use the same default "lifetime" as `Persistent`).
- The entry will expire only if the contract instance expires (unless a `bump`
  has been invoked on the contract instance itself.) Once the contract instance
  expires, the instance storage expires alongside it, and both become
  inaccessible to the network.
- Later on, the contract instance _cannot_ be re-created, but it _can_ be
  restored (this prevents over-writing a restorable contract with different
  data). If a contract is restored, its instance storage is restored as well.

### Storing Data

Soroban uses the `Env.storage().storage_type().set()` function (where
`storage_type` is one of `temporary`, `persistent`, or `instance`) to store data
in a contract's ledger entries. As a general rule, `Temporary` storage should
only be used for data that can be easily recreated or is only valid for a period
of time, where `Persistent` or `Instance` storage should be used for data that
can not be recreated and should kept permanently, such as a user's token
balance.

You can think of these ledger entries as key-value storage that can only be
accessed through the contract that owns it. You can construct a contract's
ledger entries in many different ways. They could be made up of very simple
elements like a symbol or number. Or, they can also be made from very complex
vectors or maps.

The `persistent` ledger entries for this quest will store a supplied `value`
alongside an `Address`. Using Soroban's `Address::require_auth()` function gives
us a simple method of authenticating a user. Only someone who could successfully
sign for an address (and, thus, invoke the contract from the address) is
permitted to store data in this contract as that address.

_Invoke the contract's `put()` function to store some data into the contract's
ledger entry for your quest account._

### Retrieving Data

To retrieve data from within a contract's ledger entries, the
`Env.storage().storage_type().set()` function (where `storage_type` is one of
`temporary`, `persistent`, or `instance`) is available to us. When called with a
`key` that corresponds to data the contract has previously stored, we get the
`value` stored alongside it in return.

The contract's `get()` function will retrieve stored data associated with any
address. When supplied with an `Address` as an argument, this function will
search for stored data corresponding to that address.

_Invoke the contract's `get()` function to retrieve contract data associated
with your quest account._

### Bumping Lifetimes

For every storage entry, there is the potential to `bump()` the entry's lifetime
to expire at some time further in the future. The Stellar network has lifetime
values for storage types that are configurable (by means of a validator vote).
The current set of values for these lifetime settings are listed in the
following table. The values are listed in "number of ledgers" and a typical
ledger will settle in an estimated 5 seconds.

| Storage Type | Minimum Lifetime | Maximum Lifetime    | Default Lifetime |
| ------------ | ---------------- | ------------------- | ---------------- |
| Temporary    | 16 (~80 seconds) | 6,312,000 (~1 year) | 16 (~80 seconds) |
| Persistent   | 4,096 (~5 hours) | 6,312,000 (~1 year) | 86,400 (~5 days) |
| Instance     | 4,096 (~5 hours) | 6,312,000 (~1 year) | 86,400 (~5 days) |

Because the `temporary` storage type has such a short default lifetime, our
contract code immediately `bump()`s the storage entry to the maximum lifetime
when it is created. This is one technique to bump ledger entries, but certainly
not the only method. You can also utilize a [`BumpFootprintExpirationOp`][bump]
operation inside a Stellar transaction to accomplish the same thing. The
`soroban` CLI also has a command to help facilitate these bump transactions:

```bash
soroban contract bump \
    --id  CARCWZOD26AJQ42VRJ3UYC3MJNGJV5UHO4VFHV5FWLVIKDCJ4CZOJXII \
    --key KeySymbol \
    --durability temporary \
    --ledgers-to-expire 100
```

When you specify a new lifetime when bumping a ledger entry, it ensures that the
expiration is _at least_ that many ledgers in the future (from the time of
invocation). For example, if a ledger entry is bumped by 100 ledgers and the
current lifetime is 50, the lifetime will be extended to 100. If a ledger entry
is bumped by 100 ledgers and the current lifetime is 150, the lifetime will not
be extended.

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
[auth-example]: https://soroban.stellar.org/docs/basic-tutorials/auth
[persist-data]: https://soroban.stellar.org/docs/fundamentals-and-concepts/persisting-data
[state-expiration]: https://soroban.stellar.org/docs/fundamentals-and-concepts/state-expiration
[bump]: https://soroban.stellar.org/docs/fundamentals-and-concepts/state-expiration#BumpFootprintExpirationOp
