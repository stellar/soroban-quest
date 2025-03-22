#!/bin/bash

# Terminal style
apt-get install fonts-powerline
starship preset no-runtime-versions -o ~/.config/starship.toml
code --install-extension idleberg.emoji-code

# Create tunnel
cloudflared tunnel --url http://localhost:8080
