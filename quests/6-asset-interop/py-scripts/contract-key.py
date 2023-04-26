from stellar_sdk import StrKey

def get_contract_key(contract_id: str) -> str:
    contract_id_bytes = bytes.fromhex(contract_id)
    contract_key = StrKey.encode_contract(contract_id_bytes)
    return contract_key

if __name__ == '__main__':
    contract_id = 'd93f5c7bb0ebc4a9c8f727c5cebc4e41194d38257e1d0d910356b43bfc528813'
    print(f"Contract Key: {get_contract_key(contract_id)}")