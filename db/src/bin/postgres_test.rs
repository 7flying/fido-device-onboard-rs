use db::postgres::{PostgresManufacturerDB, PostgresOwnerDB, PostgresRendezvousDB};
use db::schema::manufacturer_vouchers;
use db::schema::owner_vouchers;
use db::schema::rendezvous_vouchers;
use db::{DBStoreManufacturer, DBStoreOwner, DBStoreRendezvous};
use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;

use anyhow::Result;
use diesel::prelude::*;
use std::collections::HashMap;
use std::env;

fn test_manufacturer_database() -> Result<()> {
    println!("Current directory: {:?}", env::current_dir());

    // read test ovs from the integration tests dir
    let mut ov_map = HashMap::new();
    let pool = PostgresManufacturerDB::get_conn_pool();

    // last_guid used later to delete an ov with that key
    let mut last_guid = String::new();
    for path in std::fs::read_dir("../../integration-tests/vouchers/v101").expect("Dir not found") {
        let ov_path = path.expect("error getting path").path();
        let content = std::fs::read(ov_path).expect("OV couldn't be read");
        let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
        last_guid = ov.header().guid().to_string();
        ov_map.insert(ov.header().guid().to_string(), ov);
    }

    // get a connection from the pool
    let conn = &mut pool.get().unwrap();

    for (_, ov) in ov_map.clone().into_iter() {
        PostgresManufacturerDB::insert_ov(&ov, Some(5000_i64), conn)?;
    }

    // we should have 3 ovs
    let count: i64 = manufacturer_vouchers::dsl::manufacturer_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 3);

    // delete an ov by guid, we should have 2 at the end
    PostgresManufacturerDB::delete_ov(&last_guid, conn)?;
    let count: i64 = manufacturer_vouchers::dsl::manufacturer_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 2);

    // delete manufacturer ovs with ttl <= 4000, we shouldn't delete any of them
    PostgresManufacturerDB::delete_ov_ttl_le(4000_i64, conn)?;
    let count: i64 = manufacturer_vouchers::dsl::manufacturer_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 2);

    // delete manufacturer ovs with ttl <= 5000, we should delete the remaining 2 ovs
    PostgresManufacturerDB::delete_ov_ttl_le(5000_i64, conn)?;
    let count: i64 = manufacturer_vouchers::dsl::manufacturer_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 0);
    Ok(())
}

fn test_owner_database() -> Result<()> {
    println!("Current directory: {:?}", env::current_dir());

    // read test ovs from the integration tests dir
    let mut ov_map = HashMap::new();
    let pool = PostgresOwnerDB::get_conn_pool();

    // last_guid used later to delete an ov with that key
    let mut last_guid = String::new();
    for path in std::fs::read_dir("../../integration-tests/vouchers/v101").expect("Dir not found") {
        let ov_path = path.expect("error getting path").path();
        let content = std::fs::read(ov_path).expect("OV couldn't be read");
        let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
        last_guid = ov.header().guid().to_string();
        ov_map.insert(ov.header().guid().to_string(), ov);
    }

    // get a connection from the pool
    let conn = &mut pool.get().unwrap();

    let mut to2_done = true;
    for (_, ov) in ov_map.clone().into_iter() {
        if to2_done {
            PostgresOwnerDB::insert_ov(&ov, Some(to2_done), Some(2000_i64), conn)?;
        } else {
            PostgresOwnerDB::insert_ov(&ov, Some(to2_done), Some(3000_i64), conn)?;
        }
        to2_done = !to2_done;
    }

    // we should have 3 ovs
    let count: i64 = owner_vouchers::dsl::owner_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 3);

    // select the owner ovs with to2 performed = true, we should have 2
    let result = PostgresOwnerDB::select_ov_to2_performed(true, conn)?;
    assert_eq!(result.len(), 2);

    // select the owner ovs with to0 less than 2500, we should have 2
    let result = PostgresOwnerDB::select_ov_to0_less_than(2500_i64, conn)?;
    assert_eq!(result.len(), 2);

    // delete an ov from the owner, we should have 2 left
    PostgresOwnerDB::delete_ov(&last_guid.to_string(), conn)?;
    let count: i64 = owner_vouchers::dsl::owner_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 2);

    Ok(())
}

fn test_rendezvous_database() -> Result<()> {
    println!("Current directory: {:?}", env::current_dir());

    // read test ovs from the integration tests dir
    let mut ov_map = HashMap::new();
    let pool = PostgresRendezvousDB::get_conn_pool();

    // last_guid used later to delete an ov with that key
    let mut last_guid = String::new();
    for path in std::fs::read_dir("../../integration-tests/vouchers/v101").expect("Dir not found") {
        let ov_path = path.expect("error getting path").path();
        let content = std::fs::read(ov_path).expect("OV couldn't be read");
        let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
        last_guid = ov.header().guid().to_string();
        ov_map.insert(ov.header().guid().to_string(), ov);
    }

    // get a connection from the pool
    let conn = &mut pool.get().unwrap();

    for (_, ov) in ov_map.clone().into_iter() {
        PostgresRendezvousDB::insert_ov(&ov, Some(5000_i64), conn)?;
    }

    // we should have 3 ovs
    let count: i64 = rendezvous_vouchers::dsl::rendezvous_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 3);

    // delete an ov by guid, we should have 2 at the end
    PostgresRendezvousDB::delete_ov(&last_guid, conn)?;
    let count: i64 = rendezvous_vouchers::dsl::rendezvous_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 2);

    // delete rendezvous ovs with ttl <= 4000, we shouldn't delete any of them
    PostgresRendezvousDB::delete_ov_ttl_le(4000_i64, conn)?;
    let count: i64 = rendezvous_vouchers::dsl::rendezvous_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 2);

    // delete rendezvous ovs with ttl <= 5000, we should delete the remaining 2 ovs
    PostgresRendezvousDB::delete_ov_ttl_le(5000_i64, conn)?;
    let count: i64 = rendezvous_vouchers::dsl::rendezvous_vouchers
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 0);
    Ok(())
}

fn main() {
    test_manufacturer_database().expect("Manufacturer tests failed");
    test_owner_database().expect("Owner tests failed");
    test_rendezvous_database().expect("Rendezvous tests failed");
}
