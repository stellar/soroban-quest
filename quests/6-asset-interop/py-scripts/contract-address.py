from stellar_sdk import StrKey

def get_contract_address(contract_id: str) -> str:
    contract_id_bytes = bytes.fromhex(contract_id)
    contract_address = StrKey.encode_contract(contract_id_bytes)
    return contract_address

if __name__ == '__main__':
    contract_id = 'd93f5c7bb0ebc4a9c8f727c5cebc4e41194d38257e1d0d910356b43bfc528813'
    print(f"Contract Address: {get_contract_address(contract_id)}")
