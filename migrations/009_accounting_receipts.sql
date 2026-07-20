-- Accounting: receipts — money received from customers, one row per
-- successful payments charge. The charge id is a cross-service reference
-- (payments schema, opaque id over HTTP, not a foreign key) and doubles as
-- the idempotency key: recording the same charge twice is a no-op, so the
-- cart's best-effort push and the reconcile backstop can safely overlap.

CREATE TABLE IF NOT EXISTS accounting.receipts (
    id TEXT PRIMARY KEY,
    charge_id TEXT NOT NULL UNIQUE,
    order_id TEXT,
    user_id TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('deposit', 'balance', 'refund')),
    amount_cents BIGINT NOT NULL CHECK (amount_cents >= 1),
    currency TEXT NOT NULL DEFAULT 'USD',
    occurred_at TIMESTAMPTZ NOT NULL,
    notes TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS accounting_receipts_occurred_at
    ON accounting.receipts (occurred_at DESC);
CREATE INDEX IF NOT EXISTS accounting_receipts_order_id
    ON accounting.receipts (order_id) WHERE order_id IS NOT NULL;

GRANT SELECT, INSERT, UPDATE, DELETE ON accounting.receipts TO accounting;
