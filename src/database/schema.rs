// @generated automatically by Diesel CLI.

diesel::table! {
    server_variables (id) {
        id -> Integer,
        server_id -> BigInt,
    }
}

diesel::table! {
    servers (id) {
        id -> Integer,
        server_id -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(server_variables, servers,);
