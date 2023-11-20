# Usa un'immagine di Rust come base
FROM rust:latest as builder

# Imposta il working directory
WORKDIR /usr/src/webAppRust

# Copia i file necessari
COPY . .

# Compila l'applicazione Rust con ottimizzazioni per il rilascio
RUN cargo build --release

# Secondo stadio
FROM rust:latest

# Imposta il working directory
WORKDIR /usr/src/webAppRust

# Copia l'eseguibile dall'immagine precedente
COPY --from=builder /usr/src/webAppRust/target/release/webAppRust .

# Esponi la porta (se l'applicazione la usa)
EXPOSE 8000

# Avvia l'applicazione Rust
CMD ["./webAppRust"]


