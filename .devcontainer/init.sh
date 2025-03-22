#!/bin/bash
set -e

# Populate dotfiles
mkdir -p .vscode/ || { echo "Command failed"; exit 1; }
mv -n .devcontainer/dotfiles/settings.json .vscode/ || { echo "Command failed"; exit 1; }
mv -n .devcontainer/dotfiles/tasks.json .vscode/ || { echo "Command failed"; exit 1; }

mv -n .devcontainer/dotfiles/.bashrc $HOME/ || { echo "Command failed"; exit 1; }
mv -n .devcontainer/dotfiles/.zshrc $HOME/ || { echo "Command failed"; exit 1; }
