#!/bin/bash

# Common scripts
COPY library-scripts/common-debian.sh /tmp/library-scripts/
RUN apt-get update && bash /tmp/library-scripts/common-debian.sh

# Populate dotfiles
mkdir ./.vscode
mv dotfiles/settings.json ../.vscode/
mv dotfiles/tasks.json ../.vscode/
