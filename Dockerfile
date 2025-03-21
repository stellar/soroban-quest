# Use the official Rust image as the base image
FROM mcr.microsoft.com/devcontainers/rust:bullseye

USER vscode

# Set the working directory
WORKDIR /workspace

RUN cargo install --locked stellar-cli --features opt

# Expose port 8000 for web development
EXPOSE 8000