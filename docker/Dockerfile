FROM rust:1.60 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/myapp/target/release/rust_base rust_base
EXPOSE 8080
CMD ["./rust_base"]