use rusqlite::{params, Result};

use crate::db_pool::DbPool;

#[derive(Debug)]
pub struct Business {
    pub id: Option<i64>,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Business {
    pub(crate) fn new(name: String, latitude: f64, longitude: f64) -> Self {
        let mut business = Business {
            id: None,
            name,
            latitude,
            longitude,
        };

        business.save_to_database();

        business
    }

    pub fn get(id: i64) -> Self {
        let result = Business::read_from_database(id);
        result.unwrap()
    }

    pub fn query_all() -> Vec<Business> {
        let connection = DbPool::global().unwrap().get_connection();
        let mut statement = connection.prepare("select * from businesses").unwrap();
        let businesses: Vec<Business> = match statement.query_map([], |row| {
            Ok(Business {
                id: row.get("id")?,
                name: row.get("name")?,
                latitude: row.get("latitude")?,
                longitude: row.get("longitude")?,
            })
        }) {
            Ok(it) => it,
            Err(err) => panic!("{}", err),
        }
        .collect::<Result<Vec<Business>>>()
        .unwrap();
        return businesses;
    }

    // self parameter needs to be mutable because id is given by database
    fn save_to_database(&mut self) {
        let connection = DbPool::global().unwrap().get_connection();
        connection
            .execute(
                "INSERT INTO businesses (name, latitude, longitude) VALUES (?1, ?2, ?3)",
                params![self.name, self.latitude, self.longitude],
            )
            .expect("Could not execute the insert statement to the database");
        self.id = Some(connection.last_insert_rowid());
    }

    fn read_from_database(id: i64) -> Result<Self> {
        let connection = DbPool::global().unwrap().get_connection();
        let mut statement = connection
            .prepare("select * from businesses where id = ?1")
            .unwrap();
        statement.query_row([id], |row| {
            Ok(Business {
                id: row.get(0)?,
                name: row.get(1)?,
                latitude: row.get(2)?,
                longitude: row.get(3)?,
            })
        })
        // match query_result {
        //     Ok(business) => business,
        //     // Err(rusqlite::Error::QueryReturnedNoRows) => (),
        //     Err(e) => Err(e),
        // }
    }

    pub fn get_direction(self: &Self) -> Direction {
        // todo: return direction based on the coordinates and the anchor
        Direction::NorthWest
    }
}

pub enum Direction {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}
