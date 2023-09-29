use crate::schema::rendezvous_vouchers;

use super::DBStore;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;

use std::env;

use anyhow::Result;
use dotenvy::dotenv;

use super::models::{NewOwnerOV, NewOwnershipVoucherModel, NewRendezvousOV, OwnershipVoucherModel};
use super::schema::ownership_voucher;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use fdo_data_formats::Serializable;

pub struct SqliteDB {}

impl DBStore<SqliteConnection> for SqliteDB {
    fn get_connection() -> SqliteConnection {
        dotenv().ok();
        let database_url =
            env::var("SQLITE_DATABASE_URL").expect("SQLITE_DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
    }

    fn get_conn_pool() -> Pool<ConnectionManager<SqliteConnection>> {
        dotenv().ok();
        let database_url =
            env::var("SQLITE_DATABASE_URL").expect("SQLITE_DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Couldn't build db connection pool")
    }

    fn insert_ov(ov: &OV, conn: &mut SqliteConnection) -> Result<()> {
        let new_ov_model = NewOwnershipVoucherModel {
            guid: ov.header().guid().to_string(),
            contents: ov.serialize_data().expect("Error serializing OV"),
        };
        diesel::insert_into(super::schema::ownership_voucher::table)
            .values(&new_ov_model)
            .execute(conn)
            .expect(&format!("Error saving OV {:?}", new_ov_model.guid));
        Ok(())
    }

    fn get_ov_model(guid: &String, conn: &mut SqliteConnection) -> Result<OwnershipVoucherModel> {
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(super::schema::ownership_voucher::guid.eq(guid))
            .first(conn)?;
        Ok(result)
    }

    fn get_ov(guid: &String, conn: &mut SqliteConnection) -> Result<OV> {
        let result = ownership_voucher::dsl::ownership_voucher
            .select(OwnershipVoucherModel::as_select())
            .filter(super::schema::ownership_voucher::guid.eq(guid))
            .first(conn)?;
        let ov = OV::from_pem_or_raw(&result.contents)?;
        Ok(ov)
    }

    fn delete_ov(guid: &String, conn: &mut SqliteConnection) -> Result<()> {
        diesel::delete(ownership_voucher::dsl::ownership_voucher)
            .filter(super::schema::ownership_voucher::guid.eq(guid))
            .execute(conn)?;
        Ok(())
    }

    fn insert_ov_ref_rv(
        guid: &String,
        ttl: Option<i64>,
        conn: &mut SqliteConnection,
    ) -> Result<()> {
        let new_rv_ov = NewRendezvousOV {
            ov_guid: guid.to_owned(),
            ttl,
        };
        diesel::insert_into(super::schema::rendezvous_vouchers::table)
            .values(&new_rv_ov)
            .execute(conn)?;
        Ok(())
    }

    fn update_ov_ttl_metadata_rv(
        guid: &String,
        value: &i64,
        conn: &mut SqliteConnection,
    ) -> Result<()> {
        diesel::update(super::schema::rendezvous_vouchers::dsl::rendezvous_vouchers)
            .filter(super::schema::rendezvous_vouchers::ov_guid.eq(guid))
            .set(super::schema::rendezvous_vouchers::ttl.eq(value))
            .execute(conn)?;
        Ok(())
    }

    fn insert_ov_ref_owner(
        guid: &String,
        to2: Option<bool>,
        to0: Option<i64>,
        conn: &mut SqliteConnection,
    ) -> Result<()> {
        let new_owner_ov = NewOwnerOV {
            ov_guid: guid.to_owned(),
            to0_accept_owner_wait_seconds: to0,
            to2_performed: to2,
        };
        diesel::insert_into(super::schema::owner_vouchers::table)
            .values(&new_owner_ov)
            .execute(conn)?;
        Ok(())
    }

    #[allow(non_snake_case)]
    fn update_ov_tO0_metadata_owner(
        guid: &String,
        value: &i64,
        conn: &mut SqliteConnection,
    ) -> Result<()> {
        diesel::update(super::schema::owner_vouchers::dsl::owner_vouchers)
            .filter(super::schema::owner_vouchers::ov_guid.eq(guid))
            .set(super::schema::owner_vouchers::to0_accept_owner_wait_seconds.eq(value))
            .execute(conn)?;
        Ok(())
    }

    #[allow(non_snake_case)]
    fn update_ov_tO2_metadata_owner(
        guid: &String,
        value: &bool,
        conn: &mut SqliteConnection,
    ) -> Result<()> {
        diesel::update(super::schema::owner_vouchers::dsl::owner_vouchers)
            .filter(super::schema::owner_vouchers::ov_guid.eq(guid))
            .set(super::schema::owner_vouchers::to2_performed.eq(value))
            .execute(conn)?;
        Ok(())
    }

    fn delete_ov_rv(guid: &String, conn: &mut SqliteConnection) -> Result<()> {
        diesel::delete(rendezvous_vouchers::dsl::rendezvous_vouchers)
            .filter(super::schema::rendezvous_vouchers::ov_guid.eq(guid))
            .execute(conn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteDB;
    use crate::{schema::*, DBStore};
    use anyhow::Result;
    use diesel::connection::SimpleConnection;
    use diesel::prelude::*;
    use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
    use std::collections::HashMap;
    use std::env;

    #[test]
    fn test_database() -> Result<()> {
        println!("Current directory: {:?}", env::current_dir());

        // read test ovs from the integration tests dir
        let mut ov_map = HashMap::new();
        let pool = SqliteDB::get_conn_pool();

        let mut last_guid = String::new();
        for path in std::fs::read_dir("../integration-tests/vouchers/v101").expect("Dir not found")
        {
            let ov_path = path.expect("error getting path").path();
            let content = std::fs::read(ov_path).expect("OV couldn't be read");
            let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
            last_guid = ov.header().guid().to_string();
            ov_map.insert(ov.header().guid().to_string(), ov);
        }

        // get a connection from the pool
        let conn = &mut pool.get().unwrap();
        // sqlite does not enable this by default...
        conn.batch_execute("PRAGMA foreign_keys = ON")?;

        // store ovs in the database and add them to the owner and rendezvous
        for (guid, ov) in ov_map.clone().into_iter() {
            SqliteDB::insert_ov(&ov, conn)?;
            SqliteDB::insert_ov_ref_owner(&guid, None, None, conn)?;
            SqliteDB::insert_ov_ref_rv(&guid, None, conn)?;
        }

        // we should have 3 ovs in each table
        let count: i64 = ownership_voucher::dsl::ownership_voucher
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 3);

        let count: i64 = owner_vouchers::dsl::owner_vouchers
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 3);

        let count: i64 = rendezvous_vouchers::dsl::rendezvous_vouchers
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 3);

        // add some metadata for the ovs
        for (guid, _) in ov_map.clone().into_iter() {
            SqliteDB::update_ov_ttl_metadata_rv(&guid, &(2000 as i64), conn)?;
            SqliteDB::update_ov_tO0_metadata_owner(&guid, &(2500 as i64), conn)?;
            SqliteDB::update_ov_tO2_metadata_owner(&guid, &true, conn)?;
        }

        // this should error since there is no OV with that guid created
        assert!(
            SqliteDB::insert_ov_ref_owner(&"non-existing-GUID".to_string(), None, None, conn)
                .is_err()
        );

        // delete an ov, we should have 2
        SqliteDB::delete_ov(&last_guid.to_string(), conn)?;
        let count: i64 = ownership_voucher::dsl::ownership_voucher
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 2);

        // ...and the on cascade should work for the other tables
        let count: i64 = owner_vouchers::dsl::owner_vouchers
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 2);

        let count: i64 = rendezvous_vouchers::dsl::rendezvous_vouchers
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count, 2);

        Ok(())
    }
}
