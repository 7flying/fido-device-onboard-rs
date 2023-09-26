use std::env;
use diesel::prelude::*;
use anyhow::Result;
//use dotenvy::dotenv;

use db::{schema::ownership_voucher, DBStore, OVMetadataKey};
use std::collections::HashMap;

use db::postgres::PostgresDB;

use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;

fn main() -> Result<()>{
    println!("Current directory: {:?}", env::current_dir());

    // read test ovs from the integration tests dir
    let mut ov_map = HashMap::new();
    let pool = PostgresDB::get_conn_pool();

    let mut last_guid = "".to_string();
    for path in std::fs::read_dir("./integration-tests/vouchers/v101").expect("Dir not found") {
        let ov_path = path.expect("error getting path").path();
        let content = std::fs::read(ov_path).expect("OV couldn't be read");
        let ov = OV::from_pem_or_raw(&content).expect("Error serializing OV");
        last_guid = ov.header().guid().to_string();
        ov_map.insert(ov.header().guid().to_string(), ov);
    }

    // get a connection from the pool
    let conn = &mut pool.get().unwrap();
    // store ovs in the database
    for (_, ov) in ov_map.clone().into_iter() {
        PostgresDB::insert_ov(&ov, conn)?;
    }
    // we should have 3 ovs
    let count: i64 = ownership_voucher::dsl::ownership_voucher
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 3);

    // add some metadata for the ovs
    for (guid, _) in ov_map.clone().into_iter() {
        PostgresDB::update_ov_metadata_i64(
            &guid,
            OVMetadataKey::To0AcceptOwnerWaitSeconds,
            &(2000 as i64),
            conn,
        )?;
        PostgresDB::update_ov_metadata_bool(&guid, OVMetadataKey::To2Performed, &true, conn)?;
    }

    // this should error since the key needs a bool value
    assert!(PostgresDB::update_ov_metadata_i64(
        &last_guid,
        OVMetadataKey::To2Performed,
        &(2000 as i64),
        conn,
    )
    .is_err());

    // delete an ov, we should have 2
    PostgresDB::delete_ov(&last_guid, conn)?;
    let count: i64 = ownership_voucher::dsl::ownership_voucher
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(count, 2);

    Ok(())
}
