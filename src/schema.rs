// @generated automatically by Diesel CLI.

diesel::table! {
    default_categorys (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    default_products (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        unity_types_id -> Uuid,
        default_categorys_id -> Uuid,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    market (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_list (id) {
        id -> Uuid,
        shopping_list_id -> Uuid,
        user_product_id -> Uuid,
        quantity -> Float4,
        value -> Money,
        total -> Money,
        on_cart -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    shopping_list (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        user_id -> Uuid,
        final_value -> Money,
        unique_itens -> Int2,
        total_itens -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    shopping_log (id) {
        id -> Uuid,
        date -> Date,
        user_id -> Uuid,
        shopping_list_id -> Uuid,
        market_id -> Uuid,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    unity_types (id) {
        id -> Uuid,
        #[max_length = 25]
        name -> Varchar,
        #[max_length = 10]
        nick -> Varchar,
        calc_value -> Int2,
    }
}

diesel::table! {
    user_categorys (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_products (id) {
        id -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        unity_type_id -> Uuid,
        value -> Money,
        value_unity_type_id -> Uuid,
        category_id -> Uuid,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        #[max_length = 50]
        barcode -> Nullable<Varchar>,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        nickname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 60]
        password -> Varchar,
        birth_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(default_products -> default_categorys (default_categorys_id));
diesel::joinable!(default_products -> unity_types (unity_types_id));
diesel::joinable!(market -> users (user_id));
diesel::joinable!(product_list -> shopping_list (shopping_list_id));
diesel::joinable!(product_list -> user_products (user_product_id));
diesel::joinable!(shopping_list -> users (user_id));
diesel::joinable!(shopping_log -> market (market_id));
diesel::joinable!(shopping_log -> shopping_list (shopping_list_id));
diesel::joinable!(shopping_log -> users (user_id));
diesel::joinable!(user_categorys -> users (user_id));
diesel::joinable!(user_products -> user_categorys (category_id));
diesel::joinable!(user_products -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    default_categorys,
    default_products,
    market,
    product_list,
    shopping_list,
    shopping_log,
    unity_types,
    user_categorys,
    user_products,
    users,
);
