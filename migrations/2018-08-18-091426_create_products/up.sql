CREATE TABLE products (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  specification_id INTEGER NOT NULL,
  FOREIGN KEY (specification_id) REFERENCES specifications(id)
)
