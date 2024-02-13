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
  contract. Create and fund this Testnet account on your own.

**For our finale quest, you must build and deploy the `AllowanceContract` using
your Quest Account (`Parent_Account`). Next grant your deployed contract an
allowance enabling it to transfer XLM from the parent to the child. Then, using
either account, you must withdraw an allowance to the `Child_Account` using the
contract deployed by the `Parent_Account`.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Stellar Assets as Soroban Tokens](#stellar-assets-as-soroban-tokens)
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

### Stellar Assets as Soroban Tokens

One of the defining characteristics of the Stellar network is that assets are
first-class citizens. They are easy to create, cheap to use/transfer/trade, and
have many incredible use-cases. There also exists an extensive set of
authorization tools that asset issuers can use to control who can acquire, use,
or retain those assets. We won't spend _too_ much time here, because you may
already be up-to-speed here (and if you aren't, you can still make it through
today's quest). If you need a refresher the [developer
documentation][docs-assets] and our own [Stellar Quest][sq-learn] course both
have **loads** of information about assets. For now, just remember that the
`native` asset on Stellar is the [Lumen][lumens]. It's identified using the
asset code `XLM`.

As Soroban was designed and developed, one of the _key_ requirements was that
assets issued on Stellar can be used and incorporated into Soroban. It's even
[one of the FAQs][assets-faq]! This interoperability is facilitated by using the
interface provided by [the Stellar Asset Contract](#the-stellar-asset-contract).
(Note tokens/assets created using a custom Soroban contract cannot be exported
to a Stellar asset trustline.)

### The Stellar Asset Contract

On of the big priorities in the process of Soroban development involved an
effort to decide what a "Standardized Asset" looks like in a smart contract
context. These decisions, and related discussions, are recorded in
[CAP-46-6][cap-46-6]. If you're familiar with Ethereum, this proposal tries to
follow an ERC-20 model, where applicable.

The [Stellar Asset Contract][asset-contract] is an implementation of the
CAP-46-6 proposal. It can be used to enable smart contract interactions for an
existing Stellar asset. _Every_ Stellar asset has a contract reserved for it,
which anybody can deploy. For today, we'll be using it with native Lumens (an
asset that has already had its smart contract enabled). This Stellar Asset
Contract implements a [token interface][token-interface] that is quite
feature-full. The most notable function you'll need from it today is `approve`,
which will allow us increase the amount of some token that one address can spend
_from_ another address. It's like spending money directly from your parent's
bank account (everyone's dream).

### Yeah, but How do I Use That Asset Contract?

> It should be noted that a Soroban token developer can choose to implement any
> token interface they choose. There is no _requirement_ to implement everything
> from CAP-46-6, but doing so does allow a token to interoperate more easily
> with other tokens which _are_ compliant with CAP-46-6. You can learn more
> about the [suggested token interface][sac-interface] in the Soroban docs.

So, how do we actually make one of them tokens, then? There are a few methods
available to us. Let's (briefly) look at them.

1. (Spicy 🌶️🌶️🌶️) You _could_ write the whole thing from scratch,
   implementing whatever features, functions, and fun suit your needs. That
   would take a lot of work, but you could do it. I won't stop you.

2. (Medium 🌶️🌶️) There's a `create-token.py` script in the `py-scripts/`
   directory here that will do a lot of the heavy lifting for you. This can be
   used and adapted to match whatever asset you're trying to create. It's a
   fantastic starting point.

3. (Mild 🌶️) Fun fact, the `soroban` CLI has a handy little helper command
   built right into it that will (we promise, we're not making this up) do
   _everything_ for you! You don't have to code anything, just run the command a
   single time, and the contract is **deployed**. You could use it like this:

```bash
soroban contract asset deploy --asset QUEST6:GAS4VPQ22OBEAEWBZZIO2ENPGPZEOPJ4JBSN6F7BIQQDGAHUXY7XJAR2
# CDCPEACOOZULMT6GGHK44TP6DPF4VUXKBM6B5DBQNIRQBMRXWJZYODGD

# It even works with the `native` asset!
soroban contract asset deploy --asset native
```

It should be noted that enabling an asset's smart contract will work exactly one
time per asset (per network). The `native` asset contract is already enabled on
the Testnet, and trying to enable it again (on Testnet) will return an error
rather than a `contract_address`. (Caveat: since even SAC contract instances are
subject to state archival, you _may_ need to re-enable the native token if
Soroban gives you a `contract not found` error. We've extended the TTL of the
native XLM contract instance to its maximum lifetime, however. So, this
"shouldn't" be a problem for anyone.)

> It should _also_ be noted you don't need to deploy or enable any smart
> contract tokens or Stellar assets for this quest. We just put this here for
> fun!

### Native XLM on Soroban

Speaking of the `native` asset: One of the cool things about the Stellar Asset
Contract is that even the native XLM token utilizes it! To use it, we just need
to figure out the `contract_address` we should invoke. That can be done easily
enough with one of the Stellar SDKs (below, we're using Python):

```python
import hashlib
from stellar_sdk import Asset, Network, StrKey, xdr

# This will work using either native or issued assets
native_asset = Asset.native()
issued_asset = Asset("QUEST6", "GAS4VPQ22OBEAEWBZZIO2ENPGPZEOPJ4JBSN6F7BIQQDGAHUXY7XJAR2")

network_id_hash = xdr.Hash(hashlib.sha256(Network.TESTNET_NETWORK_PASSPHRASE.encode()).digest())
preimage = xdr.HashIDPreimage.from_envelope_type_contract_id(
    contract_id=xdr.HashIDPreimageContractID(
        network_id=network_id_hash,
        contract_id_preimage=xdr.ContractIDPreimage.from_contract_id_preimage_from_asset(
           from_asset=native_asset.to_xdr_object()
        ),
    ),
)
contract_address = StrKey.encode_contract(
    xdr.Hash(hashlib.sha256(preimage.to_xdr_bytes()).digest()).hash.hex()
)
print(f"Contract Address: {contract_address}")
```

You can find an expanded version of the above script, as well as some other
_very_ handy Python scripts (big shoutout to [Jun Luo (@overcat)][overcat]) in
the `py-scripts/` directory. They deal with all kinds of Soroban tasks: creating
tokens, payments, finding asset contract addresses, deploying contracts, etc.

As per our [tl;dr](#tldr) at the top, this native asset contract will _need_ to
be invoked only once: The `Parent_Account` will need to `approve` the
`AllowanceContract` as a proxy spender. You could also make use of the `balance`
and/or `allowance` functions of the contract to check your work along the way.

Don't forget to look into the [Token Interface][sac-interface] to figure out
which arguments you'll need to use when making those invocations. You remember
how to format those arguments, don't you? What!? You don't?! Ok, ok, ok. It's
gonna be fine. You can check back in [Quest 4](../4-cross-contract/README.md)
and [Quest 5](../5-custom-types/README.md) for a recap.

### Back to Your Quest

Ok, so we've gone through a bunch of theory, and looked at how assets can (or
cannot) be interacted with in Soroban. Now, it's time to let you go and bring
this thing home!

If you forgot what your task is, here it is again:

- [ ] Deploy the `AllowanceContract` as the `Parent_Account`
- [ ] Invoke the `init` function of the `AllowanceContract`
- [ ] Use the `approve` function of the native token contract to allow your
  `AllowanceContract` to make proxy transfers from the `Parent_Account` to the
  `Child_Account` (you'll need to specify an expiration ledger for this
  allowance, check back to quest 2 for a refresher on calculating ledgers)
- [ ] Invoke the `withdraw` function of the `AllowanceContract` using either the
  `Child_Account` or `Parent_Account`

> _Note:_ We _could_ have programmed the invocation of the XLM's `approve`
> function right into the contract's `init` function. That would be a pretty
> clean way to accomplish the job. However, we're trying to help you get used to
> interacting with contracts and making invocations to the XLM SAC is a really
> useful skill to learn.

While performing the above steps, you'll want to consider the amount of XLM
you're using along the way. In Soroban, assets are quantified using
[Stroop][stroop]s (that is, one ten-millionth of the asset). For example, if you
want to `transfer` 1 XLM, you'll need to supply `10000000`, `10_000_000` or `1 *
10**7` stroops as an argument in your invocation.

Additionally, the astute observer might notice an interesting disparity between
the Parent's asset balance and the approved allowance the contract has access to
at any given time. For example you could have a balance of  100,000 XLM in the
Parent account, but only `approve` a "first tranche" to the contract of 10,000
XLM. Then the contract - depending on the `init` arguments passed - might
`withdraw` 5,000 XLM during each successful invocation. The contract will only
ever be able to proxy _from_ the parent _to_ the child as determined by the
contract settings, but this flexibility allows the parent to more safely and
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

- **[Core Advancement Proposal 46-6][cap-46-6]** contains more information
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
[asset-contract]: https://soroban.stellar.org/docs/tutorials/stellar-asset-contract
[token-interface]: https://soroban.stellar.org/docs/tokens/token-interface
[sac-interface]: https://soroban.stellar.org/docs/tokens/token-interface#code
[cap-46-6]: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046-06.md
[docs-assets]: https://developers.stellar.org/docs/fundamentals-and-concepts/stellar-data-structures/assets
[assets-faq]: https://soroban.stellar.org/docs/faq#can-soroban-contracts-interact-with-stellar-assets
[lumens]: https://developers.stellar.org/docs/fundamentals-and-concepts/lumens
[overcat]: https://github.com/overcat
[stroop]: https://developers.stellar.org/docs/glossary#stroop
[timelock]: https://soroban.stellar.org/docs/tutorials/timelock
[single-offer]: https://soroban.stellar.org/docs/tutorials/single-offer-sale
[liquidity-pool]: https://soroban.stellar.org/docs/tutorials/liquidity-pool
[sq-learn]: https://quest.stellar.org/learn
[sdk-ledger]: https://docs.rs/soroban-sdk/latest/soroban_sdk/ledger/struct.Ledger.html
