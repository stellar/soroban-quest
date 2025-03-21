#!/bin/bash

brew install stellar-cli
echo "source <(stellar completion --shell bash)" >> ~/.bashrc
echo "Enabled Stellar CLI auto-completion"

stellar keys generate --global alice --network testnet --fund
