-- Accounting: expense tracking. Direct spend records, optionally linked to a
-- vendor bill (same schema, real foreign key) and/or a sales order (cross-
-- service reference by id, validated over HTTP like accounting.bills.order_id).

CREATE TABLE IF NOT EXISTS accounting.expenses (
    id TEXT PRIMARY KEY,
    expense_date DATE NOT NULL,
    category TEXT NOT NULL CHECK (
        category IN ('materials', 'shipping', 'tooling', 'software', 'travel', 'fees', 'other')
    ),
    description TEXT NOT NULL,
    vendor TEXT,
    amount_cents BIGINT NOT NULL CHECK (amount_cents >= 1),
    currency TEXT NOT NULL DEFAULT 'USD',
    receipt_uri TEXT,
    bill_id TEXT REFERENCES accounting.bills (id) ON DELETE SET NULL,
    order_id TEXT,
    notes TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS accounting_expenses_expense_date
    ON accounting.expenses (expense_date DESC);
CREATE INDEX IF NOT EXISTS accounting_expenses_category
    ON accounting.expenses (category);
CREATE INDEX IF NOT EXISTS accounting_expenses_bill_id
    ON accounting.expenses (bill_id) WHERE bill_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS accounting_expenses_order_id
    ON accounting.expenses (order_id) WHERE order_id IS NOT NULL;

GRANT SELECT, INSERT, UPDATE, DELETE ON accounting.expenses TO accounting;
