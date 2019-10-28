use crate::models::budget::{Budget, NewBudget};
use chrono::{NaiveDate, Utc};
use juniper::{FieldResult, RootNode};

pub mod budget;
pub mod category;
pub mod contribution;
pub mod expenditure;
pub mod expenditure_kind;
pub mod user;

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field budget(&executor, id: i32) -> FieldResult<Budget> {
        Ok(Budget{
            id: 1,
            name: "Ethan".into(),
            start_date: NaiveDate::from_ymd(1981, 8, 26),
            end_date: Some(NaiveDate::from_ymd(1981, 8, 26)),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createBudget(&executor, new_budget: NewBudget) -> FieldResult<NewBudget> {
            Ok(new_budget)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

// =================================================================================================
// Tests
// =================================================================================================

#[cfg(test)]
mod test {
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use diesel::result::Error;
    use dotenv::dotenv;
    use std::env;

    pub(super) fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    pub(super) fn run_in_transaction(myfn: &dyn Fn(&PgConnection) -> Result<(), Error>) {
        let conn = establish_connection();

        conn.test_transaction::<_, Error, _>(|| myfn(&conn))
    }
}
