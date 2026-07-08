-- One line per SKU per cart; merge quantities in application code on conflict.
CREATE UNIQUE INDEX IF NOT EXISTS cart_cart_lines_cart_sku ON cart.cart_lines (cart_id, sku_id);
