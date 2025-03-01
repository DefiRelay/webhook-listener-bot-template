 


# compile the binary 
FROM rust:1.85.0-slim-bullseye AS builder
WORKDIR /app
COPY Cargo.toml /app/
COPY Cargo.lock /app/
COPY abi /app/abi/
COPY src /app/src/
RUN apt update && apt install -y pkg-config libssl-dev ca-certificates
RUN cargo build --release




#set up the scrapebot_runtime image with the binary and env 
FROM debian:bullseye-slim AS webhook_listener_bot_runtime
WORKDIR /app
COPY --from=builder /app/target/release/webhook_listener_bot /app/webhook_listener_bot
COPY --from=builder /app/abi /app/abi
#COPY .env /app/.env

#run the app 
ENTRYPOINT ["/app/webhook_listener_bot"]

 

  