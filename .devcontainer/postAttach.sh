#!/bin/bash
set -e

# Terminal style
apt-get install fonts-powerline
starship preset no-runtime-versions -o ~/.config/starship.toml
code --install-extension idleberg.emoji-code

# Create tunnel
cloudflared tunnel --url http://localhost:8080

brew install stellar-cli
echo "source <(stellar completion --shell bash)" >>~/.bashrc
echo "Enabled Stellar CLI auto-completion"
