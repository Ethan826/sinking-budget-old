use crate::schema::expenditures;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Queryable, Debug, Insertable, PartialEq)]
#[table_name = "expenditures"]
pub struct NewExpenditure {
    pub planned_date: NaiveDate,
    pub actual_date: Option<NaiveDate>,
    pub amount: i32,
    pub expenditure_kind_id: i32,
    pub budget_id: i32,
}

#[derive(Queryable, Debug)]
pub struct Expenditure {
    pub id: i32,
    pub planned_date: NaiveDate,
    pub actual_date: Option<NaiveDate>,
    pub amount: i32,
    pub description: Option<String>,
    pub expenditure_kind_id: i32,
    pub budget_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::budget::test::mock_budget;
    use crate::models::test::run_in_transaction;

    use diesel::prelude::*;

    impl From<Expenditure> for NewExpenditure {
        fn from(input: Expenditure) -> NewExpenditure {
            NewExpenditure {
                planned_date: input.planned_date,
                actual_date: input.actual_date,
                amount: input.amount,
                expenditure_kind_id: input.expenditure_kind_id,
                budget_id: input.budget_id,
            }
        }
    }

    #[test]
    fn test_expenditure_db_round_trip() {
        use crate::models::expenditure_kind::test::mock_expenditure_kind;
        use crate::schema::expenditures::dsl::*;

        run_in_transaction(&|conn| {
            let expenditure = NewExpenditure {
                planned_date: NaiveDate::from_ymd(1981, 8, 26),
                actual_date: None,
                amount: 500,
                expenditure_kind_id: mock_expenditure_kind(conn)?,
                budget_id: mock_budget(conn)?,
            };

            let committed_value = diesel::insert_into(expenditures)
                .values(&expenditure)
                .get_result::<Expenditure>(conn)?;

            assert_eq!(NewExpenditure::from(committed_value), expenditure);

            Ok(())
        });
    }
}
