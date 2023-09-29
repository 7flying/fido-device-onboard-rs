-- Your SQL goes here

CREATE TABLE ownership_voucher (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents bytea NOT NULL
);

CREATE TABLE manufacturer_vouchers (
    ov_guid references ownership_voucher(guid) PRIMARY KEY ON DELETE CASCADE
);

CREATE TABLE owner_vouchers (
--    id bigserial NOT NULL PRIMARY KEY,
    ov_guid references ownership_voucher(guid) PRIMARY KEY ON DELETE CASCADE,
    to2_performed boolean,
    to0_accept_owner_wait_seconds bigint
);

CREATE TABLE rendezvous_vouchers (
    ov_guid references ownership_voucher(guid) PRIMARY KEY ON DELETE CASCADE,
    ttl bigint
);

