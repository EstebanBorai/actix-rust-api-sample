table! {
    rental (id) {
        id -> Uuid,
        building_type -> Text,
        price -> Numeric,
        year_of_construction -> Int2,
        address -> Text,
        bedrooms -> Int2,
        bathrooms -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    rental,
    user,
);
