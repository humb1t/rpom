table! {
    orders (id) {
        id -> Integer,
        specification_id -> Integer,
        quantity -> Integer,
    }
}

table! {
    specifications (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    orders,
    specifications,
);
