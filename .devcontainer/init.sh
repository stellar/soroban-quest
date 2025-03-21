#!/bin/bash
brew install stellar-cli

brew install --cask warp

echo "source <(stellar completion --shell bash)" >> ~/.bashrc
