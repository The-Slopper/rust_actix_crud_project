# Rust Actix Crud - (c) 2026 Example Org
FROM rust:1.72

WORKDIR /app

COPY . .
RUN cargo build

EXPOSE 3000

CMD ["sh", "-c", "cargo run"]
