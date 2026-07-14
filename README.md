# sigma-pg

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

Shared PostgreSQL helpers for Sigma Tactical Group web services: connection pooling, schema migrations, and per-service relational schemas.

Repository: https://github.com/sigmatactical-org/sigma-pg

## Usage

Add as a dependency:

```toml
sigma-pg = { git = "https://github.com/sigmatactical-org/sigma-pg.git" }
```

```rust
let pool = sigma_pg::connect().await?;
sigma_pg::ping(&pool).await?;
```

Each service owns SQL queries against its schema tables (for example `catalog.skus`, `cart.carts`). Embedded migrations create those tables and indexes.

## Configuration

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | PostgreSQL connection URL for this service (see per-service roles below) |
| `SIGMA_PG_MIGRATE` | Set to `1` to run embedded migrations on connect (any user) |
| `SIGMA_PG_SKIP_MIGRATE` | Set to `1` to skip migrations even when connecting as `sigma` |

Each service connects with its own database role and only has access to its schema. The `sigma` superuser (or `SIGMA_PG_MIGRATE=1`) runs migrations.

## Schemas

| Schema | Tables |
|--------|--------|
| `catalog` | `skus`, `sku_components` |
| `store` | `listings` |
| `cart` | `carts`, `cart_lines` |
| `"order"` | `orders`, `order_lines` |
| `contact` | `contacts` |
| `accounting` | `bills`, `bill_line_items`, `integrations` |
| `identity` | session tables (tower-sessions) |
| `sentry` | `events` |

## Migrations

`001_sigma_init.sql` — schemas, service roles, tables, indexes, constraints, and grants (including sentry).

`002_keycloak_schema.sql` — `keycloak` schema in database `sigma` for the IdP (Keycloak owns the tables).

PostgreSQL layout: **one application database** (`sigma`) with per-service schemas plus `keycloak` for the identity provider. Keycloak connects with `KC_DB_URL_DATABASE=sigma` and `KC_DB_SCHEMA=keycloak`.

**Reset (dev only)** — drops all application schemas and re-applies the single migration:

```bash
cd platform
./scripts/postgres-dev.sh reset-and-seed
```

Or schema-only reset (no dev seed data):

```bash
DATABASE_URL=postgres://sigma:sigma@127.0.0.1:5432/sigma cargo run --bin sigma-pg-reset
```

Use after squashing migrations or when you need a clean local database. This deletes all application data. Prefer `reset-and-seed` in dev so Keycloak users and the Sigma Racer product are recreated.

Passwords are in the private `platform` repo (`.env.dev-seed`). When using schema-only `reset`, run:

```bash
cd platform
./scripts/seed-keycloak-dev-users.sh
```

See `platform/.env.dev-seed` for credentials.

## Local development (kind)

PostgreSQL runs **in the kind cluster** ([platform](https://github.com/sigmatactical-org/platform) `services/postgres`).

1. Deploy the dev stack (once) — see platform README.
2. Port-forward and migrate:

   ```bash
   cd platform
   ./scripts/postgres-dev.sh port-forward-bg
   ./scripts/postgres-dev.sh migrate
   ```

Host connection URLs (password `sigma`, port-forward on `127.0.0.1:5432`):

| Service | `DATABASE_URL` |
|---------|----------------|
| catalog | `postgres://catalog:sigma@127.0.0.1:5432/sigma` |
| store | `postgres://store:sigma@127.0.0.1:5432/sigma` |
| cart | `postgres://cart:sigma@127.0.0.1:5432/sigma` |
| contact | `postgres://contact:sigma@127.0.0.1:5432/sigma` |
| accounting | `postgres://accounting:sigma@127.0.0.1:5432/sigma` |
| order | `postgres://order:sigma@127.0.0.1:5432/sigma` |
| identity | `postgres://identity:sigma@127.0.0.1:5432/sigma` |
| sentry | `postgres://sentry:sigma@127.0.0.1:5432/sigma` |
| migrations | `postgres://sigma:sigma@127.0.0.1:5432/sigma` |

Set `SIGMA_PG_DIR` when running `postgres-dev.sh migrate` from a non-standard checkout path.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

MIT OR Apache-2.0 for **source code** only. Branding remains proprietary.
