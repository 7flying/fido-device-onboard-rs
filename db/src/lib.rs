pub mod models;
#[cfg(feature = "postgres")]
pub mod postgres;
pub mod schema;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use self::models::OwnershipVoucherModel;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;

#[derive(PartialEq, Debug)]
pub enum OVMetadataKey {
    To2Performed,
    To0AcceptOwnerWaitSeconds,
    Ttl,
}

pub trait DBStore<T>
where
    T: diesel::r2d2::R2D2Connection + 'static,
{
    /// Gets a connection pool
    fn get_conn_pool() -> Pool<ConnectionManager<T>>;

    /// Gets a connection to the db
    fn get_connection() -> T;

    /// Stores an OwnershipVoucher
    fn insert_ov(ov: &OV, conn: &mut T) -> Result<()>;

    /// Gets an OwnershipVoucherModel from the DB given its Guid
    fn get_ov_model(guid: &String, conn: &mut T) -> Result<OwnershipVoucherModel>;

    /// Gets an OwnshipVoucher from the DB given its Guid
    fn get_ov(guid: &String, conn: &mut T) -> Result<OV>;

    /// Deletes an ownership voucher from the DB given its Guid
    fn delete_ov(guid: &String, conn: &mut T) -> Result<()>;

    /// Updates the value of the given metadata key, value must be i64
    fn update_ov_metadata_i64(
        guid: &String,
        key: OVMetadataKey,
        value: &i64,
        conn: &mut T,
    ) -> Result<()>;

    /// Updates the value of the given metadata key, value must be bool
    fn update_ov_metadata_bool(
        guid: &String,
        key: OVMetadataKey,
        value: &bool,
        conn: &mut T,
    ) -> Result<()>;
}
