FROM rust:1.79 as builder

WORKDIR /app

# Copie les fichiers sources
COPY . .

# Installe les dépendances nécessaires pour construire le projet
RUN apt-get update && apt-get install -y libpq-dev && \
    cargo build --release

# Étape finale : image légère contenant juste le binaire
FROM ubuntu:22.04

COPY --from=builder /app/target/release/portefeuil /app/portefeuil
COPY .env /app/.env

# Commande de lancement
CMD ["/app/portefeuil"]
