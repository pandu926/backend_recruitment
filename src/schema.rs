// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        owner_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_applications (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        job_id -> Nullable<Int4>,
        status -> Varchar,
        applied_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobs (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        requirements -> Text,
        salary -> Numeric,
        location -> Varchar,
        company_id -> Nullable<Int4>,
        created_by_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    userdetails (id) {
        id -> Int4,
        user_id -> Int4,
        address -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        cv_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(companies -> users (owner_id));
diesel::joinable!(job_applications -> jobs (job_id));
diesel::joinable!(job_applications -> users (user_id));
diesel::joinable!(jobs -> companies (company_id));
diesel::joinable!(jobs -> users (created_by_id));
diesel::joinable!(userdetails -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    job_applications,
    jobs,
    userdetails,
    users,
);
