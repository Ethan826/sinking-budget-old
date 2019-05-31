CREATE TABLE contributions (
  id SERIAL PRIMARY KEY,
  budget_id INTEGER NOT NULL,
  amount INTEGER NOT NULL,
  planned_date DATE NOT NULL,
  actual_date DATE,
  FOREIGN KEY (budget_id) REFERENCES budgets(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
