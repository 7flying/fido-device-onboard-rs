pub mod models;
pub mod schema;

use anyhow::{bail, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::SqliteConnection;
use fdo_data_formats::Serializable;
use schema::ownership_voucher;

use self::models::{NewOwnershipVoucherModel, OwnershipVoucherModel};

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use fdo_data_formats::types::Guid;
//use fdo_util::servers::OwnershipVoucherStoreMetadataKey;

use dotenvy::dotenv;
use std::env;

#[derive(PartialEq, Debug)]
pub enum OVMetadataKey {
    To2Performed,
    To0AcceptOwnerWaitSeconds,
    Ttl,
}

pub trait DBStore {
    //fn connect_to_db() -> dyn Connection;

    ///
    //  fn insert_ov_metadata(key: &OwnershipVoucherStoreMetadataKey, guid: &Guid);

    ///
    fn insert_ov(ov: &OV) -> Result<()>;

    /// Given a GUID find an OV with that key
    fn get_ov(ov_guid: &String) -> Result<OwnershipVoucherModel>;
}

pub trait PostgresStorable: DBStore {
    fn connect_to_db() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is misisng");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }

    //  fn insert_ov_metadata(key: &OwnershipVoucherStoreMetadataKey, guid: &Guid) {}

    fn insert_ov(ov: &OV) -> Result<()> {
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

    // fn get_ov(ov_guid: &String) -> Result<OwnershipVoucherModel> {
    //     let connection = &mut Self::connect_to_db();
    //     ownership_voucher
    //         .find(ov_guid)
    //         .first(connection)
    //         .expect("Error retrieving OV")
    // }
}

pub trait SqliteStorable: DBStore {
    fn connect_to_db() -> SqliteConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }

    fn insert_ov(ov: &OV) -> Result<()> {
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

    fn get_ov_model(guid: &Guid) -> Result<OwnershipVoucherModel> {
        let connection = &mut Self::connect_to_db();
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(schema::ownership_voucher::guid.eq(guid.to_string()))
            .first(connection)?;
        Ok(result)
    }

    fn get_ov(guid: &Guid) -> Result<OV> {
        let connection = &mut Self::connect_to_db();
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(schema::ownership_voucher::guid.eq(guid.to_string()))
            .first(connection)?;
        let ov = OV::from_pem_or_raw(&result.contents)?;
        Ok(ov)
    }

    fn update_ov_metadata_i64(guid: &Guid, key: OVMetadataKey, value: &i64) -> Result<()> {
        let connection = &mut Self::connect_to_db();
        match key {
            OVMetadataKey::To0AcceptOwnerWaitSeconds => {
                diesel::update(ownership_voucher::dsl::ownership_voucher)
                    .filter(schema::ownership_voucher::guid.eq(guid.to_string()))
                    .set(schema::ownership_voucher::to0_accept_owner_wait_seconds.eq(value))
                    .execute(connection)?;
            }
            OVMetadataKey::Ttl => {
                diesel::update(ownership_voucher::dsl::ownership_voucher)
                    .filter(schema::ownership_voucher::guid.eq(guid.to_string()))
                    .set(schema::ownership_voucher::ttl.eq(value))
                    .execute(connection)?;
            }
            _ => bail!("No such metadata key '{key:?}' with i64 type"),
        };
        Ok(())
    }

    fn update_ov_metadata_bool(guid: &Guid, key: OVMetadataKey, value: &bool) -> Result<()> {
        if key != OVMetadataKey::To2Performed {
            bail!("No such metadata key '{key:?}' with bool type");
        }
        let connection = &mut Self::connect_to_db();
        diesel::update(ownership_voucher::dsl::ownership_voucher)
            .filter(schema::ownership_voucher::guid.eq(guid.to_string()))
            .set(schema::ownership_voucher::to2_performed.eq(value))
            .execute(connection)?;
        Ok(())
    }
}
