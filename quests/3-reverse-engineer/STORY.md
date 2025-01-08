# Quest 3 - Reverse Engineer

## TL;DR

You again!? You're back here looking for the quick task? Well, alright, if you
think you're really ready for it. Good luck!

**In this quest, you will do something?? ¯\\_(ツ)_/¯**

## YET ANOTHER TESTNET RESET

- Public Key: GAEURQTDWOQU2G6GYUUH6IPVBOE5TLJJC2R3LZ5DZN6G5XFFRVMAIKOM
- Secret Key: SCR3YDW2IQ767HW4DTDYFJFKXF4DYGZ7HNCZTLZRYUUROB4SGKUJYEJI
- Salt: 2f5f8ead3a088198014cc0c46b3155cb4ae29389febac22f9c7c73ba4ea309a1
- Contract: CBUQDEUHDGFNN5NSR46KC42JI57AKXJPCKGVLWPCJFWFJWGLA6B7TIJ5

## STABLE RELEASE UPDATE: 2024-01-03

I'm deploying the contract to the following, "fresh" account:

- Public Key: GAEURQTDWOQU2G6GYUUH6IPVBOE5TLJJC2R3LZ5DZN6G5XFFRVMAIKOM
- Secret Key: SCR3YDW2IQ767HW4DTDYFJFKXF4DYGZ7HNCZTLZRYUUROB4SGKUJYEJI
- Contract: CCZQFPOYWX5PJFPXQ5RRAUN7UK56FLYWF6RHGQXY5LCI54VKV6UAGKWR

## PREVIEW 11 UPDATE: 2023-10-05

I'm deploying the contract to the following, "fresh" account:

- Public Key: GDJPW32ZTVL7XGYNJPODYSWEQV6YPZ35TGXN5C2NWIO3VC3ROQVIZAP2
- Secret Key: SCR3CMSISGYCUMPYHTWIJKPAAEJ3UUOJIQF42QOCYO74Y4BBCS3MZRYV
- Contract: CD7AFGUHXDBZS4XUQSXUILDJMWVM2ZNS7TX6ECYGIH2QMO7YVDS4DBHK

## UPDATE: 2023-04-25

I'm deploying the contract to the following, "fresh" account:

- Public Key: GB5AM4XMNNFVPTAANNKMYSUIYTAYFXVLEFB7EQ46GM2T23YTJFLV6SFY
- Secret Key: SBTCCCFEIWKAVZBZ7GH2VR2OO5GRWRAK7MFRYDUYLBTRCE6IXNA3GCJH
- Contract ID: 0c9c6e60f2ccaf84d0014e6b19f45532397828e3ccb577acc3d291e0036f20b6

## FINAL NOTE: DEPLOYING THIS CONTRACT ONE LAST TIME

In the course of all my edits and writing, I've _completely_ lost track of which
account has which contract deployed, and at which address... So, today,
2022-11-16 at 12:30pm (UTC-6), I'm deploying the contract one final time. Here's
the information I'm using:

- Public Key: GAC7HE4PQWI7H34JQN6QLQ7Y7NTAUZDWJZ4SJ6GXVC4K2DN7N65K5NLI
- Secret Key: SBOBDYVFY4F2TLYCB7GCYQVF4XPMFJPTEACRP56XSNCRIEP2UCHZP3JU
- Contract ID: 4cd6c3e267ca2b59e1c26f6850c17e1a91583e610582ffee8cebc8371e94573a

I've changed the corresponding lines in the README, cheatsheet, and check files.

### Account for the Examples

This one has deployed/invoked the `HelloWorld` contract

- Public Key: GBYCYGFSZY4MYVLAUJHIBWBE6GVS4FLS4PRV3R3OUXNR3PDOPAGAGPGK
- Secret Key: SDPWBXE6PWPQRLDR2QNKT4OEPD332IZOGZCQOEN7A3PPY76G7OMAY53T
- Contract ID: 6d4e8c57473219b0c5574a151c8944d6e669708de047947aa1fe5fb658ca2243

## Pushing this contract to Futurenet

Use the commands in `cheatsheat.sh` to push this contract to the futurenet.
The contract will be posted to the account `GCUXCUSJIUZCAIDVC4NLWW7AWSKKHBKM5E2PTUYYFU6HU6EUSH64BSQ3`.
This account is already funded (at the time of writing... else just use friendbot...)
The secret is as `SA6A6EEKT3CVZ74N5657Y3O64QDDXMTU2AXHUDP2W4XQKOMV2FMZYGWQ`.
Given the salt of 32x '0' the contract id should resolve to `a8793103223e3370df87718a30301fcd73b18621b97542861ffc3ae05efe8d0f`

> I might have messed this account up, by farting around with it while writing
> the README. I'm really sorry about that. \
> ElliotFriend

## Pushing My contract to Futurenet

- Use the commands in `cheatsheat.sh` to push this contract to the futurenet.
- The contract will be posted to the account
  `GAW6OM4TO5X65JAPSUWDLV6VJWRTZJ55LO7GRL5YUBJ7RLNBEIIAPVST`.
- This account is already funded (at the time of writing... else just use
  friendbot...)
- The secret is as `SBTS2ICL5TXEMID62V2JUALMNIVENFYIJEZZZZPTRTL6EUMJUCBQ3C2E`.
- Given the salt of 32x '0' the contract id should resolve to
  `81d67fb5e3fe77e8ad54de2e4dea2dbc0bd1c66fdd1cf35f4410808bcecff1a8`

### How I did it

#### Compile the contract

