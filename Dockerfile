FROM node:14-stretch-slim

COPY . $HOME

# Build frontend
# Scratch that - JS is too broken, just check in the built files for now.
# RUN cd client && npm update && npm install && npm rebuild node-sass && npm run build

# Get Rust
RUN apt-get update && apt-get install curl build-essential sqlite3 -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> rustup-init.sh
RUN chmod +x rustup-init.sh && ./rustup-init.sh -y

# Build backend
RUN $HOME/.cargo/bin/cargo build

ENTRYPOINT $HOME/.cargo/bin/cargo run --bin gameserver
