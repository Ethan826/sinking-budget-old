#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;

pub mod models;
pub(crate) mod schema;

// The following structs define many-to-many relationships and so probably will
// not be in-memory objects. They are preserved here while driving to MVP for
// convenience's sake.
//
// TODO: Implement or delete.

/*
#[derive(Queryable, Debug)]
pub struct BudgetCategory {
    pub id: i32,
    pub budget_id: i32,
    pub category_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub amount_required: i32,
    pub balance: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct UsersBudgets {
    pub user_id: i32,
    pub budget_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
*/
