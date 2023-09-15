// @generated automatically by Diesel CLI.

diesel::table! {
    manufacturing_sessions (id) {
        id -> Integer,
        session_id -> Integer,
    }
}

diesel::table! {
    owner_sessions (id) {
        id -> Integer,
        session_id -> Integer,
    }
}

diesel::table! {
    ownership_voucher (guid) {
        guid -> Text,
        contents -> Binary,
        to2_performed -> Nullable<Bool>,
        to0_accept_owner_wait_seconds -> Nullable<BigInt>,
        ttl -> Nullable<BigInt>,
    }
}

diesel::table! {
    rendezvous_sessions (id) {
        id -> Integer,
        session_id -> Integer,
    }
}

diesel::table! {
    rv_item (guid) {
        guid -> Text,
        public_key -> Binary,
        to1d -> Binary,
    }
}

diesel::table! {
    sessions (session_id) {
        session_id -> Text,
        contents -> Binary,
        ttl_metadata -> Nullable<BigInt>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    manufacturing_sessions,
    owner_sessions,
    ownership_voucher,
    rendezvous_sessions,
    rv_item,
    sessions,
);
