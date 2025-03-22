#!/bin/bash

COPY library-scripts/common-debian.sh /tmp/library-scripts/
RUN apt-get update && bash /tmp/library-scripts/common-debian.sh


mkdir ./.vscode
mv mv dotfiles/settings.json ./.vscode/settings.json
mv mv dotfiles/tasks.json ./.vscode/tasks.json
