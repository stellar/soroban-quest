# Quest 5 - Custom Types <!-- omit in toc -->

## TL;DR

Ok, fine! I guess you _maybe_ kinda sorta have what it takes to get started with
just a short prompt. Fine! Be my guest! But don't forget: Knowledge abounds
further down in the README!

**ALSO**: We're going to flip things around on you today! You've gotten pretty
good at the deploy and invoke steps of the Soroban process (and, yes, you'll
have to do those today, as well). But, how are your Rust chops?? That's right,
today you're going to be _writing_ some actual Rust for your contract!

**For today's quest, you must create a set of custom types in your contract that
conform to the specifications listed below. You must also deploy that contract,
and then make the necessary invocations for each of the custom types. Finally,
you must invoke the `verify` function of our verification contract
`CDQRNT7GJ5F3X6KH23YTZZEIOVKI3VUSL3B6GKX4PKW2WVSIZETQORNG` so we can
double-check your custom type definitions.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Custom Types in Rust](#custom-types-in-rust)
    - [Rust `enum`s](#rust-enums)
    - [Rust `struct`s](#rust-structs)
  - [Custom Types in Soroban](#custom-types-in-soroban)
  - [Create Your Custom Types](#create-your-custom-types)
    - [Rectangle](#rectangle)
    - [Animal](#animal)
    - [User](#user)
    - [RGB](#rgb)
    - [Color](#color)
    - [Participant](#participant)
    - [RoyalCard](#royalcard)
  - [Invoke Your Contract](#invoke-your-contract)
  - [Let Us Check Your Work](#let-us-check-your-work)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quests, or you just need a refresher, we have
some (pretty extensive) instructions for the _mechanics_ of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need to use those
instructions again.

## The Task at Hand

This is a big moment! You feel prepared. You feel ready. Quite frankly, you
_are_ ready! You've got the gumption and the tenacity to tackle this quest!

But, again, please: **Read the code!!** There is important stuff that you need
to know inside of there. (Plus, we worked really hard on it, and you should
totally use it to the fullest extent!)

_Bonus:_ You've heard us say you should "read `lib.rs`" like a hundred times by
now. But today there's a fancy new `types.rs` file you should take a gander at.

### Custom Types in Rust

A custom type in the [Soroban Rust dialect][contract-dialect] is declared using
the `contracttype` attribute macro on either a `struct` or `enum` definition.
But, before we dive into the Soroban-specific information, let's camp out with
how this concept plays out in a standard Rust environment. Don't worry: this'll
be quick.

#### Rust `enum`s

In Rust, we can use an `enum` (short for `enumeration`) to define a type by
listing (or "enumerating") all of its possible variants. You could think of a
defined `enum` as a "menu" from which you choose one item. You won't necessarily
choose the same thing every time, but you'll choose only one when it is time to
make a selection. For example:

```rust
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    Skittles, // an important part of a balanced diet
}
```

The _Rust Book_ contains a [whole chapter on `enum`s][rust-enums], and it has a
lot more very valuable information that you can learn. Be sure to check it out!

#### Rust `struct`s

In Rust, a `struct` allows us to gather together and name multiple values that
are related in some way. Such a `struct` will be a custom data type that
represents a meaningful group of those values. Rather than a menu you choose
from, a `struct` is more like that "template" document you save so you can just
make a copy and fill out a few things when the time comes to start on your
homework. For example:

```rust
struct Homework {
    class: String,
    subject: String,
    studentId: u32,
    complete: bool,
    date: String,
}
```

We could then create a new _instance_ of a `Homework` assignment by doing
something like this:

```rust
let homework1 = Homework {
    class: String::from("AA429"),
    subject: String::from("Advanced Astrophotography"),
    student_id: 8675309,
    complete: false,
    date: String::from("Next Wednesday"),
};
```

The _Rust Book_ also has a [whole chapter on `struct`s][rust-struct], and it has
even more information! Check this one out, too!

### Custom Types in Soroban

If you choose to click only one link in this entire README, please make it this
one: The **[Custom Types][fc-ct]** article in the Soroban documentation is just
truly _very_ good. The custom types you can create on Soroban are made up of
`struct` types and `enum` types, though there are a few different conventions
used to define those types. The broad categories of custom types you can create
are:

- [`Struct` with Named Fields][struct-named]
- [`Struct` with Unnamed Fields][struct-unnamed]
- [`Enum` with Unit and Tuple Variants][enum-unit-tuple]
- [`Enum` with Integer Variants][enum-integer]

[struct-named]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types#structs-with-named-fields
[struct-unnamed]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types#structs-with-unnamed-fields
[enum-unit-tuple]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types#enum-unit-and-tuple-variants
[enum-integer]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types#enum-integer-variants

It's also important to understand that `enum`s are currently supported as
contract types in Soroban only when all variants have an explicit integer
literal, **or** when all variants are unit or single field variants.

In the **[Custom Types][fc-ct]** article you'll even learn quite a bit about how
Soroban will store your custom types on the Ledger, XDR conversion, JSON
representation, and more!

An additional (and very useful) resource in the Soroban documentation can be
found here: [Error Enums][error-enums]. This article describes how you might use
an `enum` to meaningfully convey error information from your contract. That
might seem vaguely familiar, if you remember having to frantically figure out
what error you were receiving (and why) during a previous quest.

### Create Your Custom Types

Ok, that was some **great** educational content, but we're back on track! For
this quest, you must create and then use in an invocation from the stellar-cli
the following custom types in your contract:

#### Rectangle

The `Rectangle` type must be a `struct`, with two fields: `width` and `height`
which both must be a `u32` value.

Invoke the `c_rect` function to create a `Rectangle` using something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_rect \
    --_rect '{"height":<a-u32-integer>,"width":<a-u32-integer>}'
```

#### Animal

The `Animal` type must be an `enum`, with at least two variations: `Cat` and
`Dog`.

Invoke the `c_animal` function to create an `Animal` using something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_animal \
    --_animal <an animal-variant>
```

#### User

The `User` type must be a `struct` with `name`, `age`, `pet`, and `food` fields,
corresponding to `Bytes`, `u32`, `Animal`, and `String` values, respectively.

Invoke the `c_user` function to create a `User` using something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_user \
    --_user '{"name":"<a-hex-encoded-string>","age":<a-u32-integer>,"pet":<an-animal-variant>,"food":"<a-string-can-be-as-long-as-you-want-it-to-be-yay-strings>"}'
```

The `String` type is a relatively new type to be implemented in Soroban, and it
has proven to be a very valuable addition. On the rust side of things, a Soroban `String`
is stored as a growable array of `u8`s. They're still quite easy to create, however:

```rust
use soroban_sdk::{Env, String};
let env = Env::default();
let msg = "a message";
let s = String::from_slice(&env, msg);
```

#### RGB

The `RGB` type must be a tuple `struct` type made with a tuple of 3 `u32`
values.

Invoke the `c_rgb` function to create a `RGB` value using something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_rgb \
    --_rgb [<a-u32-integer>,<a-u32-integer>,<a-u32-integer>]
```

#### Color

The `Color` type will combine the `RGB` custom type nested within a tuple `enum`
type. Construct your `RGB` struct type as described above, Then, your `Color`
enum type must be defined as a variant with a name of "RGB" and an instance of
your `RGB` type.

Invoke the `c_color` function to create a `Color` using something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_color \
    --_val '{"RGB":<a-rgb-object>}'
```

#### Participant

The `Participant` type must be an `enum` with single-value tuple variants as
follows:

- An "Account" variant with an `Address` type
- A "Contract" variant with an `Address` type

The `Address` type is a universal opaque identifier that can be useful for input
arguments, authentication, data keys, and more. It is _opaque_ in the sense that
while it might represent a Stellar account, or a Stellar contract, you can treat
it the same no matter what type of `Address` you're dealing with.

Invoke the `c_part` function to create an account `Participant` using something
like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_part \
    --_participant '{"Account":"<stellar-public-g-address>"}'
```

Also invoke the `c_part` function to create a contract `Participant` using
something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_part \
    --_participant '{"Contract":"<stellar-contract-c-address>"}'
```

#### RoyalCard

The `RoyalCard` type must be an `enum` containing three `u32` integer variations
as follows:

- A "Jack" variant, with a value of 11
- A "Queen" variant, with a value of 12
- A "King" variant, with a value of 13

Invoke the `c_card` function using something like:

```bash
stellar contract invoke \
    --id C... \
    -- \
    c_card \
    --_card <a-u32-integer>
```

### Invoke Your Contract

That was a lot of work, wasn't it! You should be really proud of yourself. I
know I am. Now that you have all your custom types written and deployed (oh
yeah, don't forget to deploy your contract!), you need to _invoke_ each of the
functions listed in `src/lib.rs` and pass a valid argument for your custom type.
There's some helpful hints on how to invoke these throughout this README.

In case you lost track, you must invoke the following functions providing an
argument of the custom type you created:

| Function   | Argument Name    | Argument Type                |
| ---------- | ---------------- | ---------------------------- |
| `c_rect`   | `--_rect`        | [Rectangle](#rectangle)      |
| `c_animal` | `--_animal`      | [Animal](#animal)            |
| `c_user`   | `--_user`        | [User](#user)                |
| `c_rgb`    | `--_rgb`         | [RGB](#rgb)                  |
| `c_color`  | `--_val`         | [Color](#color)              |
| `c_part`   | `--_participant` | [Participant](#participant)* |
| `c_card`   | `--_card`        | [RoyalCard](#royalcard)      |

> \* Don't forget the `Participant` type must be invoked twice, once as an
> `Account`, and once as a `Contract`.

### Let Us Check Your Work

Well done, you've customized all the types, you've invoked all the things, and
you're ready to claim your prize! Before we get on with it, just _one_ more
thing: **You need to invoke our verification contract.**

Using your Quest Keypair, you must invoke the `verify` function on the contract
with the ID `CDQRNT7GJ5F3X6KH23YTZZEIOVKI3VUSL3B6GKX4PKW2WVSIZETQORNG`,
supplying your own contract address as the `--contract_address` argument. We'll
double-check all your hard work, and make sure you've implemented the required
custom types with the necessary fields, variants, values, etc.

**Then**, you are free to use `sq check 5` to (try and) claim your prize!

## Further Reading

- Again, just trust me and **read this**: The [Custom Types][fc-ct] article in
  the Learn section of the Soroban docs could _not_ be more useful!
- You can look in the [SDK Docs][sdk-contracttype] to learn more about the
  `contracttype` attribute macro.
- There is an entire [Custom Types example contract][example-ct] you can look at
  and read through in the Soroban docs. It's great for inspiration, or to see
  how all these pieces can fit together.
- Read more about the [Contract Dialect][contract-dialect] of Rust used in
  Soroban in the documentation.

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[sdk-contracttype]: https://docs.rs/soroban-sdk/latest/soroban_sdk/attr.contracttype.html
[fc-ct]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types
[example-ct]: https://developers.stellar.org/docs/build/smart-contracts/example-contracts/custom-types
[rust-struct]: https://doc.rust-lang.org/book/ch05-00-structs.html
[rust-enums]: https://doc.rust-lang.org/book/ch06-00-enums.html
[error-enums]: https://developers.stellar.org/docs/learn/encyclopedia/errors-and-debugging/errors#error-enums
[contract-dialect]: https://developers.stellar.org/docs/learn/encyclopedia/contract-development/rust-dialect