# Rust URL Shortener

A simple URL shortening service written in Rust using **Axum** + **SQLx** + **Postgres**, containerized with Docker, CI/CD ready, and deployable via Docker Compose or Kubernetes.

---

## Features

- Shortens long URLs to short codes  
- Stores mapping in a Postgres database  
- REST API endpoints for creating and resolving short URLs  
- Dockerized for easy deployment  
- GitHub Actions CI pipeline: build, test, and push Docker image  

---

## Repository Structure

| File / Directory        | Description |
|-------------------------|-------------|
| `src/`                  | Rust source code (handlers, database access, etc.) |
| `init.sql`              | SQL script to initialize the database schema (table definitions) |
| `Dockerfile`            | Builds the Rust app and creates a Docker container image |
| `docker-compose.yml`    | Local dev / simple deployment (app + Postgres) |
| `.github/workflows/ci.yml` | CI/CD settings (tests, building, pushing Docker image) |
| `Cargo.toml`, `Cargo.lock` | Rust project dependencies and versions |

---

## Setup & Local Development

1. Clone the repo

   ```bash
   git clone https://github.com/darrendc26/rust-url-shortener.git
   cd rust-url-shortener
   ```
2. Start Postgres and the app using Docker Compose
```bash
docker-compose up --build
```

3. Access the API

You can send requests to endpoints like POST /shorten or GET /{short_url}.

By default, the app listens on port 8080 (adjust if changed).

4. Database initialization

The init.sql script is used to create the urls table.

SQLx is used to query / mutate data.

## CI/CD

CI pipeline (GitHub Actions) runs on push / pull requests to main branch:

Checks out code

Installs Rust

Starts a Postgres service

Runs database schema init (via init.sql)

Runs tests

Docker image build & push triggered when CI passes. Image published to Docker Hub 

## 
Secrets & Environment Variables

DATABASE_URL: connection string to the Postgres database

Docker registry credentials (e.g. DOCKER_USERNAME, DOCKER_PASSWORD) stored as GitHub Secrets

