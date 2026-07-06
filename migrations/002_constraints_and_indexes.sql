-- Integrity constraints and performance indexes (idempotent).

CREATE UNIQUE INDEX IF NOT EXISTS order_orders_cart_id_active
    ON "order".orders (cart_id)
    WHERE status <> 'cancelled';

CREATE INDEX IF NOT EXISTS identity_session_expiry ON identity.session (expiry_date);

DO $$ BEGIN
    ALTER TABLE catalog.sku_components
        ADD CONSTRAINT catalog_sku_components_no_self
        CHECK (parent_sku_id <> component_sku_id);
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    ALTER TABLE contact.contacts
        ADD CONSTRAINT contact_contacts_source_identity
        CHECK (
            (source = 'identity' AND identity_id IS NOT NULL)
            OR (source = 'external' AND identity_id IS NULL)
        );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;
