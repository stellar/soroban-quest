"""
This example shows how to derive a contract ID for a particular Stellar asset,
and also how to convert a contract ID into a StrKey address representation.
"""
import hashlib

from stellar_sdk import Asset, Network, StrKey
from stellar_sdk import xdr as stellar_xdr

def get_asset_contract_id(asset: Asset, network_passphrase: str) -> str:
    """Get the contract id of the wrapped token contract."""
    network_id = stellar_xdr.Hash(hashlib.sha256(network_passphrase.encode()).digest())
    preimage = stellar_xdr.HashIDPreimage(
        type=stellar_xdr.EnvelopeType.ENVELOPE_TYPE_CONTRACT_ID,
        contract_id=stellar_xdr.HashIDPreimageContractID(
            network_id=network_id,
            contract_id_preimage=stellar_xdr.ContractIDPreimage(
                type=stellar_xdr.ContractIDPreimageType.CONTRACT_ID_PREIMAGE_FROM_ASSET,
                from_asset=asset.to_xdr_object(),
            ),
        ),
    )
    return stellar_xdr.Hash(hashlib.sha256(preimage.to_xdr_bytes()).digest()).hash.hex()

def get_asset_contract_address(token_id: str) -> str:
    """Get the contract address of a wrapped token contract."""
    token_address = StrKey.encode_contract(bytes.fromhex(token_id))
    return token_address


if __name__ == '__main__':
    network_passphrase = Network.FUTURENET_NETWORK_PASSPHRASE
    asset = Asset.native() # Asset("QUEST6", "GAS4VPQ22OBEAEWBZZIO2ENPGPZEOPJ4JBSN6F7BIQQDGAHUXY7XJAR2")

    token_id = get_asset_contract_id(asset, network_passphrase)
    token_address = get_asset_contract_address(token_id)

    print(f"Contract ID: {token_id}")
    print(f"Contract Address: {token_address}")
