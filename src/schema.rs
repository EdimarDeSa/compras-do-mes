// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 150]
        nickname -> Varchar,
        #[max_length = 250]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        birthday -> Nullable<Date>,
        cad_date -> Timestamptz,
    }
}
