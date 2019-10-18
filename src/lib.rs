#[macro_use]
extern crate diesel;

use chrono::{NaiveDate, NaiveDateTime};

mod schema;

use schema::budgets;

#[derive(Queryable, Debug, Default, Insertable, PartialEq)]
#[table_name = "budgets"]
struct NewBudget {
    name: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

#[derive(Queryable, Debug, PartialEq)]
struct Budget {
    id: i32,
    name: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Budget> for NewBudget {
    fn from(input: Budget) -> NewBudget {
        NewBudget {
            name: input.name,
            start_date: Some(input.start_date),
            end_date: input.end_date,
        }
    }
}

#[derive(Queryable, Debug)]
struct BudgetCategory {
    id: i32,
    budget_id: i32,
    category_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    amount_required: i32,
    balance: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
struct Categories {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
struct Contributions {
    id: i32,
    budget_id: i32,
    amount: i32,
    planned_date: NaiveDate,
    actual_date: Option<NaiveDate>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
struct ExpenditureKinds {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
struct Expenditures {
    id: i32,
    planned_date: NaiveDate,
    actual_date: Option<NaiveDate>,
    amount: i32,
    description: Option<String>,
    expenditure_kind_id: i32,
    budget_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
struct Users {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
struct UsersBudgets {
    user_id: i32,
    budget_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use diesel::result::Error;
    use dotenv::dotenv;
    use std::env;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    fn run_in_transaction(myfn: &Fn(&PgConnection) -> Result<(), Error>) {
        let conn = establish_connection();

        conn.test_transaction::<_, Error, _>(|| myfn(&conn))
    }

    #[test]
    fn test_making_a_budget() {
        use schema::budgets::dsl::*;

        run_in_transaction(&|conn| {
            let mut budget = NewBudget {
                name: "Ethan's Budget".into(),
                ..Default::default()
            };

            let committed_value = diesel::insert_into(budgets)
                .values(&budget)
                .get_result::<Budget>(conn)?;

            // The database will supply the current date for this if `None`.
            if budget.start_date.is_none() {
                budget.start_date = Some(committed_value.start_date);
            }

            assert_eq!(NewBudget::from(committed_value), budget);

            Ok(())
        });
    }
}