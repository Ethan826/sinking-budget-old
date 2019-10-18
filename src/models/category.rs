use crate::schema::categories;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Default, Insertable, PartialEq)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::test::run_in_transaction;
    use diesel::prelude::*;

    impl From<Category> for NewCategory {
        fn from(input: Category) -> NewCategory {
            NewCategory { name: input.name }
        }
    }

    #[test]
    fn test_category_db_round_trip() {
        use crate::schema::categories::dsl::*;

        run_in_transaction(&|conn| {
            let category = NewCategory {
                name: "Ethan's Category".into(),
            };

            let committed_value = diesel::insert_into(categories)
                .values(&category)
                .get_result::<Category>(conn)?;

            assert_eq!(NewCategory::from(committed_value), category);

            Ok(())
        });
    }
}
