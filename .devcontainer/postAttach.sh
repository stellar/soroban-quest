#!/bin/bash
set -e

# Terminal style
apt-get install fonts-powerline
starship preset no-runtime-versions -o ~/.config/starship.toml
code --install-extension idleberg.emoji-code



echo "source <(stellar completion --shell bash)" >>~/.bashrc
echo "Enabled Stellar CLI auto-completion"
