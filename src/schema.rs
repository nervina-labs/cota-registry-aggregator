table! {
    register_cota_kv_pairs (id) {
        id -> Bigint,
        block_number -> Unsigned<Bigint>,
        lock_hash -> Char,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    schema_migrations (version) {
        version -> Bigint,
        dirty -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(register_cota_kv_pairs, schema_migrations,);
