CREATE TABLE users_budgets (
  user_id INTEGER NOT NULL,
  budget_id INTEGER NOT NULL,
  PRIMARY KEY (user_id, budget_id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (budget_id) REFERENCES budgets(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
