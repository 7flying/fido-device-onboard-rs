pub mod models;
pub mod schema;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use schema::ownership_voucher::dsl::*;
use std::env;
use std::fs;

use crate::models::OwnershipVoucherModel;


fn connect_to_db() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

// copies ov files from a given path onto the db
fn ovs_to_db(path: &String) -> Result<()> {
    for ov in fs::read_dir(path)? {
        let ov = ov?;
        let content = fs::read(ov.path()).context("Error reading ov")?;
        let ov_struct =
            OV::from_pem_or_raw(&content).context("Error deserializing OV")?;
        println!("OV: {:?}", ov_struct);
    }
    Ok(())
}

fn store_ov(ov: &OV) {
    let new_ov_model = OwnershipVoucherModel {
        guid: ov.header().guid().to_string(),
        //contents: ov.to_pem(), // TODO: serialize to u8
    };
}

fn main() {
    let connection = &mut connect_to_db();
    let results = ownership_voucher
        .filter(to2_performed.eq(false))
        .limit(5)
        .select(OwnershipVoucherModel::as_select())
        .load(connection)
        .expect("error loading ovs");

    println!("we've got {} ovs", results.len());
    for ov in results {
        println!("guid: {}", ov.guid);
        println!("contents: {:?}", ov.contents);
        println!("to2_performed: {:?}", ov.to2_performed);
        println!(
            "to0_accepted_owner_wait_seconds: {:?}",
            ov.to0_accepted_owner_wait_seconds
        );
    }
    ovs_to_db(&"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs".to_string());

    let results = ownership_voucher
        .filter(to2_performed.eq(false))
        .limit(5)
        .select(OwnershipVoucherModel::as_select())
        .load(connection)
        .expect("error loading ovs");

    println!("we've got {} ovs", results.len());
    for ov in results {
        println!("guid: {}", ov.guid);
        println!("contents: {:?}", ov.contents);
        println!("to2_performed: {:?}", ov.to2_performed);
        println!(
            "to0_accepted_owner_wait_seconds: {:?}",
            ov.to0_accepted_owner_wait_seconds
        );
    }
    
    println!("all good");
}
