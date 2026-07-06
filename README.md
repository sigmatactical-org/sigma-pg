# sigma-pg

Shared PostgreSQL helpers for Sigma Tactical Group web services: connection pooling, schema migrations, and JSONB document storage per service schema.

Repository: https://github.com/sigmatactical-org/sigma-pg

## Usage

Add as a dependency:

```toml
sigma-pg = { git = "https://github.com/sigmatactical-org/sigma-pg.git" }
```

```rust
let pool = sigma_pg::connect().await?;
let data: MyDb = sigma_pg::load_document(&pool, "catalog").await?;
sigma_pg::save_document(&pool, "catalog", &data).await?;
```

## Configuration

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | PostgreSQL connection URL for this service (see per-service roles below) |
| `SIGMA_PG_MIGRATE` | Set to `1` to run embedded migrations on connect (any user) |
| `SIGMA_PG_SKIP_MIGRATE` | Set to `1` to skip migrations even when connecting as `sigma` |

Each service connects with its own database role and only has access to its schema. The `sigma` superuser (or `SIGMA_PG_MIGRATE=1`) runs migrations.

## Schemas

Embedded migrations create one JSONB document table per service schema:

- `catalog.document`
- `store.document`
- `cart.document`
- `contact.document`
- `accounting.document`
- `"order".document`
- `identity` — session tables (created by identity via tower-sessions)

Local dev connection URLs (password `sigma`):

| Service | `DATABASE_URL` |
|---------|----------------|
| catalog | `postgres://catalog:sigma@127.0.0.1:5432/sigma` |
| store | `postgres://store:sigma@127.0.0.1:5432/sigma` |
| cart | `postgres://cart:sigma@127.0.0.1:5432/sigma` |
| contact | `postgres://contact:sigma@127.0.0.1:5432/sigma` |
| accounting | `postgres://accounting:sigma@127.0.0.1:5432/sigma` |
| order | `postgres://order:sigma@127.0.0.1:5432/sigma` |
| identity | `postgres://identity:sigma@127.0.0.1:5432/sigma` |
| migrations | `postgres://sigma:sigma@127.0.0.1:5432/sigma` |

`init/01-keycloak-db.sql` is for Docker Postgres init (creates the `keycloak` database).

## Local Postgres

**Single source of truth** for local Sigma PostgreSQL: this repo's `docker-compose.deps.yml` and `init/` scripts. Other repos include or invoke this file — do not duplicate Postgres compose elsewhere.

Clone and start (creates `sigma` and `keycloak` databases):

```bash
git clone https://github.com/sigmatactical-org/sigma-pg.git
cd sigma-pg
docker compose -f docker-compose.deps.yml up -d
```

Apply schema migrations once (connect as `sigma`, or any user with `SIGMA_PG_MIGRATE=1`):

```bash
DATABASE_URL=postgres://sigma:sigma@127.0.0.1:5432/sigma cargo test -p sigma-pg
```

The first connection as `sigma` applies pending migrations automatically.

### Used by other repos

| Consumer | How |
|----------|-----|
| [identity](https://github.com/sigmatactical-org/identity) | `scripts/dev-stack.sh` starts this compose; devcontainer `include`s it |
| store / catalog / cart / contact / accounting / order | Per-service `DATABASE_URL` after migrations have run |
| conformance harness | `conformance-stack.sh` starts sigma-pg compose; identity uses `host.docker.internal:5432` |

Set `SIGMA_PG_DIR` if the checkout is not at `../sigma-pg` relative to identity.
