use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Default, Insertable, PartialEq)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct User {
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

    impl From<User> for NewUser {
        fn from(input: User) -> NewUser {
            NewUser { name: input.name }
        }
    }

    #[test]
    fn test_user_db_round_trip() {
        use crate::schema::users::dsl::*;

        run_in_transaction(&|conn| {
            let user = NewUser {
                name: "Ethan's User".into(),
            };

            let committed_value = diesel::insert_into(users)
                .values(&user)
                .get_result::<User>(conn)?;

            assert_eq!(NewUser::from(committed_value), user);

            Ok(())
        });
    }
}
