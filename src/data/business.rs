use rusqlite::params;

use crate::db_pool::DbPool;

pub struct Business {
    pub id: Option<i64>,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Business {
    pub fn new(name: String, address: String, latitude: f64, longitude: f64) -> Self {
        let mut business = Business {
            id: None,
            name,
            address,
            latitude,
            longitude,
        };

        business.save_to_database();

        business
    }

    // self parameter needs to be mutable because id is given by database
    fn save_to_database(&mut self) {
        let connection = DbPool::global().unwrap().get_connection();
        connection.execute(
            "INSERT INTO businesses (name, address, latitude, longitude) VALUES (?1, ?2, ?3, ?4)",
            params![self.name, self.address, self.latitude, self.longitude],
        ).expect("Could not execute the insert statement to the database");
        self.id = Some(connection.last_insert_rowid());
    }
}
