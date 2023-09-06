FROM rustlang/rust:nightly-buster-slim AS builder
RUN update-ca-certificates
RUN apt update
RUN apt install -y libpq-dev libssl-dev pkg-config
WORKDIR /app
COPY . /app
ARG SQLX_OFFLINE=true
RUN cargo build --release 

FROM debian:buster-slim
RUN apt update
RUN apt install -y libpq-dev
COPY --from=builder /app/target/release/entry /
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/entry"]

# FROM gcr.io/distroless/cc
# RUN apt update
# RUN apt install -y libpq-dev
# COPY /entry /
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 8000
# CMD ["/entry"]