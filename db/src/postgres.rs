use super::{DBStore, OVMetadataKey};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

use std::env;

use anyhow::{bail, Result};
use dotenvy::dotenv;

use super::models::{NewOwnershipVoucherModel, OwnershipVoucherModel};
use super::schema::ownership_voucher;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use fdo_data_formats::Serializable;

pub struct PostgresDB {}

impl DBStore<PgConnection> for PostgresDB {
    fn get_connection() -> PgConnection {
        dotenv().ok();
        let database_url =
            env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }

    fn get_conn_pool() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();
        let database_url =
            env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .test_on_check_out(true)
            .min_idle(Some(2))
            .build(manager)
            .expect("Coulnd't build db connection pool")
    }

    fn insert_ov(ov: &OV, conn: &mut PgConnection) -> Result<()> {
        let new_ov_model = NewOwnershipVoucherModel {
            guid: ov.header().guid().to_string(),
            contents: ov.serialize_data().expect("Error serializing OV"),
            to2_performed: None,
            to0_accept_owner_wait_seconds: None,
            ttl: None,
        };
        diesel::insert_into(super::schema::ownership_voucher::table)
            .values(&new_ov_model)
            .execute(conn)
            .expect("Error saving OV");
        Ok(())
    }

    fn get_ov_model(guid: &String, conn: &mut PgConnection) -> Result<OwnershipVoucherModel> {
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .first(conn)?;
        Ok(result)
    }

    fn get_ov(guid: &String, conn: &mut PgConnection) -> Result<OV> {
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .first(conn)?;
        let ov = OV::from_pem_or_raw(&result.contents)?;
        Ok(ov)
    }

    fn delete_ov(guid: &String, conn: &mut PgConnection) -> Result<()> {
        diesel::delete(ownership_voucher::dsl::ownership_voucher)
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .execute(conn)?;
        Ok(())
    }

    fn update_ov_metadata_i64(
        guid: &String,
        key: OVMetadataKey,
        value: &i64,
        conn: &mut PgConnection,
    ) -> Result<()> {
        match key {
            OVMetadataKey::To0AcceptOwnerWaitSeconds => {
                diesel::update(ownership_voucher::dsl::ownership_voucher)
                    .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
                    .set(super::schema::ownership_voucher::to0_accept_owner_wait_seconds.eq(value))
                    .execute(conn)?;
            }
            OVMetadataKey::Ttl => {
                diesel::update(ownership_voucher::dsl::ownership_voucher)
                    .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
                    .set(super::schema::ownership_voucher::ttl.eq(value))
                    .execute(conn)?;
            }
            _ => bail!("No such metadata key '{key:?}' with i64 type"),
        };
        Ok(())
    }

    fn update_ov_metadata_bool(
        guid: &String,
        key: OVMetadataKey,
        value: &bool,
        conn: &mut PgConnection,
    ) -> Result<()> {
        if key != OVMetadataKey::To2Performed {
            bail!("No such metadata key '{key:?}' with bool type");
        }
        diesel::update(ownership_voucher::dsl::ownership_voucher)
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .set(super::schema::ownership_voucher::to2_performed.eq(value))
            .execute(conn)?;
        Ok(())
    }
}
