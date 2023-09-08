-- Your SQL goes here
    
CREATE TABLE ov_metadata (
    id integer PRIMARY KEY,
    to2Performed bool,
    to0AcceptedOwnerWaitSeconds bigint
);
    
CREATE TABLE ov (
    guid varchar(36) PRIMARY KEY,
    contents bytea,
    metadata_id integer REFERENCES ov_metadata (id)
);  

CREATE TABLE rv_item(
    guid varchar(36) PRIMARY KEY,
    public_key bytea,
    to1d bytea
);

CREATE TABLE sessions (
    session_id varchar(70) PRIMARY KEY,
    contents bytea,
    ttl_metadata bigint
);

CREATE TABLE manufacturing_sessions (
    id integer PRIMARY KEY,
    session_id integer REFERENCES sessions (id) NOT NULL
);

CREATE TABLE owner_sessions (
    id integer PRIMARY KEY,
    session_id integer REFERENCES sessions (id) NOT NULL
);

CREATE TABLE rendezvous_sessions (
    id integer PRIMARY KEY, 
    session_id integer REFERENCES sessions (id) NOT NULL
);



