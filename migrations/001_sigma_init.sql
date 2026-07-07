-- Sigma application database: per-service schemas, roles, and relational tables.

CREATE SCHEMA IF NOT EXISTS catalog;
CREATE SCHEMA IF NOT EXISTS store;
CREATE SCHEMA IF NOT EXISTS cart;
CREATE SCHEMA IF NOT EXISTS contact;
CREATE SCHEMA IF NOT EXISTS accounting;
CREATE SCHEMA IF NOT EXISTS identity;
CREATE SCHEMA IF NOT EXISTS sentry;
CREATE SCHEMA IF NOT EXISTS "order";

DO $$ BEGIN CREATE ROLE catalog LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE store LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE cart LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE contact LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE accounting LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE "order" LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE identity LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE sentry LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;

REVOKE CONNECT ON DATABASE sigma FROM PUBLIC;
GRANT CONNECT ON DATABASE sigma TO sigma, catalog, store, cart, contact, accounting, "order", identity, sentry;
GRANT CREATE ON DATABASE sigma TO identity;

-- catalog
CREATE TABLE catalog.skus (
    id TEXT PRIMARY KEY,
    sku_code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    kind TEXT NOT NULL CHECK (kind IN ('simple', 'composite')),
    active BOOLEAN NOT NULL DEFAULT true,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX catalog_skus_sku_code_lower ON catalog.skus (lower(sku_code));
CREATE INDEX catalog_skus_category ON catalog.skus (category);
CREATE INDEX catalog_skus_name_lower ON catalog.skus (lower(name));

CREATE TABLE catalog.sku_components (
    parent_sku_id TEXT NOT NULL REFERENCES catalog.skus (id) ON DELETE CASCADE,
    component_sku_id TEXT NOT NULL REFERENCES catalog.skus (id),
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    CHECK (parent_sku_id <> component_sku_id),
    PRIMARY KEY (parent_sku_id, component_sku_id)
);
CREATE INDEX catalog_sku_components_component ON catalog.sku_components (component_sku_id);

-- store
CREATE TABLE store.listings (
    id TEXT PRIMARY KEY,
    sku_id TEXT NOT NULL UNIQUE,
    price_cents BIGINT,
    featured BOOLEAN NOT NULL DEFAULT false,
    visible BOOLEAN NOT NULL DEFAULT true,
    sort_order INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX store_listings_visible_sort ON store.listings (visible, sort_order, sku_id);

-- cart
CREATE TABLE cart.carts (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    status TEXT NOT NULL CHECK (status IN ('open', 'submitted', 'cancelled')),
    note TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX cart_carts_user_id ON cart.carts (user_id);
CREATE INDEX cart_carts_status ON cart.carts (status);
CREATE INDEX cart_carts_updated_at ON cart.carts (updated_at DESC);

CREATE TABLE cart.cart_lines (
    id TEXT PRIMARY KEY,
    cart_id TEXT NOT NULL REFERENCES cart.carts (id) ON DELETE CASCADE,
    sku_id TEXT NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX cart_cart_lines_cart_id ON cart.cart_lines (cart_id);
CREATE INDEX cart_cart_lines_sku_id ON cart.cart_lines (sku_id);

-- order
CREATE TABLE "order".orders (
    id TEXT PRIMARY KEY,
    cart_id TEXT NOT NULL,
    username TEXT NOT NULL,
    user_id TEXT,
    subtotal_cents BIGINT NOT NULL,
    deposit_cents BIGINT NOT NULL,
    status TEXT NOT NULL CHECK (
        status IN ('pending_deposit', 'deposit_paid', 'in_build', 'shipped', 'cancelled')
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX order_orders_cart_id ON "order".orders (cart_id);
CREATE INDEX order_orders_username ON "order".orders (username);
CREATE INDEX order_orders_user_id ON "order".orders (user_id);
CREATE INDEX order_orders_status ON "order".orders (status);
CREATE INDEX order_orders_created_at ON "order".orders (created_at DESC);
CREATE UNIQUE INDEX order_orders_cart_id_active
    ON "order".orders (cart_id)
    WHERE status <> 'cancelled';

CREATE TABLE "order".order_lines (
    order_id TEXT NOT NULL REFERENCES "order".orders (id) ON DELETE CASCADE,
    line_no SMALLINT NOT NULL,
    sku_id TEXT NOT NULL,
    sku_code TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    unit_price_cents BIGINT NOT NULL,
    line_total_cents BIGINT NOT NULL,
    deposit_cents BIGINT NOT NULL,
    PRIMARY KEY (order_id, line_no)
);
CREATE INDEX order_order_lines_sku_id ON "order".order_lines (sku_id);
CREATE INDEX order_order_lines_sku_code ON "order".order_lines (sku_code);

-- contact
CREATE TABLE contact.contacts (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL CHECK (source IN ('identity', 'external')),
    identity_id TEXT,
    display_name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    notes TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        (source = 'identity' AND identity_id IS NOT NULL)
        OR (source = 'external' AND identity_id IS NULL)
    )
);
CREATE INDEX contact_contacts_display_name_lower ON contact.contacts (lower(display_name));
CREATE INDEX contact_contacts_email ON contact.contacts (lower(email));
CREATE UNIQUE INDEX contact_contacts_identity_id ON contact.contacts (identity_id)
    WHERE source = 'identity' AND identity_id IS NOT NULL;

-- accounting
CREATE TABLE accounting.bills (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('scanned', 'digital')),
    status TEXT NOT NULL CHECK (status IN ('draft', 'approved', 'paid', 'void')),
    vendor TEXT NOT NULL,
    invoice_number TEXT,
    bill_date DATE NOT NULL,
    due_date DATE,
    currency TEXT NOT NULL DEFAULT 'USD',
    total_cents BIGINT NOT NULL,
    scan_uri TEXT,
    notes TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX accounting_bills_vendor_lower ON accounting.bills (lower(vendor));
CREATE INDEX accounting_bills_bill_date ON accounting.bills (bill_date DESC);
CREATE INDEX accounting_bills_status ON accounting.bills (status);
CREATE INDEX accounting_bills_invoice_number ON accounting.bills (invoice_number);

CREATE TABLE accounting.bill_line_items (
    bill_id TEXT NOT NULL REFERENCES accounting.bills (id) ON DELETE CASCADE,
    line_no SMALLINT NOT NULL,
    sku_id TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    unit_price_cents BIGINT NOT NULL,
    PRIMARY KEY (bill_id, line_no)
);
CREATE INDEX accounting_bill_line_items_sku_id ON accounting.bill_line_items (sku_id)
    WHERE sku_id IS NOT NULL;

CREATE TABLE accounting.integrations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL CHECK (provider IN ('quickbooks', 'xero', 'custom')),
    enabled BOOLEAN NOT NULL DEFAULT true,
    external_account_id TEXT,
    webhook_url TEXT,
    notes TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX accounting_integrations_name_lower ON accounting.integrations (lower(name));

-- sentry
CREATE TABLE sentry.events (
    id TEXT PRIMARY KEY,
    service TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (
        kind IN ('status_change', 'check_failed', 'recovered', 'probe_error')
    ),
    previous_status TEXT,
    current_status TEXT NOT NULL,
    message TEXT NOT NULL,
    detail JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX sentry_events_recorded_at ON sentry.events (recorded_at DESC);
CREATE INDEX sentry_events_service ON sentry.events (service, recorded_at DESC);

-- identity sessions (tower-sessions)
CREATE TABLE identity.session (
    id TEXT PRIMARY KEY NOT NULL,
    data BYTEA NOT NULL,
    expiry_date TIMESTAMPTZ NOT NULL
);
CREATE INDEX identity_session_expiry ON identity.session (expiry_date);

-- per-service schema isolation
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

REVOKE ALL ON SCHEMA sentry FROM PUBLIC;
GRANT USAGE ON SCHEMA sentry TO sentry;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA sentry TO sentry;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA sentry
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO sentry;
