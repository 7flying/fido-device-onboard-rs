use diesel::prelude::*;

// Queriable will generate the code to load this type of struct from an sql query
// Selectable will generate the code to construct a select based on the model type and on the table that we have referenced with the table_name stuff
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::ownership_voucher)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct OwnershipVoucherModel {
    pub guid: String,
    pub contents: Vec<u8>,
    pub to2_performed: Option<bool>,
    pub to0_accepted_owner_wait_seconds: Option<i64>,
}
