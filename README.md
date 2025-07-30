# Partner Locator App

This is a full-stack partner management application that allows users to browse business partners by region and country, and provides an admin interface to manage those partners.

## Features

### User View (`/`)
- Browse available countries grouped by region: EMEA, NORAM, APAC, LATAM
- View partners associated with each country

### Admin Panel (`/admin`)
- Add new partners with contact information and assigned countries
- Edit partner details, including their assigned countries
- Delete partners
- Assign or remove countries per partner from the modal interface

## Technology Stack

| Layer        | Technology                  |
|--------------|-----------------------------|
| Frontend     | React (with Vite) + Tailwind CSS |
| Backend      | Rust with Actix Web         |
| Database     | PostgreSQL (Docker)         |
| Query Layer  | SQLx (compile-time checked SQL) |

**Note:** SQLx is not an ORM — it provides type-safe interaction with raw SQL queries in Rust.

## Getting Started

### Prerequisites

Make sure the following tools are installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) and npm
- [Docker](https://www.docker.com/)
- [`sqlx-cli`](https://docs.rs/sqlx/latest/sqlx/macro.query.html) (install with `cargo install sqlx-cli`)

### 1. Start PostgreSQL with Docker

From the project root:

```bash
docker compose up -d db
```

This will start a PostgreSQL database container.

### 2. Set up Environment Variables
```bash
export DATABASE_URL=postgres://postgres:password@localhost:5432/partner_locator
```

### 3. Run Migrations
```bash
sqlx database create
sqlx migrate run
```

### 4. Start the Backend Server
```bash
cargo run
```

### 5. Start the Frontend App
```bash
npm install
npm run dev
```

## Project Structure:
partner-locator/
├── backend/               # Rust Actix Web backend
│   ├── src/
│   ├── migrations/        # SQLx migration files
│   └── Cargo.toml
├── frontend/              # React frontend with Vite and Tailwind CSS
│   ├── src/
│   ├── public/
│   └── package.json
├── docker-compose.yml     # Defines the PostgreSQL container
└── README.md              # This file

API Overview

The backend exposes RESTful endpoints for managing:

    GET /countries – list countries grouped by region

    GET /partners – list all partners with assigned countries

    POST /partners – create a new partner

    PUT /partners/:id – update partner information

    DELETE /partners/:id – delete a partner

    Additional endpoints for assigning/removing countries

Notes
-----
    All SQL is written explicitly and validated at compile-time with SQLx.
    Tailwind CSS is used for styling the frontend with utility classes.
    The admin panel provides full CRUD functionality for partner management.
    Data consistency is maintained manually — no ORM abstractions.
