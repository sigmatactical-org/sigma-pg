-- addresses and payments schemas/roles/tables, matching the isolation model
-- established in 001_sigma_init.sql.

CREATE SCHEMA IF NOT EXISTS addresses;
CREATE SCHEMA IF NOT EXISTS payments;

DO $$ BEGIN CREATE ROLE addresses LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE payments LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;

GRANT CONNECT ON DATABASE sigma TO addresses, payments;

-- addresses
CREATE TABLE addresses.addresses (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    category TEXT NOT NULL CHECK (category IN ('billing', 'shipping')),
    label TEXT,
    recipient_name TEXT,
    line1 TEXT NOT NULL,
    line2 TEXT,
    city TEXT NOT NULL,
    region TEXT,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX addresses_addresses_user_id ON addresses.addresses (user_id);
CREATE INDEX addresses_addresses_user_category ON addresses.addresses (user_id, category);
-- One default address per user per category (the billing address, the
-- default shipping address).
CREATE UNIQUE INDEX addresses_addresses_user_category_default
    ON addresses.addresses (user_id, category)
    WHERE is_default;

-- payments
-- billing_address_id references addresses.addresses(id) by convention only
-- (no DB foreign key): addresses and payments are independently owned
-- services sharing one Postgres instance, same as identity user ids aren't
-- FK-enforced against Keycloak. Payments validates the reference over HTTP.
CREATE TABLE payments.payment_methods (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    method_type TEXT NOT NULL CHECK (method_type IN ('credit_card', 'bank_account')),
    billing_address_id TEXT NOT NULL,
    label TEXT,
    brand TEXT,
    last4 TEXT NOT NULL,
    expiry_month SMALLINT,
    expiry_year SMALLINT,
    is_default BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX payments_payment_methods_user_id ON payments.payment_methods (user_id);
-- One default payment method per user.
CREATE UNIQUE INDEX payments_payment_methods_user_default
    ON payments.payment_methods (user_id)
    WHERE is_default;

-- per-service schema isolation
REVOKE ALL ON SCHEMA addresses FROM PUBLIC;
GRANT USAGE ON SCHEMA addresses TO addresses;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA addresses TO addresses;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA addresses
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO addresses;

REVOKE ALL ON SCHEMA payments FROM PUBLIC;
GRANT USAGE ON SCHEMA payments TO payments;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA payments TO payments;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA payments
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO payments;
