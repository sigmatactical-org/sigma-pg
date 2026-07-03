# sigma-pg

Shared PostgreSQL helpers for Sigma Tactical Group web services: connection pooling, schema migrations, and JSONB document snapshots per service schema.

Repository: https://github.com/sigmatactical-org/sigma-pg

## Usage

Add as a dependency:

```toml
sigma-pg = { git = "https://github.com/sigmatactical-org/sigma-pg.git" }
```

```rust
let pool = sigma_pg::connect().await?;
let data: MyDb = sigma_pg::load_snapshot(&pool, "catalog").await?;
sigma_pg::save_snapshot(&pool, "catalog", &data).await?;
```

## Configuration

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | PostgreSQL connection URL (default `postgres://sigma:sigma@127.0.0.1:5432/sigma`) |

## Schemas

Embedded migrations create one JSONB snapshot table per service schema:

- `catalog.snapshot`
- `store.snapshot`
- `cart.snapshot`
- `contact.snapshot`
- `accounting.snapshot`

`init/01-keycloak-db.sql` is for Docker Postgres init (creates the `keycloak` database).

## Local Postgres

Clone this repo and start the bundled compose file (creates `sigma` and `keycloak` databases):

```bash
git clone https://github.com/sigmatactical-org/sigma-pg.git
cd sigma-pg
docker compose -f docker-compose.deps.yml up -d
```

Service crates in other repos can also point at `postgres://sigma:sigma@127.0.0.1:5432/sigma` once this container is running.
