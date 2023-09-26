-- Your SQL goes here

CREATE TABLE ownership_voucher (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents bytea NOT NULL,
    to2_performed boolean,
    to0_accept_owner_wait_seconds bigint,
    ttl bigint
);



