#!/bin/bash
set -e

# NPM install
bash -i -c 'nvm install --lts'
git config devcontainers-theme.hide-status 1

# Create tunnel
cloudflared tunnel --url http://localhost:3000

