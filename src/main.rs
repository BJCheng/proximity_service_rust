pub mod data;
pub mod db_pool;
mod search;

use rusqlite::Result;

use crate::{data::business::Business, db_pool::DbPool, search::quadtree::Quadtree};

fn main() -> Result<()> {
    DbPool::init().expect("Errors when initializing database");

    // let business = Business::new("first business".to_owned(), 36.68333000, 71.53333000);
    // println!(
    //     "successfully created a business with id: {}, name: {}!",
    //     business.name,
    //     business.id.unwrap()
    // );

    // let business = Business::get(56);
    // println!("{:?}", business);

    let businesses = Business::query_all();
    for business in businesses {
        println!("==={:?}===", business);
    }
    // let quadtree = Quadtree {
    //     businesses: Vec::new(),
    // };
    // quadtree.put(businesses);

    // start the server and listen for http requests

    Ok(())
}
