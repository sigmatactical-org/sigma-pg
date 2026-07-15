-- Accounting: link vendor bills to sales orders by id (cross-service
-- reference over HTTP, not a foreign key — order rows live in the order
-- schema owned by the orders service).

ALTER TABLE accounting.bills ADD COLUMN IF NOT EXISTS order_id TEXT;

CREATE INDEX IF NOT EXISTS accounting_bills_order_id
    ON accounting.bills (order_id) WHERE order_id IS NOT NULL;
