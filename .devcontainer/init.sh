#!/bin/bash

# Populate dotfiles
mkdir -p .vscode/
mv .devcontainer/dotfiles/settings.json .vscode/
mv .devcontainer/dotfiles/tasks.json .vscode/

mv .devcontainer/dotfiles/.bashrc $HOME/
mv .devcontainer/dotfiles/.zshrc $HOME/

