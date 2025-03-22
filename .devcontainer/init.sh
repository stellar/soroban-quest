#!/bin/bash

# Common scripts
COPY library-scripts/common-debian.sh /tmp/library-scripts/
RUN apt-get update && bash /tmp/library-scripts/common-debian.sh

# Populate dotfiles
mkdir ./.vscode
mv .devcontainer/dotfiles/settings.json .vscode/
mv .devcontainer/dotfiles/tasks.json .vscode/

mv .devcontainer/dotfiles/.bashrc "$HOME"

# zsh prompt
git config devcontainers-theme.hide-status 1
