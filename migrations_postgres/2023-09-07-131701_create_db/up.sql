-- Your SQL goes here

CREATE TABLE ownership_voucher (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents bytea NOT NULL
);

CREATE TABLE manufacturer_vouchers (
    ov_guid varchar(36) PRIMARY KEY references ownership_voucher(guid) ON DELETE CASCADE
);

-- CREATE TABLE owner_vouchers (
--     ov_guid varchar(36) PRIMARY KEY references ownership_voucher(guid) ON DELETE CASCADE,
--     to2_performed boolean,
--     to0_accept_owner_wait_seconds bigint
-- );

-- CREATE TABLE rendezvous_vouchers (
--     ov_guid varchar(36) PRIMARY KEY references ownership_voucher(guid) ON DELETE CASCADE,
--     ttl bigint
-- );

