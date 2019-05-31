CREATE TABLE expenditures (
  id SERIAL PRIMARY KEY,
  planned_date DATE NOT NULL,
  actual_date DATE,
  amount INTEGER NOT NULL,
  description TEXT,
  expenditure_kind_id INTEGER NOT NULL,
  budget_id INTEGER NOT NULL,
  FOREIGN KEY (budget_id) REFERENCES budgets(id),
  FOREIGN KEY (expenditure_kind_id) REFERENCES expenditure_kinds(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
