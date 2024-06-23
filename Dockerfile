FROM rust:1.79-buster AS build

# create a new empty shell project
RUN USER=root cargo new --bin maci-telegram-bot
WORKDIR /maci-telegram-bot

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/maci_telegram_bot*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# Set environment variables for non-interactive installation
ENV DEBIAN_FRONTEND=noninteractive

# Update the package list and install the OpenSSL development dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    ca-certificates \
    wget \
    libpq-dev \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /maci-telegram-bot/target/release/maci-telegram-bot .

# set the startup command to run your binary
CMD ["./maci-telegram-bot"]
