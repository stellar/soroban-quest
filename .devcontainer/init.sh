#!/bin/bash
set -e

# Populate dotfiles
mkdir -p .vscode/

if [ -e .devcontainer/dotfiles/settings.json ]; then
  mv -n .devcontainer/dotfiles/settings.json .vscode/
fi

if [ -e .devcontainer/dotfiles/tasks.json ]; then
  mv -n .devcontainer/dotfiles/tasks.json .vscode/
fi

if [ -e .devcontainer/dotfiles/.bashrc ]; then
  mv -n .devcontainer/dotfiles/.bashrc $HOME/
fi

if [ -e .devcontainer/dotfiles/.zshrc ]; then
  mv -n .devcontainer/dotfiles/.zshrc $HOME/
fi
