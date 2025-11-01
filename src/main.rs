pub mod data;
pub mod db_pool;

use rusqlite::Result;

use crate::{data::business::Business, db_pool::DbPool};

fn main() -> Result<()> {
    DbPool::init().expect("Errors when initializing database");

    let business = Business::new(
        "first business".to_owned(),
        "first address".to_owned(),
        36.68333000,
        71.53333000,
    );
    println!(
        "successfully created a business with id: {}, name: {}!",
        business.name,
        business.id.unwrap()
    );
    Ok(())
}
