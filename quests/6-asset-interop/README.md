# Quest 6 - Asset Interop <!-- omit in toc -->

## TL;DR

Big day, huh!? Final Quest in our inaugural series of Soroban Quest! This has
been so awesome! And there's only more greatness to come, so stay tuned!

Today's Quest is a banger! It will not only challenge you, but will also show
you some of the **amazing** stuff that's possible in this brave new
Soroban-ified world! Now, it's a doozy so you're going to _really want_ to read
through this document. But, here's the short-n-sweet instructions, if you want
to jump ahead and muck things up.

There are two relevant accounts today:

- `Parent_Account` will be your Quest Account, (what you are given when you run
  `sq play {n}`) and will be used to deploy an `AllowanceContract`.
- `Child_Account` will be a secondary account which will interact with your
  contract. Create and fund this Futurenet account on your own.

**For our finale quest, you must build and deploy the `AllowanceContract` using
your Quest Account (`Parent_Account`). Next grant your deployed contract an
allowance enabling it to transfer XLM from the parent to the child. Then, using
either account, you must withdraw an allowance to the `Child_Account` using the
contract deployed by the `Parent_Account`.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - ["Classic" Stellar Assets as Soroban Tokens](#classic-stellar-assets-as-soroban-tokens)
  - [The Stellar Asset Contract](#the-stellar-asset-contract)
  - [Yeah, but How do I Use That Asset Contract?](#yeah-but-how-do-i-use-that-asset-contract)
  - [Native XLM on Soroban](#native-xlm-on-soroban)
  - [Back to Your Quest](#back-to-your-quest)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quests, or you just need a refresher, we have
some (pretty extensive) instructions for the _mechanics_ of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need those
instructions again.

## The Task at Hand

For a moment, put yourself in the position of a parent, sending their child out
into the world to face the challenges of college and/or the workforce. I know,
it's emotional. That's ok. You don't want to send them off with nothing. You
want to give them some kind of confidence they won't have an empty stomach at
the end of the day, no matter what happens.

So, you're setting up an allowance contract for them. You will deploy a contract
that will require a one-time action on your part, but will make available to
them a steady trickle of resources should they ever have need. This approach
gives us two very powerful wins: _they_ can't withdraw everything at once, but
_you_ don't have to remember to make transfers all the time!

Back in reality, we're ready to talk assets! But, first, we _have_ to say it one
last time: **Read the code!** This contract is relatively complex. We won't make
you fiddle with it today, but there's no better way to understand what's
happening than to actually _read_ it. Seriously.

### "Classic" Stellar Assets as Soroban Tokens

> _Please Note_: If you've forgotten, Soroban is still under active development,
> design, and discussion. Significant changes can happen and should even be
> expected. The area of asset interoperability between "Classic" Stellar and
> Soroban is one such area that is under active consideration. We have designed
> this quest to be as up-to-date as possible at the time of writing, but the
> conventions, steps, terminology, architecture, etc. used in today's quest are
> subject to change in the future.
>
> _Revision Date: 2023-06-01_

One of the defining characteristics of the "Classic" Stellar network is that
assets are first-class citizens. They are easy to create, cheap to
use/transfer/trade, and have many incredible use-cases. There also exists an
extensive set of authorization tools that asset issuers can use to control who
can acquire, use, or retain those assets. We won't spend _too_ much time here,
because you may already be up-to-speed here (and if you aren't, you can still
make it through today's quest). If you need a refresher the [developer
documentation][docs-assets] and our own [Stellar Quest][sq-learn] course both
have **loads** of information about assets. For now, just remember that the
`native` asset on Stellar is the [Lumen][lumens]. It's identified using the
asset code `XLM`.

As Soroban development continues, one of the _key_ requirements is that assets
issued on "Classic" Stellar can be used and incorporated into Soroban. It's even
[one of the FAQs][assets-faq]! This interoperability is facilitated by using the
interface provided by [the Stellar Asset Contract](#the-stellar-asset-contract).
(Note assets minted in Soroban cannot be exported to a "Classic" Stellar asset.)

### The Stellar Asset Contract

Soroban development regarding assets involves an effort to decide what a
"Standardized Asset" looks like in a smart contract context. These decisions,
and related discussions, are recorded in [CAP-0046-06][cap-46-6]. If you're
familiar with Ethereum, this proposal tries to follow an ERC-20 model, where
applicable.

The [Stellar Asset Contract][asset-contract] is an implementation of the
CAP-46-6 proposal. It can be used to to create a new token on Soroban, or to
interact with a "Classic" asset that has been "wrapped" for use in Soroban. For
today, we'll be using it with native Lumens that have been wrapped. This Stellar
Asset Contract implements a [token interface][token-interface] that is quite
feature-full. The most notable function you'll need from it today is
`increase_allowance`, which will increase the amount of some token that one
address can spend _from_ another address. It's like spending money from your
parent's bank account.

### Yeah, but How do I Use That Asset Contract?

> It should be noted that a Soroban token developer can choose to implement any
> token interface they choose. There is no _requirement_ to implement everything
> from CAP-46-6, but doing so does allows a token to interoperate more easily
> with other tokens which _are_ compliant with CAP-46-6. You can learn more
> about the [suggested token interface][sac-interface] in the Soroban docs.

So, how do we actually make one of them tokens, then? There are a few methods
available to us. Let's (briefly) look at them.

1. (Spicy 🌶️🌶️🌶️) You _could_ write the whole thing from scratch, implementing
   whatever features, functions, and fun suit your needs. That would take a lot
   of work, but you could do it. I won't stop you.

2. (Medium 🌶️🌶️) There's a `create-token.py` script in the `py-scripts/`
   directory here that will do a lot of the heavy lifting for you. This can be
   used and adapted to match whatever asset you're trying to create. It's a
   fantastic starting point.

3. (Mild 🌶️) Fun fact, the `soroban` CLI has a handy little helper command built
   right into it that will (we promise, we're not making this up) do
   _everything_ for you! You don't have to code anything, just run the command a
   single time, and the contract is **deployed**. You could use it like this:

```bash
soroban lab token wrap --asset QUEST6:GAS4VPQ22OBEAEWBZZIO2ENPGPZEOPJ4JBSN6F7BIQQDGAHUXY7XJAR2
# 36d479817b7c64e765f084c121640ee8de62db22a2b37e0b40c5b08e09b63f59

# It even works with the `native` asset!
soroban lab token wrap --asset native
```

It should be noted that wrapping an asset will work exactly one time per asset
(per network). The `native` asset contract is already deployed to the Futurenet,
and trying to wrap that again (on the Futurenet) will return an error rather
than a `contract_id`.

> It should _also_ be noted you don't need to deploy or wrap any tokens or
> assets for this quest. We just put this here for fun!

### Native XLM on Soroban

Speaking of the `native` asset: One of the cool things about the Stellar Asset
Contract is that even the native XLM token utilizes it! To use it, we just need
to figure out the `contract_id` we should invoke. That can be done easily enough
with one of the Stellar SDKs (below, we're using Python):

```python
import hashlib
from stellar_sdk import Asset, Network, xdr

# This will work using either native or issued assets
native_asset = Asset.native()
issued_asset = Asset("QUEST6", "GAS4VPQ22OBEAEWBZZIO2ENPGPZEOPJ4JBSN6F7BIQQDGAHUXY7XJAR2")

network_id_hash = xdr.Hash(Network(Network.FUTURENET_NETWORK_PASSPHRASE).network_id())
data = xdr.HashIDPreimage(
    xdr.EnvelopeType.ENVELOPE_TYPE_CONTRACT_ID_FROM_ASSET,
    from_asset=xdr.HashIDPreimageFromAsset(
        network_id=network_id_hash, asset=native_asset.to_xdr_object()
    ),
)
contract_id = hashlib.sha256(data.to_xdr_bytes()).hexdigest()
print(f"Contract ID: {contract_id}")
```

You can find an expanded version of the above script, as well as some other
_very_ handy Python scripts (big shoutout to [Jun Luo (@overcat)][overcat]) in
the `py-scripts/` directory. They deal with all kinds of Soroban tasks: creating
tokens, pyaments, finding asset contract IDs, deploying contracts, etc.

As per our [tl;dr](#tldr) at the top, this native asset contract will _need_ to
be invoked only once: The `Parent_Account` will need to `increase_allowance` to
establish the `AllowanceContract` as a proxy spender. You could also make use of
the `balance` and `allowance` functions of the contract to check your work along
the way.

Don't forget to look into the [Token Interface][sac-interface] to figure out
which arguments you'll need to use when making those invocations. You remember
how to format those arguments, don't you? What!? You don't?! Ok, ok, ok. It's
gonna be fine. You can check back in [Quest 4](../4-cross-contract/README.md)
and [Quest 5](../5-custom-types/README.md) for a recap.

<sup><sub><sup><sub><sup><sub>
or poke around in here some more
</sup></sub></sup></sub></sup></sub>

### Back to Your Quest

Ok, so we've gone through a bunch of theory, and looked at how assets can (or
cannot) be interacted with in Soroban. Now, it's time to let you go and bring
this thing home!

If you forgot what your task is, here it is again:

- [ ] Deploy the `AllowanceContract` as the `Parent_Account`
- [ ] Invoke the `init` function of the `AllowanceContract`
- [ ] Use the `increase_allowance` function of the native token contract to
  allow your `AllowanceContract` to make proxy transfers from the
  `Parent_Account` to the `Child_Account`
- [ ] Invoke the `withdraw` function of the `AllowanceContract` using either the
  `Child_Account` or `Parent_Account`

While performing the above steps, you'll want to consider the amount of XLM
you're using along the way. In Soroban, most assets are quantified using
[Stroop][stroop]s (that is, one ten-millionth of the asset). For example, if you
want to `transfer` 1 XLM, you'll need to supply `10000000`, `10_000_000` or `1 *
10**7` stroops as an argument in your invocation.

Additionally, the astute observer might notice an interesting separation between
the Parent's asset balance and the approved allowance the contract has access to
at any given time. For example you could have a balance of  100,000 XLM in the
Parent account, but only `increase_allowance` a "first tranche" to the contract
of 10,000 XLM. Then the contract - depending on the `init` arguments passed -
might `withdraw` 5,000 XLM during each successful invocation. The contract will
only ever be able to proxy _from_ the parent _to_ the child as determined by the
contract arithmetic, but this flexibility allows the parent to more safely and
sensibly control the flow of funds. All the levers! You **are** the man behind
the curtain!

Finally, given this flexibility, great care should be taken when calling the
various invocations, as you don't want to enable a `withdraw` to take place that
would be greater than the contract's available allowance. Choose your numbers
wisely, my friend.

> If you're really confused about the units, digits, and numbers to use, read
> through the `src/test.rs` file for some inspiration and to see which numbers
> we used during development.

## Further Reading

- **[Core Advancement Proposal 0046-06][cap-46-6]** contains more information
  than you probably want about how the asset interoperability is intended to
  work. These "CAP" documents are excellent resources for discovering not only
  _how_ something on Stellar works, but also _why_ it is designed that way.
  Check it out some time.
- The **[Stellar Asset Contract][asset-contract]** article in the Soroban
  documentation is a probably less intimidating resource. It has so much more
  good stuff than we could even mention here. This is definitely one to read
  through.
- The **[Timelock][timelock]**, **[Single Offer Sale][single-offer]**, and
  **[Liquidity Pool][liquidity-pool]** example contracts are a great place to
  learn more about how assets on Soroban can interact with each other, and how
  they could be interacted with. These are great examples for a real world use
  of the concepts we've discussed today.
- **[Assets][docs-assets]** in Stellar are an enormous part of the network
  architecture. If you're unfamiliar with how assets work with "Classic"
  Stellar, than the Developer Documentation has all the information you'll need.
  Or, if you want to earn some more sweet badges while you learn, level 1 of
  [Stellar Quest][sq-learn] is exactly what you want! Lots of excellent
  knowledge about assets and payments there.
- Soroban doesn't know a _whole lot_ about the state of the Stellar network at
  execution time. But, it does know a few things, and those are presented to it
  as a `Ledger` data structure. There is pretty significant stuff to know, so
  here's the relevant **[documentation page][sdk-ledger]** all about it!

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[asset-contract]: https://soroban.stellar.org/docs/how-to-guides/stellar-asset-contract
[token-interface]: https://soroban.stellar.org/docs/reference/interfaces/token-interface
[sac-interface]: https://soroban.stellar.org/docs/reference/interfaces/token-interface#code
[cap-46-6]: https://stellar.org/protocol/cap-46-06
[docs-assets]: https://developers.stellar.org/docs/fundamentals-and-concepts/stellar-data-structures/assets
[assets-faq]: https://soroban.stellar.org/docs/learn/faq#can-soroban-contracts-interact-with-stellar-assets
[lumens]: https://developers.stellar.org/docs/fundamentals-and-concepts/lumens
[overcat]: https://github.com/overcat
[stroop]: https://developers.stellar.org/docs/glossary#stroop
[timelock]: https://soroban.stellar.org/docs/how-to-guides/timelock
[single-offer]: https://soroban.stellar.org/docs/how-to-guides/single-offer-sale
[liquidity-pool]: https://soroban.stellar.org/docs/how-to-guides/liquidity-pool
[sq-learn]: https://quest.stellar.org/learn
[sdk-ledger]: https://docs.rs/soroban-sdk/latest/soroban_sdk/ledger/struct.Ledger.html
