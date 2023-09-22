-- Your SQL goes here

CREATE TABLE ownership_voucher (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents blob NOT NULL,
    to2_performed bool,
    to0_accept_owner_wait_seconds bigint,
    ttl bigint
);


