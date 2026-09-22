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


### Examples

### 1. GET All tasks

```bash
curl -X GET http://localhost:8000/to_do/v1/item \
  -H "Content-Type: application/json"


Response:

{"pending_items":[],"done_items":[],"pending_item_count":0,"done_item_count":0}                                                                             
```


### 2. Create a new task

```bash
curl -X POST http://localhost:8000/to_do/v1/item/create \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "status": "PENDING"}'

Response:

{"pending_items":[{"title":"Learn Rust","status":{"status":"PENDING"}}],"done_items":[],"pending_item_count":1,"done_item_count":0}                                                                       
```



### 3. Edit a task status

```bash
curl -X PUT "http://localhost:8000/to_do/v1/item/edit/Learn%20Rust" \
  -H "Content-Type: application/json" \
  -d '{"title": "Aprender Rust avanzado", "status": "DONE"}'

Response:

{"pending_items":[],"done_items":[{"title":"Learn Rust","status":{"status":"DONE"}}],"pending_item_count":0,"done_item_count":1}                                                                   
```


### 4. Delete a task by title

```bash
curl -X DELETE "http://localhost:8000/to_do/v1/item/edit/Learn%20Rust" \
  -H "Content-Type: application/json" \
  -d '{"title": "Aprender Rust avanzado", "status": "DONE"}'                                                        
```