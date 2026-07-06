-- Per-service login roles, schema isolation, and rename snapshot → document.
-- Dev passwords are 'sigma'; override in production via ALTER ROLE … PASSWORD.

CREATE SCHEMA IF NOT EXISTS identity;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'catalog' AND tablename = 'snapshot'
    ) THEN
        ALTER TABLE catalog.snapshot RENAME TO document;
    END IF;
    IF EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'store' AND tablename = 'snapshot'
    ) THEN
        ALTER TABLE store.snapshot RENAME TO document;
    END IF;
    IF EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'cart' AND tablename = 'snapshot'
    ) THEN
        ALTER TABLE cart.snapshot RENAME TO document;
    END IF;
    IF EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'contact' AND tablename = 'snapshot'
    ) THEN
        ALTER TABLE contact.snapshot RENAME TO document;
    END IF;
    IF EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'accounting' AND tablename = 'snapshot'
    ) THEN
        ALTER TABLE accounting.snapshot RENAME TO document;
    END IF;
    IF EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'order' AND tablename = 'snapshot'
    ) THEN
        ALTER TABLE "order".snapshot RENAME TO document;
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS catalog.document (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS store.document (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS cart.document (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS contact.document (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS accounting.document (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "order".document (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'catalog') THEN
        CREATE ROLE catalog LOGIN PASSWORD 'sigma';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'store') THEN
        CREATE ROLE store LOGIN PASSWORD 'sigma';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'cart') THEN
        CREATE ROLE cart LOGIN PASSWORD 'sigma';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'contact') THEN
        CREATE ROLE contact LOGIN PASSWORD 'sigma';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'accounting') THEN
        CREATE ROLE accounting LOGIN PASSWORD 'sigma';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'order') THEN
        CREATE ROLE "order" LOGIN PASSWORD 'sigma';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'identity') THEN
        CREATE ROLE identity LOGIN PASSWORD 'sigma';
    END IF;
END $$;

REVOKE CONNECT ON DATABASE sigma FROM PUBLIC;
GRANT CONNECT ON DATABASE sigma TO sigma, catalog, store, cart, contact, accounting, "order", identity;

REVOKE ALL ON SCHEMA catalog FROM PUBLIC;
GRANT USAGE ON SCHEMA catalog TO catalog;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA catalog TO catalog;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA catalog
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO catalog;

REVOKE ALL ON SCHEMA store FROM PUBLIC;
GRANT USAGE ON SCHEMA store TO store;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA store TO store;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA store
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO store;

REVOKE ALL ON SCHEMA cart FROM PUBLIC;
GRANT USAGE ON SCHEMA cart TO cart;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA cart TO cart;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA cart
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO cart;

REVOKE ALL ON SCHEMA contact FROM PUBLIC;
GRANT USAGE ON SCHEMA contact TO contact;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA contact TO contact;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA contact
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO contact;

REVOKE ALL ON SCHEMA accounting FROM PUBLIC;
GRANT USAGE ON SCHEMA accounting TO accounting;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA accounting TO accounting;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA accounting
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO accounting;

REVOKE ALL ON SCHEMA "order" FROM PUBLIC;
GRANT USAGE ON SCHEMA "order" TO "order";
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA "order" TO "order";
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA "order"
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO "order";

REVOKE ALL ON SCHEMA identity FROM PUBLIC;
GRANT USAGE, CREATE ON SCHEMA identity TO identity;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA identity TO identity;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA identity
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO identity;
ALTER DEFAULT PRIVILEGES FOR ROLE identity IN SCHEMA identity
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO identity;
