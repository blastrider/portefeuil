# Étape 1 : Utiliser une image officielle Node.js comme image de base
FROM node:22.6-alpine AS builder

# Étape 2 : Définir le répertoire de travail dans le conteneur
WORKDIR /app

# Étape 3 : Copier les fichiers package.json et package-lock.json dans le répertoire de travail
COPY package*.json ./

# Étape 4 : Installer les dépendances de l'application
RUN npm install

# Étape 5 : Copier tout le contenu du projet dans le répertoire de travail
COPY . .

# Étape 6 : Construire l'application Next.js pour la production
RUN npm run build

# Étape 7 : Utiliser une image minimale pour exécuter l'application
FROM node:22.6-alpine AS runner

# Étape 8 : Définir le répertoire de travail dans le conteneur
WORKDIR /app

# Étape 9 : Copier les fichiers nécessaires depuis l'étape précédente
COPY --from=builder /app/package*.json ./
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/public ./public

# Étape 10 : Exposer le port sur lequel l'application va tourner
EXPOSE 3000

# Étape 11 : Commande pour lancer l'application
CMD ["npm","start"]