pub mod budget;
pub mod category;
pub mod contribution;
pub mod expenditure;
pub mod expenditure_kind;
pub mod user;

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
