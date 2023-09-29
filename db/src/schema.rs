// @generated automatically by Diesel CLI.

diesel::table! {
    manufacturer_vouchers (ov_guid) {
        ov_guid -> Text,
    }
}

diesel::table! {
    owner_vouchers (ov_guid) {
        ov_guid -> Text,
        to2_performed -> Nullable<Bool>,
        to0_accept_owner_wait_seconds -> Nullable<BigInt>,
    }
}

diesel::table! {
    ownership_voucher (guid) {
        guid -> Text,
        contents -> Binary,
    }
}

diesel::table! {
    rendezvous_vouchers (ov_guid) {
        ov_guid -> Text,
        ttl -> Nullable<BigInt>,
    }
}

diesel::joinable!(manufacturer_vouchers -> ownership_voucher (ov_guid));
diesel::joinable!(owner_vouchers -> ownership_voucher (ov_guid));
diesel::joinable!(rendezvous_vouchers -> ownership_voucher (ov_guid));

diesel::allow_tables_to_appear_in_same_query!(
    manufacturer_vouchers,
    owner_vouchers,
    ownership_voucher,
    rendezvous_vouchers,
);
