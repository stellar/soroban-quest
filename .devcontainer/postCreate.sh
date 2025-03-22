#!/bin/bash
set -e

# NPM install
. "${NVM_DIR}"/nvm.sh && nvm install --lts

cd client || exit | pnpm i
