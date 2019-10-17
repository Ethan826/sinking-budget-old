table! {
    budgets (id) {
        id -> Int4,
        name -> Text,
        start_date -> Date,
        end_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    budgets_categories (id) {
        id -> Int4,
        budget_id -> Int4,
        category_id -> Int4,
        start_date -> Date,
        end_date -> Date,
        amount_required -> Int4,
        balance -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    contributions (id) {
        id -> Int4,
        budget_id -> Int4,
        amount -> Int4,
        planned_date -> Date,
        actual_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    expenditure_kinds (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    expenditures (id) {
        id -> Int4,
        planned_date -> Date,
        actual_date -> Nullable<Date>,
        amount -> Int4,
        description -> Nullable<Text>,
        expenditure_kind_id -> Int4,
        budget_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_budgets (user_id, budget_id) {
        user_id -> Int4,
        budget_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(budgets_categories -> budgets (budget_id));
joinable!(budgets_categories -> categories (category_id));
joinable!(contributions -> budgets (budget_id));
joinable!(expenditures -> budgets (budget_id));
joinable!(expenditures -> expenditure_kinds (expenditure_kind_id));
joinable!(users_budgets -> budgets (budget_id));
joinable!(users_budgets -> users (user_id));

allow_tables_to_appear_in_same_query!(
    budgets,
    budgets_categories,
    categories,
    contributions,
    expenditure_kinds,
    expenditures,
    users,
    users_budgets,
);
