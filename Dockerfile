FROM rust:1.65 AS builder
WORKDIR /app
COPY . .


RUN apt-get update && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install -f wasm-bindgen-cli

RUN cargo build --release --target wasm32-unknown-unknown
RUN  wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rust-rhythm-game.wasm
RUN cp index.html ./out/
RUN cp -r assets/ ./out

FROM node:14-alpine
WORKDIR /app
COPY --from=builder /app/out ./out

ENV PORT 8080
ENV HOST 127.0.0.1
EXPOSE 8080

RUN npm i -g http-server

CMD http-server out