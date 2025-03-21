#!/bin/bash

brew install stellar-cli
echo "source <(stellar completion --shell bash)" >> ~/.bashrc
echo "Enabled Stellar CLI auto-completion"

stellar keys generate --global alice --network testnet --fund

# Install warp terminal
wget -qO- https://releases.warp.dev/linux/keys/warp.asc | gpg --dearmor > warpdotdev.gpg
sudo install -D -o root -g root -m 644 warpdotdev.gpg /etc/apt/keyrings/warpdotdev.gpg
sudo sh -c 'echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/warpdotdev.gpg] https://releases.warp.dev/linux/deb stable main" > /etc/apt/sources.list.d/warpdotdev.list'
rm warpdotdev.gpg
sudo apt update && sudo echo "Y" | apt install warp-terminal

chmod +x client/test.ts
