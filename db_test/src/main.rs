pub mod models;
pub mod schema;

use std::env;
use std::fs;
use std::fs::File;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use fdo_data_formats::ownershipvoucher::OwnershipVoucher as OV;
use fdo_data_formats::Serializable;
//use schema::ownership_voucher::dsl::ownership_voucher;
use xattr::FileExt;

use crate::models::{NewOwnershipVoucherModel, OwnershipVoucherModel};
use crate::schema::ownership_voucher;

fn connect_to_db() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn print_ovs_on_disk(path: &String) -> Result<()> {
    for ov in fs::read_dir(path)? {
        let ov = ov?;
        let content = fs::read(ov.path()).context("Error reading ov")?;
        let ov_struct = OV::from_pem_or_raw(&content).context("Error deserializing OV")?;
        println!("{ov_struct:?}");
        match xattr::get(ov.path(), "user.store_ttl") {
            Ok(Some(v)) => {
                let value = i64::from_le_bytes(v.try_into().unwrap());
                println!("user.store_ttl: {value}");
            }
            Ok(None) => {}
            Err(e) => {
                println!("error: {e}");
            }
        }
        match xattr::get(ov.path(), "user.fdo.to0_accept_owner_wait_seconds") {
            Ok(Some(v)) => {
                let value = i64::from_le_bytes(v.try_into().unwrap());
                println!("user.fdo.to0_accpet_owner_wait_seconds: {value}");
            }
            Ok(None) => {}
            Err(e) => {
                println!("error: {e}");
            }
        }
        match xattr::get(ov.path(), "user.fdo.to2_performed") {
            Ok(Some(v)) => {
                let bool_str = std::str::from_utf8(&v).unwrap();
                println!("user.fdo.to2_performed: {bool_str}");
            }
            Ok(None) => {}
            Err(e) => {
                println!("error: {e}");
            }
        }
        println!();
    }
    Ok(())
}

// copies ov files from a given path onto the db
fn ovs_to_db(path: &String) -> Result<()> {
    for ov in fs::read_dir(path)? {
        let ov = ov?;
        let content = fs::read(ov.path()).context("Error reading ov")?;
        let ov_struct = OV::from_pem_or_raw(&content).context("Error deserializing OV")?;
        println!("OV: {:?}", ov_struct);
        let mut store_ttl = None;
        let mut to0_accept_owner = None;
        let mut to2_performed = None;
        match xattr::get(ov.path(), "user.store_ttl") {
            Ok(Some(v)) => {
                let value = i64::from_le_bytes(v.try_into().unwrap());
                store_ttl = Some(value);
            }
            Ok(None) => {}
            Err(e) => {
                println!("error: {e}");
            }
        }
        match xattr::get(ov.path(), "user.fdo.to0_accept_owner_wait_seconds") {
            Ok(Some(v)) => {
                let value = i64::from_le_bytes(v.try_into().unwrap());
                to0_accept_owner = Some(value);
            }
            Ok(None) => {}
            Err(e) => {
                println!("error: {e}");
            }
        }
        match xattr::get(ov.path(), "user.fdo.to2_performed") {
            Ok(Some(v)) => {
                let value = std::str::from_utf8(&v).unwrap();
                println!("bool value: {value}");
                to2_performed = if value == "true" {
                    Some(true)
                } else {
                    Some(false)
                };
            }
            Ok(None) => {}
            Err(e) => {
                println!("error: {e}");
            }
        }
        _store_ov(&ov_struct, to2_performed, to0_accept_owner, store_ttl)?;
    }
    Ok(())
}

fn _store_ov(ov: &OV, to2: Option<bool>, to0: Option<i64>, ttl_: Option<i64>) -> Result<()> {
    let new_ov_model = NewOwnershipVoucherModel {
        guid: ov.header().guid().to_string(),
        contents: ov.serialize_data()?,
        to2_performed: to2,
        to0_accept_owner_wait_seconds: to0,
        ttl: ttl_,
    };
    let conn: &mut SqliteConnection = &mut connect_to_db();
    diesel::insert_into(schema::ownership_voucher::table)
        .values(&new_ov_model)
        .execute(conn)
        .expect("Error saving ov");
    Ok(())
}

fn add_dummy_xattr(path: &String, xattr_name: &String) -> Result<()> {
    let f = File::open(path)?;
    let wait_seconds: u32 = 32;
    f.set_xattr(xattr_name, &(wait_seconds as i64).to_le_bytes())?;
    Ok(())
}

fn add_dummy_xattr_bool(path: &String, xattr_name: &String) -> Result<()> {
    let f = File::open(path)?;
    let value = true;
    //f.set_xattr(xattr_name, &value.to_string().as_bytes().to_vec())?;
    f.set_xattr(xattr_name, &value.to_string().as_bytes())?;
    Ok(())
}

fn _add_extended_attributes() {
    // 7815e9ab-65c6-c8ee-a761-0691ec26a6a3
    add_dummy_xattr(
        &"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs/7815e9ab-65c6-c8ee-a761-0691ec26a6a3".to_string(),
        &"user.store_ttl".to_string(),
    )
    .unwrap();
    add_dummy_xattr(
        &"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs/7815e9ab-65c6-c8ee-a761-0691ec26a6a3".to_string(),
        &"user.fdo.to0_accept_owner_wait_seconds".to_string(),
    )

        .unwrap();
    // 78e2994b-eb7c-046e-6a31-6e61d77e9d6f
    add_dummy_xattr(
        &"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs/78e2994b-eb7c-046e-6a31-6e61d77e9d6f".to_string(),
        &"user.fdo.to0_accept_owner_wait_seconds".to_string(),
    ).unwrap();

    // fb810f4a-3314-9844-6596-9a4b6ac7ba27
    add_dummy_xattr_bool(
        &"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs/fb810f4a-3314-9844-6596-9a4b6ac7ba27".to_string(),
        &"user.fdo.to2_performed".to_string(),
    )
    .unwrap();
}

fn main() {
    //_add_extended_attributes();

    println!("Printing OVs on disk:");
    print_ovs_on_disk(
        &"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs".to_string(),
    )
    .unwrap();
    println!("\n");

    let connection = &mut connect_to_db();
    // let results = schema::ownership_voucher::dsl::ownership_voucher
    //      .select(OwnershipVoucherModel::as_select())
    //      .load(connection)
    //      .expect("error loading ovs");

    // println!(
    //     "Reading OVs (model) from the DB... we've got {} ovs",
    //     results.len()
    // );
    // for ov in results {
    //     println!("{ov}");
    //     println!("\n converting DB OV to original OV format");
    //     let ov_original = OV::from_pem_or_raw(&ov.contents);
    //     println!("{:?}\n", ov_original.unwrap());
    // }
    // println!("Storing OVs in the database");
    // let _stuff = ovs_to_db(
    //     &"/home/idiez/code/repos/fedora-iot/tests/fido-device-onboard-rs/test-ovs".to_string(),
    // );

    println!("Reading OVs from the database");
    let results = schema::ownership_voucher::dsl::ownership_voucher
        .select(OwnershipVoucherModel::as_select())
        .load(connection)
        .expect("error loading ovs");

    println!("\twe've got {} ovs", results.len());
    for ov in results {
        println!("\t· {ov}");
    }

    // update the metadata of an OV.
    let test_guid = "7815e9ab-65c6-c8ee-a761-0691ec26a6a3";
    let result = diesel::update(schema::ownership_voucher::dsl::ownership_voucher)
        .filter(ownership_voucher::guid.eq(test_guid))
        .set(ownership_voucher::to2_performed.eq(true))
        .execute(connection);

    println!("All good");
}
