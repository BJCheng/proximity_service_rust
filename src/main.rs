pub mod data;
pub mod db_pool;
mod search;

use std::{
    env,
    io::{self, Write},
};

use rusqlite::Result;

use crate::{data::business::Business, db_pool::DbPool, search::quadtree::Quadtree};

fn main() -> Result<()> {
    DbPool::init().expect("Errors when initializing database");

    let mut businesses = Business::query_all();

    let quadtree = &mut Quadtree::new();
    quadtree.put(&mut businesses);
    quadtree.print();

    // listen to user input of latitude and longitude
    // return most recent businesses
    loop {
        println!("Your coordinates separated by a space:");
        io::stdout().flush().unwrap(); // ensure prompt print

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap_or_else(|e| panic!("error while parsing user input: {}", e));

        let coordinates: Vec<u32> = input
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();

        if coordinates.len() != 2 {
            panic!("user input should contains exact two numbers");
        }

        println!(
            "latitude: {}, longitude: {}",
            coordinates.get(0).unwrap(),
            coordinates.get(1).unwrap()
        );

        println!("looking for nearby businesses...");
        let businesses = quadtree.search(0, 1);

        if businesses.len() <= 0 {
            println!(
                "no business found with latitude: {}, longitude: {}",
                &coordinates.get(0).unwrap(),
                &coordinates.get(1).unwrap()
            );
        }
        for business in &businesses {
            println!("{}", business.name);
        }
    }

    // Ok(())
}
