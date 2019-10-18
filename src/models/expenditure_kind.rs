use crate::schema::expenditure_kinds;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Default, Insertable, PartialEq)]
#[table_name = "expenditure_kinds"]
pub struct NewExpenditureKind {
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct ExpenditureKind {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::test::run_in_transaction;

    use diesel::prelude::*;

    impl From<ExpenditureKind> for NewExpenditureKind {
        fn from(input: ExpenditureKind) -> NewExpenditureKind {
            NewExpenditureKind { name: input.name }
        }
    }

    #[test]
    fn test_expenditure_kind_db_round_trip() {
        use crate::schema::expenditure_kinds::dsl::*;

        run_in_transaction(&|conn| {
            let expenditure_kind = NewExpenditureKind {
                name: "Ethan's ExpenditureKind".into(),
            };

            let committed_value = diesel::insert_into(expenditure_kinds)
                .values(&expenditure_kind)
                .get_result::<ExpenditureKind>(conn)?;

            assert_eq!(NewExpenditureKind::from(committed_value), expenditure_kind);

            Ok(())
        });
    }
}
