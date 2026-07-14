-- Cardholder name for credit cards (PAN/CVV are never stored).
ALTER TABLE payments.payment_methods
    ADD COLUMN IF NOT EXISTS cardholder_name TEXT;
