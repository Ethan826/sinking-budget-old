#[macro_use]
extern crate diesel;

use chrono::{NaiveDate, NaiveDateTime};

pub mod models;
pub(crate) mod schema;

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
