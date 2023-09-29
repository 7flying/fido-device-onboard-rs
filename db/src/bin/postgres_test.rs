use anyhow::Result;
use db::postgres::PostgresDB;
use db::{schema::*, DBStore};
use diesel::prelude::*;
use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use std::collections::HashMap;
use std::env;

fn main() -> Result<()> {
    println!("Current directory: {:?}", env::current_dir());

    // read test ovs from the integration tests dir
    let mut ov_map = HashMap::new();
    let pool = PostgresDB::get_conn_pool();

    let mut last_guid = String::new();
    for path in std::fs::read_dir("./integration-tests/vouchers/v101").expect("Dir not found") {
        let ov_path = path.expect("error getting path").path();
        let content = std::fs::read(ov_path).expect("OV couldn't be read");
        let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
        last_guid = ov.header().guid().to_string();
        ov_map.insert(ov.header().guid().to_string(), ov);
    }

    // get a connection from the pool
    let conn = &mut pool.get().unwrap();

    // store ovs in the database and add them to the owner and rendezvous
    for (guid, ov) in ov_map.clone().into_iter() {
        PostgresDB::insert_ov(&ov, conn)?;
        PostgresDB::insert_ov_ref_owner(&guid, None, None, conn)?;
        PostgresDB::insert_ov_ref_rv(&guid, None, conn)?;
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
        PostgresDB::update_ov_ttl_metadata_rv(&guid, &(2000 as i64), conn)?;
        PostgresDB::update_ov_tO0_metadata_owner(&guid, &(2500 as i64), conn)?;
        PostgresDB::update_ov_tO2_metadata_owner(&guid, &true, conn)?;
    }

    // this should error since there is no OV with that guid created
    assert!(
        PostgresDB::insert_ov_ref_owner(&"non-existing-GUID".to_string(), None, None, conn)
            .is_err()
    );

    // delete an ov, we should have 2
    PostgresDB::delete_ov(&last_guid, conn)?;
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
