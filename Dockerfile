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
FROM rust:1.79-slim-buster

RUN apt update -y && apt install -y libssl-dev libpq-dev

# copy the build artifact from the build stage
COPY --from=build /maci-telegram-bot/target/release/maci-telegram-bot .

# set the startup command to run your binary
CMD ["./maci-telegram-bot"]