```bash
cd quests/3-reverse-engineer
cargo build --target wasm32-unknown-unknown --release
```

#### Deploy it

```bash
cd ../..
stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/soroban_reverse_engineer_contract.wasm
```

Returns the contract id: `81d67fb5e3fe77e8ad54de2e4dea2dbc0bd1c66fdd1cf35f4410808bcecff1a8`

#### Find the contract upload operation from horizon

- Go to lab's futurenet "endpoint explorer"
- Click on Operations -> for account -> enter my pubkey (above), select "desc"
- Look for an operation that's invoking the
  `HostFunctionHostFnCreateContractWithSourceAccount` function

#### Couple options from there

##### Option 1: Use the footprint

Copy the footprint, in this case it was:

```text
AAAAAAAAAAEAAAAGgdZ/teP+d+itVN4uTeotvAvRxm/dHPNfRBCAi87P8agAAAADAAAAAw==
```

###### Method A: Laboratory

Go to the "View XDR" page in lab, paste the footprint, select `LedgerFootprint`
for the "XDR type."

This shows the `contractId` as `gdZ/teP+d+itVN4uTeotvAvRxm/dHPNfRBCAi87P8ag=`

Pop that into runkit to decode the buffer as Hex, and it looks like this:

```js
Buffer.from('gdZ/teP+d+itVN4uTeotvAvRxm/dHPNfRBCAi87P8ag=', 'base64').toString('hex')
// output: 81d67fb5e3fe77e8ad54de2e4dea2dbc0bd1c66fdd1cf35f4410808bcecff1a8
```

###### Method B: `stellar-cli`

You've copied the footprint, and you can use Stellar CLI to decode the xdr and
retrieve the contract id:

```bash
echo "AAAAAAAAAAEAAAAGgdZ/teP+d+itVN4uTeotvAvRxm/dHPNfRBCAi87P8agAAAADAAAAAw==" | stellar xdr decode --type LedgerFootprint --output json
```

output looks like this:

```json
{
  "ledgerFootprint": {
    "readOnly": [],
    "readWrite": [
      {
        "contractData": {
          "contractId": "81d67fb5e3fe77e8ad54de2e4dea2dbc0bd1c66fdd1cf35f4410808bcecff1a8",
          "key": {
            "static": "ledgerKeyContractCode"
          }
        }
      }
    ]
  }
}
```

##### Option 2: Use the transaction's result-meta

Go to the transaction view in the endpoint explorer for the transaction you
found earlier.

Click the entry for "result_xdr" and you'll be taken to the xdr viewer, with the
transaction result parsed for you. There, we see the base64-encoded `contractId`
that we can turn into hex just like we did above using runkit.

Alternatively, you could copy/paste that `result_xdr` and decode it using the
stellar-cli:

```bash
echo "AAAAAAAAAGQAAAAAAAAAAQAAAAAAAAAYAAAAAAAAAAQAAAABAAAABAAAACCB1n+14/536K1U3i5N6i28C9HGb90c819EEICLzs/xqAAAAAA=" | stellar xdr decode --type TransactionResult --output json
```

and the output looks like this:

```json
{
  "transactionResult": {
    "feeCharged": 100,
    "result": {
      "txSuccess": [
        {
          "opInner": {
            "invokeHostFunction": {
              "success": {
                "object": {
                  "bytes": "81d67fb5e3fe77e8ad54de2e4dea2dbc0bd1c66fdd1cf35f4410808bcecff1a8"
                }
              }
            }
          }
        }
      ]
    },
    "ext": "v0"
  }
}
```

The same thing can be done using the `result_meta_xdr`, as well. This yields all
kinds of interesting output, but something like this will be shown:

```json
"contractData": {
  "contractId": "81d67fb5e3fe77e8ad54de2e4dea2dbc0bd1c66fdd1cf35f4410808bcecff1a8",
    "key": {
        "static": "ledgerKeyContractCode"
    },
    "val": {
        "object": {
        "contractCode": {
            "wasm": "0061736d0100000001090260017e017e600000030302000105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b073205066d656d6f72790200067375626d69740000015f00010a5f5f646174615f656e6403010b5f5f686561705f6261736503020a2902240002402000420f834209510d0000000b42154225200042d9ad9bbbb397bab6a67f511b0b02000b001e11636f6e7472616374656e766d6574617630000000000000000000000017003b0e636f6e747261637473706563763000000000000000067375626d6974000000000001000000067365637265740000000000060000000100000005"
        }
        }
  }
}
```

This gives us both the `contractId`, as well as the actual contract code that
was deployed! It's in hex format, so you'd have to do something like this to get
it into a usable wasm file:

```bash
echo "0061736d0100000001090260017e017e600000030302000105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b073205066d656d6f72790200067375626d69740000015f00010a5f5f646174615f656e6403010b5f5f686561705f6261736503020a2902240002402000420f834209510d0000000b42154225200042d9ad9bbbb397bab6a67f511b0b02000b001e11636f6e7472616374656e766d6574617630000000000000000000000017003b0e636f6e747261637473706563763000000000000000067375626d6974000000000001000000067365637265740000000000060000000100000005" | xxd -r -p - > output.wasm
```

Then you'd be able to run `stellar gen` to get the details of what's inside the
contract code that lives on chain:

```bash
stellar contract inspect --wasm output.wasm --output json
```

output like this:

```json
{
  "type": "function",
  "name": "submit",
  "inputs": [
    {
      "name": "secret",
      "value": {
        "type": "symbol"
      }
    }
  ],
  "outputs": [
    {
      "type": "bool"
    }
  ]
}
```