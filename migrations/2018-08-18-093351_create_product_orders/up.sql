CREATE TABLE product_orders (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
  FOREIGN KEY (product_id) REFERENCES products(id),
  FOREIGN KEY (order_id) REFERENCES orders(id)
)