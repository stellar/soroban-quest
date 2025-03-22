#!/bin/bash

# NPM install
. "${NVM_DIR}"/nvm.sh && nvm install --lts

cd client || exit | pnpm i @types/node



