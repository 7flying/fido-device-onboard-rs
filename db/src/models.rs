use diesel::prelude::*;
use std::fmt;

// Queriable will generate the code to load this type of struct from an sql query
// Selectable will generate the code to construct a select based on the model type and on the table that we have referenced with the table_name stuff
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::ownership_voucher)]
#[diesel(primary_key(guid))]
pub struct OwnershipVoucherModel {
    pub guid: String,
    pub contents: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ownership_voucher)]
pub struct NewOwnershipVoucherModel {
    pub guid: String,
    pub contents: Vec<u8>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::rendezvous_vouchers)]
#[diesel(belongs_to(OwnershipVoucherModel, foreign_key=ov_guid))]
#[diesel(primary_key(ov_guid))]
pub struct RendezvousOV {
    pub ov_guid: String,
    pub ttl: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rendezvous_vouchers)]
pub struct NewRendezvousOV {
    pub ov_guid: String,
    pub ttl: Option<i64>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::owner_vouchers)]
#[diesel(belongs_to(OwnershipVoucherModel, foreign_key=ov_guid))]
#[diesel(primary_key(ov_guid))]
pub struct OwnerOV {
    pub ov_guid: String,
    pub to2_performed: Option<bool>,
    pub to0_accept_owner_wait_seconds: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::owner_vouchers)]
pub struct NewOwnerOV {
    pub ov_guid: String,
    pub to2_performed: Option<bool>,
    pub to0_accept_owner_wait_seconds: Option<i64>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::manufacturer_vouchers)]
#[diesel(belongs_to(OwnershipVoucherModel, foreign_key=ov_guid))]
#[diesel(primary_key(ov_guid))]
pub struct ManufacturerOV {
    pub ov_guid: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::manufacturer_vouchers)]
pub struct NewManufacturerOV {
    pub ov_guid: String,
}

impl fmt::Display for OwnershipVoucherModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GUID: {}, contents: {:?}", self.guid, self.contents)
    }
}
