-- Identity runs tower-sessions migrate at startup, which executes
-- CREATE SCHEMA IF NOT EXISTS. That requires CREATE on the database.
GRANT CREATE ON DATABASE sigma TO identity;

-- Session table (tower-sessions default name) so migrate is a no-op at runtime.
CREATE TABLE IF NOT EXISTS identity.session (
    id text PRIMARY KEY NOT NULL,
    data bytea NOT NULL,
    expiry_date timestamptz NOT NULL
);

GRANT SELECT, INSERT, UPDATE, DELETE ON identity.session TO identity;
