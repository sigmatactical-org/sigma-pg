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

## Local development (kind)

PostgreSQL runs **in the kind cluster** ([platform](https://github.com/sigmatactical-org/platform) `services/postgres`). There is no docker-compose for Postgres in this repo.

1. Deploy the dev stack (once):

   ```bash
   cd platform
   kind create cluster --name sigma-platform   # if needed
   istioctl install --set profile=demo -y
   kubectl apply -k mesh/base
   ./scripts/build-and-load.sh
   kubectl apply -k environments/dev
   ./scripts/configure-kind-ingress.sh
   ```

2. Port-forward Postgres to the host for `cargo run` / tests:

   ```bash
   ./scripts/postgres-dev.sh port-forward
   # or in the background:
   ./scripts/postgres-dev.sh port-forward-bg
   ```

3. Apply schema migrations (once per database, as `sigma`):

   ```bash
   ./scripts/postgres-dev.sh migrate
   ```

Host connection URLs (password `sigma`, with port-forward on `127.0.0.1:5432`):

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

In-cluster URLs use `postgres.sigma-dev.svc.cluster.local:5432` (see platform service configmaps).

`init/01-keycloak-db.sql` is mirrored in platform `services/postgres` init (creates the `keycloak` database).

## Used by other repos

| Consumer | How |
|----------|-----|
| platform | StatefulSet Postgres + `scripts/postgres-dev.sh` for host port-forward |
| identity | kind Postgres via port-forward; Keycloak uses in-cluster Postgres |
| store / catalog / cart / contact / accounting / order | Per-service `DATABASE_URL` in kind or via port-forward |

Set `SIGMA_PG_DIR` to the sigma-pg checkout when running `postgres-dev.sh migrate` from a non-standard path.
