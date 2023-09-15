pub mod models;
pub mod schema;

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::SqliteConnection;
use fdo_data_formats::Serializable;

use self::models::{NewOwnershipVoucherModel, OwnershipVoucherModel};
use self::schema::ownership_voucher::dsl::ownership_voucher;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use fdo_data_formats::types::Guid;
use fdo_util::servers::OwnershipVoucherStoreMetadataKey;

use dotenvy::dotenv;
use std::env;

pub trait DBStore {
    //fn connect_to_db() -> dyn Connection;

    fn store_ov_metadata(key: &OwnershipVoucherStoreMetadataKey, guid: &Guid);

    fn store_ov(ov: &OV) -> Result<()>;
    /// Given a GUID find an OV with that key
    fn get_ov(ov_guid: &String); // -> OwnershipVoucherModel;
}

pub trait PostgresStorable: DBStore {
    fn connect_to_db() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is misisng");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }

    fn store_ov(ov: &OV) -> Result<()> {
        let new_ov_model = NewOwnershipVoucherModel {
            guid: ov.header().guid().to_string(),
            contents: ov.serialize_data().expect("Error serializing OV"),
            to2_performed: None,
            to0_accept_owner_wait_seconds: None,
            ttl: None,
        };
        let conn = &mut Self::connect_to_db();
        diesel::insert_into(schema::ownership_voucher::table)
            .values(&new_ov_model)
            .execute(conn)
            .expect("Error saving OV");
        Ok(())
    }

    /// Given a GUID find an OV with that key
    fn get_ov(ov_guid: &String) -> Result<OwnershipVoucherModel> {
        let connection = &mut Self::connect_to_db();
        ownership_voucher
            .find(ov_guid)
            .first(connection)
            .expect("Error retrieving OV")
    }
}

pub trait SqliteStorable: DBStore {
    fn connect_to_db() -> SqliteConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }
}
