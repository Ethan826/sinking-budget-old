use chrono::{NaiveDate, NaiveDateTime};

use crate::schema::budgets;

#[derive(Queryable, Debug, Default, Insertable, PartialEq)]
#[table_name = "budgets"]
pub struct NewBudget {
    pub name: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Queryable, Debug, PartialEq)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(test)]
pub(super) mod test {
    use super::*;
    use crate::models::test::run_in_transaction;

    use crate::schema::budgets::dsl::*;
    use diesel::prelude::*;

    impl From<Budget> for NewBudget {
        fn from(input: Budget) -> NewBudget {
            NewBudget {
                name: input.name,
                start_date: Some(input.start_date),
                end_date: input.end_date,
            }
        }
    }

    #[test]
    fn test_budget_db_round_trip() {
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

    pub fn mock_budget(conn: &PgConnection) -> Result<i32, diesel::result::Error> {
        let budget = NewBudget {
            name: "Ethan's Budget".into(),
            ..Default::default()
        };

        Ok(diesel::insert_into(budgets)
            .values(&budget)
            .returning(crate::schema::budgets::dsl::id)
            .get_result(conn)?)
    }
}
