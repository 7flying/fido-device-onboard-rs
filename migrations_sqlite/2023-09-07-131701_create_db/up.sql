-- Your SQL goes here

CREATE TABLE ownership_voucher (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents blob NOT NULL
);

CREATE TABLE manufacturer_vouchers (
    ov_guid varchar(36) NOT NULL PRIMARY KEY, 
    FOREIGN KEY (ov_guid) REFERENCES ownership_voucher(guid) ON DELETE CASCADE
);

CREATE TABLE owner_vouchers (
    ov_guid varchar(36) NOT NULL PRIMARY KEY,
    to2_performed bool,
    to0_accept_owner_wait_seconds bigint,
    FOREIGN KEY (ov_guid) REFERENCES ownership_voucher(guid) ON DELETE CASCADE
);

CREATE TABLE rendezvous_vouchers (
    ov_guid varchar(36) NOT NULL PRIMARY KEY,
    ttl bigint,
    FOREIGN KEY (ov_guid) REFERENCES ownership_voucher(guid) ON DELETE CASCADE
);
