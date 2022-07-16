### Node build image
FROM node:18.6.0-buster-slim AS build-node

WORKDIR /build

# Moving all the files declairing our dependencies, and downloading them
COPY ./package.json ./package.json
COPY ./yarn.lock ./yarn.lock

RUN yarn

# Moving all the files needed to actually build our assets
COPY ./tsconfig.json ./tsconfig.json
COPY ./webpack.config.js ./webpack.config.js
COPY ./assets ./assets

RUN yarn build:production

### Rust build image
FROM rust:1.62-slim-buster AS build-rust

RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev
WORKDIR /build

# Building just our dependencies first, for caching purposes!
RUN cargo init --name yarms \
    && cargo new entity --lib --name yarms-entity \
    && cargo new migration --lib --name yarms-migration \
    && cargo new security --lib --name yarms-security

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./entity/Cargo.toml ./entity/Cargo.toml 
COPY ./migration/Cargo.toml ./migration/Cargo.toml 
COPY ./security/Cargo.toml ./security/Cargo.toml 

RUN cargo build --release

# Building our actual software
RUN rm src/* entity/src/* migration/src/* security/src/* -rf

COPY ./src ./src

COPY ./entity/src ./entity/src
COPY ./migration/src ./migration/src
COPY ./security/src ./security/src

RUN rm ./target/release/deps/yarms* -rf \
    && rm ./target/release/deps/libentity* -rf \
    && rm ./target/release/deps/libmigration* -rf \
    && rm ./target/release/deps/libsecurity* -rf 
RUN cargo build --release

### Actual execution image
FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y openssl \
    && addgroup --gid 1000 yarms \
    && adduser --system --shell /bin/false --uid 1000 --ingroup yarms --home /var/yarms yarms

USER yarms
WORKDIR /var/yarms

COPY --from=build-node --chown=1000:1000 /build/assets/dist ./assets/dist
COPY --from=build-rust --chown=1000:1000 /build/target/release/yarms ./yarms
COPY ./templates ./templates

RUN chmod +x ./yarms

CMD ["./yarms"]
