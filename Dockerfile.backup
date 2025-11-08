# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copiar arquivos de dependências
COPY Cargo.toml Cargo.lock ./

# Copiar código
COPY src ./src

# Build
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binário do builder
COPY --from=builder /app/target/release/chatbot-volt-clickmassa /app/

# Expor porta
EXPOSE 3000

# Variáveis de ambiente
ENV RUST_LOG=info
ENV PORT=3000

# Executar
CMD ["./chatbot-volt-clickmassa"]
