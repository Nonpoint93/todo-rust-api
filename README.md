# To-Do Rust API REST

A robust, production-ready REST API built in **Rust**, following strict international standards, clean architecture, and enterprise design patterns.

---

## 🏗️ Architecture & Design Patterns

The project is structured around a strict **3-Layer Clean Architecture**:
- **Handlers (Actix-web):** Manages HTTP routing, request payload deserialization, and HTTP response formatting.
- **Services:** Contains core business logic and database orchestration.
- **Repositories (Diesel ORM):** Decouples database queries, utilizing the **Repository Pattern** with trait abstraction (`ToDoRepositoryTrait`) to support both PostgreSQL and in-memory mocks.

Additional design patterns implemented:
- **Factory Pattern (`to_do_factory`):** Dynamic creation and categorization of task items.
- **Strongly Typed Enums (`TaskStatus`):** Custom JSON serialization and bidirectional string conversion (`stringify` / `from_string`) to prevent invalid states.

---

## 🛠️ Tech Stack

- **Language:** Rust, rustc 1.98.1 (48a229cea 2026-09-01) | cargo 1.98.1 (797e8a9bc 2026-08-05)
- **Web Framework:** `actix-web`
- **ORM & Migrations:** `diesel` (with PostgreSQL)
- **Serialization:** `serde` / `serde_json`
- **Date & Time:** `chrono`

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

## 🧪 Testing Suite

The project includes a robust, lightning-fast testing suite covering enums, in-memory repository mocks, and E2E HTTP integration tests. 

Run all tests instantly via:
```bash
cargo test

Example:

cargo test
   Compiling to_do_api v0.1.0 (/home/kali/Git/Rust/todo-rust-api)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.10s
     Running unittests src/main.rs (target/debug/deps/to_do_api-a3bb6ee3751cb61f)

running 8 tests
test enums::task_status::tests::test_task_status_from_string_invalid_panics - should panic ... ok
test repositories::to_do_repository::tests::test_mock_repository_workflow ... ok
test enums::task_status::tests::test_task_status_from_string_valid ... ok
test enums::task_status::tests::test_task_status_stringify ... ok
test services::to_do_service::tests::test_get_tasks_endpoint ... ok
test services::to_do_service::tests::test_edit_task_endpoint_payload ... ok
test services::to_do_service::tests::test_delete_task_endpoint ... ok
test services::to_do_service::tests::test_create_task_endpoint_payload ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


```

Enum Unit Tests: Validates status parsing and strict error handling (should_panic).

Repository Mocks (MockToDoRepository): Tests CRUD operations in-memory in 0.00s without external database dependencies.

E2E Integration Tests (Actix-web): Validates full HTTP request/response pipelines (GET, POST, PUT, DELETE).

📡 API EndpointsMethodEndpointDescription

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| **GET** | `/to_do/v1/item` | Retrieves a sorted summary list of all tasks. |
| **POST** | `/to_do/v1/item/create` | Creates a new task via JSON payload (defaults to `PENDING`). |
| **PUT** | `/to_do/v1/item/edit/{title}` | Updates an existing task's status identified by its title. |
| **DELETE** | `/to_do/v1/item/delete/{title}` | Deletes a task by its title from the URI path. |
