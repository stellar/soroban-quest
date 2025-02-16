FROM gitpod/workspace-full:2025-02-10-10-54-28
LABEL version="1.1.24"

RUN mkdir -p ~/.local/bin
RUN curl -L https://github.com/stellar/stellar-cli/releases/download/v22.1.0/stellar-cli-22.1.0-x86_64-unknown-linux-gnu.tar.gz | tar xz -C ~/.local/bin
RUN chmod +x ~/.local/bin/stellar
RUN echo "source <(stellar completion --shell bash)" >> ~/.bashrc
RUN curl -L https://github.com/mozilla/sccache/releases/download/v0.3.3/sccache-v0.3.3-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components 1 -C ~/.local/bin sccache-v0.3.3-x86_64-unknown-linux-musl/sccache
RUN chmod +x ~/.local/bin/sccache
RUN curl -L https://github.com/watchexec/cargo-watch/releases/download/v8.1.2/cargo-watch-v8.1.2-x86_64-unknown-linux-gnu.tar.xz | tar xJ --strip-components 1 -C ~/.local/bin cargo-watch-v8.1.2-x86_64-unknown-linux-gnu/cargo-watch

RUN curl -LO https://github.com/denoland/deno/releases/download/v1.30.1/deno-x86_64-unknown-linux-gnu.zip
RUN unzip deno-x86_64-unknown-linux-gnu.zip -d ~/.local/bin

RUN git clone https://github.com/stellar/soroban-quest--pioneer.git ~/.local/_tmp/soroban-quest && \
    mv ~/.local/_tmp/soroban-quest/_client ~/.local && \
    cd ~/.local/_tmp/soroban-quest/_squirtle && \
    mv bash-hook ~/.local && \
    npm run package && \
    cd ~/.local && \
    rm -rf ~/.local/_tmp

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache

# In order to reliably install the most up-to-date version of Rust, we first
# uninstall the existing toolchain.
# https://github.com/gitpod-io/workspace-images/issues/933#issuecomment-1272616892
RUN rustup self uninstall -y
RUN rm -rf .rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y

RUN rustup install 1.76
RUN rustup target add --toolchain 1.76 wasm32-unknown-unknown
RUN rustup component add --toolchain 1.76 rust-src
RUN rustup default 1.76

RUN sudo apt-get update && sudo apt-get install -y binaryen

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
