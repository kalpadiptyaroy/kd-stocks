// @generated automatically by Diesel CLI.

diesel::table! {
    price (id) {
        id -> Integer,
        stock_id -> Integer,
        transaction_date -> Datetime,
        open -> Float,
        high -> Float,
        low -> Float,
        close -> Float,
        volume -> Integer,
    }
}

diesel::table! {
    stock (id) {
        id -> Integer,
        #[max_length = 20]
        nse_symbol -> Varchar,
        #[max_length = 20]
        bse_symbol -> Varchar,
        #[max_length = 75]
        name -> Varchar,
    }
}

diesel::joinable!(price -> stock (stock_id));

diesel::allow_tables_to_appear_in_same_query!(
    price,
    stock,
);
