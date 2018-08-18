table! {
    orders (id) {
        id -> Integer,
        specification_id -> Integer,
        quantity -> Integer,
    }
}

table! {
    product_orders (id) {
        id -> Integer,
        product_id -> Integer,
        order_id -> Integer,
    }
}

table! {
    products (id) {
        id -> Integer,
        specification_id -> Integer,
    }
}

table! {
    specifications (id) {
        id -> Integer,
        name -> Text,
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
