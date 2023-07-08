FROM rustlang/rust:nightly-buster-slim AS builder
RUN update-ca-certificates
RUN apt update
RUN apt install -y libpq-dev
WORKDIR /app
COPY . /app
RUN cargo build --release 

FROM debian:buster-slim
RUN apt update
RUN apt install -y libpq-dev
COPY --from=builder /app/target/release/todo-rust /
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/todo-rust"]

# FROM gcr.io/distroless/cc
# RUN apt update
# RUN apt install -y libpq-dev
# COPY /todo-rust /
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 8000
# CMD ["/todo-rust"]