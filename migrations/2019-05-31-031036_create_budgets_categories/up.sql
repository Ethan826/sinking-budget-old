CREATE TABLE budgets_categories (
  id SERIAL PRIMARY KEY,
  budget_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  start_date DATE NOT NULL DEFAULT NOW(),
  end_date DATE NOT NULL,
  amount_required INTEGER NOT NULL,
  balance INTEGER NOT NULL,
  FOREIGN KEY (budget_id) REFERENCES budgets(id),
  FOREIGN KEY (category_id) REFERENCES categories(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE(start_date, end_date, category_id, budget_id)
);
