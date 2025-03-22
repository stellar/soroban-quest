#!/bin/bash

# Common scripts
COPY library-scripts/common-debian.sh /tmp/library-scripts/
RUN apt-get update && bash /tmp/library-scripts/common-debian.sh

# Populate dotfiles
mkdir ./.vscode
mv mv dotfiles/settings.json ./.vscode/
mv mv dotfiles/tasks.json ./.vscode/
