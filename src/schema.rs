table! {
    use diesel::sql_types::*;
    use order::Order_status;

    orders (id) {
        id -> Int4,
        specification_id -> Int4,
        quantity -> Int4,
        status -> Order_status,
    }
}

table! {
    use diesel::sql_types::*;
    use order::Order_status;

    product_orders (id) {
        id -> Int4,
        product_id -> Int4,
        order_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use order::Order_status;

    products (id) {
        id -> Int4,
        specification_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use order::Order_status;

    specifications (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(product_orders -> orders (order_id));
joinable!(product_orders -> products (product_id));
joinable!(products -> specifications (specification_id));

allow_tables_to_appear_in_same_query!(
    orders,
    product_orders,
    products,
    specifications,
);
