-- Your SQL goes here

CREATE TABLE manufacturer_vouchers (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents bytea NOT NULL,
    ttl bigint
);
