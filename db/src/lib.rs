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

    /// Gets an OwnershipVoucher from the DB given its Guid
    fn get_ov(guid: &String, conn: &mut T) -> Result<OV>;

    /// Deletes an ownership voucher from the DB given its Guid
    fn delete_ov(guid: &String, conn: &mut T) -> Result<()>;

    /// Inserts an OV reference in the rendezvous server table
    fn insert_ov_ref_rv(guid: &String, ttl: Option<i64>, conn: &mut T) -> Result<()>;

    /// Updates the ttl of an OV in the rendezvous server table
    fn update_ov_ttl_metadata_rv(guid: &String, ttl: &i64, conn: &mut T) -> Result<()>;

    /// Deletes an OV from the rendezvous server
    fn delete_ov_rv(guid: &String, conn: &mut T) -> Result<()>;

    /// Inserts an OV reference in the owner onboarding server table
    fn insert_ov_ref_owner(
        guid: &String,
        to2: Option<bool>,
        to0: Option<i64>,
        conn: &mut T,
    ) -> Result<()>;

    #[allow(non_snake_case)]
    /// Updates the to0 metadata of an OV in the owner onboarding server table
    fn update_ov_tO0_metadata_owner(guid: &String, value: &i64, conn: &mut T) -> Result<()>;

    #[allow(non_snake_case)]
    /// Updates the to2 metadata of an OV in the owner onboarding server table
    fn update_ov_tO2_metadata_owner(guid: &String, value: &bool, conn: &mut T) -> Result<()>;
}
