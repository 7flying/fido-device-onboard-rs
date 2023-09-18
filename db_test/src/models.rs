use diesel::prelude::*;
use std::fmt;

// Queriable will generate the code to load this type of struct from an sql query
// Selectable will generate the code to construct a select based on the model type and on the table that we have referenced with the table_name stuff
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::ownership_voucher)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[primary_key(guid)]
pub struct OwnershipVoucherModel {
    pub guid: String,
    pub contents: Vec<u8>,
    pub to2_performed: Option<bool>,
    pub to0_accept_owner_wait_seconds: Option<i64>,
    pub ttl: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ownership_voucher)]
pub struct NewOwnershipVoucherModel {
    pub guid: String,
    pub contents: Vec<u8>,
    pub to2_performed: Option<bool>,
    pub to0_accept_owner_wait_seconds: Option<i64>,
    pub ttl: Option<i64>,
}

impl fmt::Display for OwnershipVoucherModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GUID: {}, contents: {:?}, to2_performed: {:?}, to0_accepted_owner_wait_seconds: {:?}, ttl: {:?}",
        self.guid, self.contents, self.to2_performed, self.to0_accept_owner_wait_seconds, self.ttl)
    }
}
