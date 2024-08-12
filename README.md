# Documentation de l'API de Portefeuil

## Introduction

L'API de Portefeuil permet de gérer un budget domestique en suivant les dépenses liées aux courses. Elle inclut des fonctionnalités pour l'authentification des utilisateurs, ainsi que pour l'ajout, la récupération, la suppression et le filtrage des courses.

## Base URL

http://localhost:8080

## Endpoints

### 1. Inscription d'un utilisateur

**URL :** `/register`

**Méthode HTTP :** `POST`

**Description :** Permet à un nouvel utilisateur de s'inscrire.

_Corps de la requête :_

```json
{
  "username": "string", // Nom d'utilisateur (obligatoire)
  "email": "string", // Email de l'utilisateur (obligatoire)
  "password": "string" // Mot de passe (obligatoire)
}
```

**Exemple de requête :**

```bash
curl -X POST http://localhost:8080/register \
-H "Content-Type: application/json" \
-d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "password123"
}'
```

Réponse :

Code HTTP 200 : L'utilisateur a été inscrit avec succès.

```bash
User registered successfully
```

Code HTTP 400 : Requête invalide (par exemple, si un champ obligatoire est manquant ou si l'utilisateur existe déjà).

Code HTTP 500 : Erreur interne du serveur.

### 2. Connexion d'un utilisateur

**URL :** `/login`

**Méthode HTTP :** `POST`

Description : Permet à un utilisateur existant de se connecter et de recevoir un token JWT.

_Corps de la requête :_

```json
{
  "email": "string", // Email de l'utilisateur (obligatoire)
  "password": "string" // Mot de passe (obligatoire)
}
```

**Exemple de requête :**

```bash
curl -X POST http://localhost:8080/login \
-H "Content-Type: application/json" \
-d '{
    "email": "john@example.com",
    "password": "password123"
}'
```

Réponse :

Code HTTP 200 : Connexion réussie, retourne un token JWT.

```bash
  "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

Code HTTP 401 : Identifiants incorrects.

Code HTTP 500 : Erreur interne du serveur.

### 3. Récupérer la liste des courses (avec filtres)

**URL :** `/api/courses`

**Méthode HTTP :** `GET`

Description : Récupère toutes les courses enregistrées dans la base de données, avec des options de filtrage. Cette route est protégée par JWT.

Paramètres de requête :

Tous les paramètres sont facultatifs. Si aucun paramètre n'est fourni, toutes les courses seront retournées.

name : Filtre les courses par nom (correspondance partielle).
category : Filtre les courses par catégorie exacte.
date : Filtre les courses par date exacte (format YYYY-MM-DD).
min_amount : Filtre les courses avec un montant supérieur ou égal à cette valeur.
max_amount : Filtre les courses avec un montant inférieur ou égal à cette valeur.
Corps de la requête :
**Exemple de requête :**

```bash
curl -X GET "http://localhost:8080/api/courses?name=alimentaire&min_amount=20&max_amount=100" \
-H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." | jq
```

Réponse :

Code HTTP 200 : Liste des courses filtrées.

```json
[
  {
    "id": 1,
    "name": "Courses alimentaires",
    "amount": 50.75,
    "category": "Alimentation",
    "date": "2024-08-10"
  }
]
```

Code HTTP 401 : Token JWT manquant ou invalide.

Code HTTP 500 : Erreur interne du serveur.

### 4. Ajouter une course

**URL :** `/api/courses`

**Méthode HTTP :** `POST`

Description : Ajoute une nouvelle course à la base de données. Cette route est protégée par JWT.

_Corps de la requête :_

```json
{
  "name": "string", // Le nom de la course (obligatoire)
  "amount": "number", // Montant dépensé (obligatoire)
  "category": "string", // Catégorie de la course (ex: Alimentation) (obligatoire)
  "date": "YYYY-MM-DD" // Date de la course (obligatoire)
}
```

**Exemple de requête :**

```bash
curl -X POST http://localhost:8080/api/courses \
-H "Content-Type: application/json" \
-H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
-d '{
    "name": "Courses alimentaires",
    "amount": 50.75,
    "category": "Alimentation",
    "date": "2024-08-10"
}'
```

Réponse :

```log
200
```

```bash
{
  "id": 1
}
```

Code HTTP 400 : Requête invalide (par exemple, si un champ obligatoire est manquant).

Code HTTP 401 : Token JWT manquant ou invalide.

Code HTTP 500 : Erreur interne du serveur.

### 5. Supprimer une course

**URL :** `/api/courses/{id}`

**Méthode HTTP :** `DELETE`

Description : Supprime une course de la base de données. Cette route est protégée par JWT.

_Paramètres de chemin :_

id (integer, obligatoire) : L'ID de la course à supprimer.

**Exemple de requête :**

```bash
curl -X DELETE http://localhost:8080/api/courses/1 \
-H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

Réponse :

Code HTTP 200 : Course a été supprimée avec succès.

Code HTTP 401 : Token JWT manquant ou invalide.

Code HTTP 500 : Erreur interne du serveur.

### 6. Vérifier l'état de l'API

**URL :** `/health`

**Méthode HTTP :** `GET`

Description : Vérifie si l'API est opérationnelle et connectée à la base de données.

**Exemple de requête :**

```bash
curl -X GET http://localhost:8080/health
```

Réponse :

Code HTTP 200 : API et base de données sont connectées.

```text
API is running and database is connected
Code HTTP 500 : L'API est opérationnelle, mais la connexion à la base de données a échoué.
```

( après "30 secondes")

### 7. Vérifier et réparer les ID des courses

**URL :** `/check-repair-ids`

Méthode HTTP : GET

Description : Vérifie si les ID des courses se suivent sans trou, et les répare si nécessaire. Cette route est protégée par JWT.

```bash
curl -X GET http://localhost:8080/check-repair-ids \
-H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

Réponse :

Code HTTP 200 :

Si tous les ID sont consécutifs :

```text
All IDs are consecutive, no repair needed
```

Si la séquence d'ID a été réparée :

```text
ID sequence was broken and has been repaired
```

Code HTTP 500 : Erreur interne du serveur ou échec de la réparation.

## Gestion des erreurs

L'API utilise les codes d'état HTTP standard pour indiquer le succès ou l'échec des opérations :

200 OK : La requête a réussi.
400 Bad Request : La requête est invalide.
401 Unauthorized : Authentification requise.
404 Not Found : La ressource demandée n'a pas été trouvée.
500 Internal Server Error : Une erreur s'est produite côté serveur.

## Sécurité

JWT : Les routes protégées nécessitent un token JWT valide pour être accédées. Ce token doit être passé dans l'en-tête Authorization de la requête sous la forme Bearer <token>.

## Notes

Pagination et Filtrage : Le filtrage est disponible sur les paramètres name, category, date, min_amount, et max_amount pour la route GET /courses.
Authentification : Les utilisateurs doivent s'inscrire via /register et se connecter via /login pour obtenir un token JWT.
Protéger les routes : Les routes sensibles (ajout, suppression, etc.) sont protégées par JWT.
