CREATE TYPE order_status AS ENUM (
    'entering', 'in_progress', 'cancelled', 'completed'
);

CREATE TABLE orders (
  id INTEGER NOT NULL PRIMARY KEY,
  specification_id INTEGER NOT NULL,
  quantity INTEGER NOT NULL,
  status order_status NOT NULL
);