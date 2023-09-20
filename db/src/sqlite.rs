use super::{DBStore, OVMetadataKey};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;

use std::env;

use anyhow::{bail, Result};
use dotenvy::dotenv;

use super::models::{NewOwnershipVoucherModel, OwnershipVoucherModel};
use super::schema::ownership_voucher;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use fdo_data_formats::Serializable;

pub trait SqliteConnectable {
    fn connect_to_db() -> SqliteConnection {
        dotenv().ok();
        let database_url =
            env::var("SQLITE_DATABASE_URL").expect("SQLITE_DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }
}

pub struct SqliteDB {}

impl SqliteConnectable for SqliteDB {}

impl DBStore<SqliteConnection> for SqliteDB {
    fn get_conn_pool() -> Pool<ConnectionManager<SqliteConnection>> {
        dotenv().ok();
        let database_url =
            env::var("SQLITE_DATABASE_URL").expect("SQLITE_DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Coulnd't build db connection pool")
    }

    fn insert_ov(ov: &OV, conn: &mut SqliteConnection) -> Result<()> {
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

    fn get_ov_model(guid: &String, conn: &mut SqliteConnection) -> Result<OwnershipVoucherModel> {
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .first(conn)?;
        Ok(result)
    }

    fn get_ov(guid: &String, conn: &mut SqliteConnection) -> Result<OV> {
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .first(conn)?;
        let ov = OV::from_pem_or_raw(&result.contents)?;
        Ok(ov)
    }

    fn delete_ov(guid: &String, conn: &mut SqliteConnection) -> Result<()> {
        diesel::delete(ownership_voucher::dsl::ownership_voucher)
            .filter(super::schema::ownership_voucher::guid.eq(guid.to_string()))
            .execute(conn)?;
        Ok(())
    }

    fn update_ov_metadata_i64(
        guid: &String,
        key: OVMetadataKey,
        value: &i64,
        conn: &mut SqliteConnection,
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
        conn: &mut SqliteConnection,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_database() -> Result<()> {
        println!("Current directory: {:?}", env::current_dir());

        // read test ovs from the integration tests dir
        let mut ov_map = HashMap::new();
        let pool = SqliteDB::get_conn_pool();

        let mut last_raw_contents: Vec<u8> = vec![];
        let mut last_guid = "".to_string();
        for path in std::fs::read_dir("../integration-tests/vouchers/v101").expect("Dir not found")
        {
            let ov_path = path.expect("error getting path").path();
            let content = std::fs::read(ov_path).expect("OV couldn't be read");
            last_raw_contents = content.clone();
            let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
            last_guid = ov.header().guid().to_string();
            ov_map.insert(ov.header().guid().to_string(), ov);
        }

        // get a connection from the pool
        let conn = &mut pool.get().unwrap();
        // store ovs in the database
        for (_, ov) in ov_map.clone().into_iter() {
            SqliteDB::insert_ov(&ov, conn)?;
        }
        // we should have 3 ovs
        let count: i64 = ownership_voucher::dsl::ownership_voucher
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 3);

        // add some metadata for the ovs
        for (guid, _) in ov_map.into_iter() {
            SqliteDB::update_ov_metadata_i64(
                &guid,
                OVMetadataKey::To0AcceptOwnerWaitSeconds,
                &(2000 as i64),
                conn,
            )?;
            SqliteDB::update_ov_metadata_bool(&guid, OVMetadataKey::To2Performed, &true, conn)?;
        }

        // this should error since the key needs a bool value
        assert!(SqliteDB::update_ov_metadata_i64(
            &last_guid,
            OVMetadataKey::To2Performed,
            &(2000 as i64),
            conn,
        )
        .is_err());

        let ov_model = SqliteDB::get_ov_model(&last_guid, conn)?;
        //let ov_format = SqliteDB::get_ov(&last_guid, conn)?;
        assert_eq!(ov_model.contents, last_raw_contents);

        // delete an ov, we should have 2
        SqliteDB::delete_ov(&last_guid, conn)?;
        let count: i64 = ownership_voucher::dsl::ownership_voucher
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 2);

        Ok(())
    }
}
