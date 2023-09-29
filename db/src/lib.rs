pub mod models;
#[cfg(feature = "postgres")]
pub mod postgres;
pub mod schema;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use models::OwnerOV;

pub trait DBStoreManufacturer<T>
where
    T: diesel::r2d2::R2D2Connection + 'static,
{
    /// Gets a connection pool
    fn get_conn_pool() -> Pool<ConnectionManager<T>>;

    /// Gets a connection to the db
    fn get_connection() -> T;

    /// Inserts an OV
    fn insert_ov(ov: &OV, ttl: Option<i64>, conn: &mut T) -> Result<()>;

    /// Deletes an OV
    fn delete_ov(guid: &str, conn: &mut T) -> Result<()>;

    /// Deletes all OVs whose ttl is less or equal to the given ttl
    fn delete_ov_ttl_le(ttl: i64, conn: &mut T) -> Result<()>;
}

pub trait DBStoreOwner<T>
where
    T: diesel::r2d2::R2D2Connection + 'static,
{
    /// Gets a connection pool
    fn get_conn_pool() -> Pool<ConnectionManager<T>>;

    /// Gets a connection to the db
    fn get_connection() -> T;

    /// Inserts an OV
    fn insert_ov(ov: &OV, to2: Option<bool>, to0: Option<i64>, conn: &mut T) -> Result<()>;

    /// Deletes an OV
    fn delete_ov(guid: &str, conn: &mut T) -> Result<()>;

    /// Selects all the OVs with the given to2_performed status
    fn select_ov_to2_performed(to2_performed: bool, conn: &mut T) -> Result<Vec<OwnerOV>>;

    /// Selects all the OVs whose to0 is less than the given maximum
    fn select_ov_to0_less_than(to0_max: i64, conn: &mut T) -> Result<Vec<OwnerOV>>;
}

pub trait DBStoreRendezvous<T>
where
    T: diesel::r2d2::R2D2Connection + 'static,
{
    /// Gets a connection pool
    fn get_conn_pool() -> Pool<ConnectionManager<T>>;

    /// Gets a connection to the db
    fn get_connection() -> T;

    /// Inserts an OV
    fn insert_ov(ov: &OV, ttl: Option<i64>, conn: &mut T) -> Result<()>;

    /// Deletes an OV
    fn delete_ov(guid: &str, conn: &mut T) -> Result<()>;

    /// Deletes all OVs whose ttl is less or equal to the given ttl
    fn delete_ov_ttl_le(ttl: i64, conn: &mut T) -> Result<()>;
}
