<div align="center">

# 🚀 To-Do REST API (Rust)

*A robust, high-performance backend built with Actix-web, Diesel ORM, and PostgreSQL.*

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Actix-web](https://img.shields.io/badge/Actix--web-4.0-blue?style=for-the-badge&logo=actix)](https://actix.rs/)
[![Diesel](https://img.shields.io/badge/Diesel-ORM-blueviolet?style=for-the-badge&logo=postgresql)](https://diesel.rs/)
[![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?style=for-the-badge&logo=docker)](https://www.docker.com/)

</div>

---

## ⚙️ Getting Started

Follow these steps to set up your local development environment, spin up the database via Docker, and launch the API.

### 📦 Prerequisites

Make sure you have the following tools installed on your system:
* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* [Docker & Docker Compose](https://docs.docker.com/get-docker/)

---

### 🛠️ Installation & Setup Guide

#### 1. Install System Dependencies
Diesel requires PostgreSQL C development headers to compile correctly on Linux distributions (Debian / Ubuntu / Kali):
```bash
sudo apt update && sudo apt install libpq-dev -y
```

#### 2. Install Diesel CLI

Install the Diesel command-line tool explicitly with PostgreSQL support:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

#### 3. Start PostgreSQL via Docker Compose

Spin up the database container defined in the project:

```bash

docker compose up -d
```

#### 4. Configure Environment Variables

Create a .env file in the root directory of the project and set your database connection string:

```bash
DATABASE_URL=postgres://username:password@localhost:5432/to_do_table
```

#### 5. Run Database Migrations

Apply Diesel migrations to generate the required schema tables:

```bash
diesel migration run
```

#### 6. Run the Application

Start the development server with Cargo:

```bash
cargo run
```

✨ The API will be up and running at: http://127.0.0.1:8000
