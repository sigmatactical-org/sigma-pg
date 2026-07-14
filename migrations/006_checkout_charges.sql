-- Checkout: payment charges + order address/payment references.

CREATE TABLE IF NOT EXISTS payments.charges (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    payment_method_id TEXT NOT NULL REFERENCES payments.payment_methods (id),
    amount_cents BIGINT NOT NULL CHECK (amount_cents > 0),
    currency TEXT NOT NULL DEFAULT 'usd',
    reference TEXT,
    status TEXT NOT NULL CHECK (status IN ('succeeded', 'failed')),
    failure_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS payments_charges_user_id_idx
    ON payments.charges (user_id);
CREATE INDEX IF NOT EXISTS payments_charges_reference_idx
    ON payments.charges (reference);

GRANT SELECT, INSERT ON payments.charges TO payments;

ALTER TABLE "order".orders
    ADD COLUMN IF NOT EXISTS billing_address_id TEXT,
    ADD COLUMN IF NOT EXISTS shipping_address_id TEXT,
    ADD COLUMN IF NOT EXISTS payment_method_id TEXT,
    ADD COLUMN IF NOT EXISTS charge_id TEXT,
    ADD COLUMN IF NOT EXISTS terms_accepted_at TIMESTAMPTZ;
