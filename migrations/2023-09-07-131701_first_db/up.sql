-- Your SQL goes here

-- replacing tytea with blob
        
-- CREATE TABLE ov_metadata (
--     id integer PRIMARY KEY,
--     to2Performed bool,
--     to0AcceptedOwnerWaitSeconds bigint
-- );

CREATE TABLE ownership_voucher (
    guid varchar(36) NOT NULL PRIMARY KEY,
    contents blob NOT NULL,
    to2_performed bool,
    to0_accepted_owner_wait_seconds bigint
    --metadata_id integer REFERENCES ov_metadata (id)
);

CREATE TABLE rv_item(
    guid varchar(36) NOT NULL PRIMARY KEY,
    public_key blob NOT NULL,
    to1d blob NOT NULL
);

CREATE TABLE sessions (
    session_id varchar(70) NOT NULL PRIMARY KEY,
    contents blob NOT NULL,
    ttl_metadata bigint
);

CREATE TABLE manufacturing_sessions (
    id integer NOT NULL PRIMARY KEY,
    --session_id integer REFERENCES sessions (id) NOT NULL
    session_id integer NOT NULL,
    FOREIGN KEY(session_id) REFERENCES sessions(id)
);

CREATE TABLE owner_sessions (
    id integer NOT NULL PRIMARY KEY,
    session_id integer REFERENCES sessions (id) NOT NULL
);

CREATE TABLE rendezvous_sessions (
    id integer NOT NULL PRIMARY KEY, 
    session_id integer REFERENCES sessions (id) NOT NULL
);



