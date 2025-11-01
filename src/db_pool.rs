use std::sync::{Arc, Mutex};

use once_cell::sync::OnceCell;
use rusqlite::{Connection, Result};

static POOL_CELL: OnceCell<DbPool> = OnceCell::new();

#[derive(Clone)]
pub struct DbPool {
    connection: Arc<Mutex<Connection>>,
}

impl DbPool {
    fn new() -> Result<Self> {
        let connection = Connection::open("proximity_service.db")?;

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS businesses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                address TEXT NOT NULL,
                latitude REAL NOT NULL,
                longitude REAL NOT NULL
            )",
                [],
            )
            .expect("Unable to create business table");

        Ok(DbPool {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn init() -> Result<()> {
        let pool = Self::new()?;
        // todo: supposed to have couple connections, now only has one connection
        POOL_CELL.set(pool).map_err(|_| {
            rusqlite::Error::InvalidParameterName("Pool already initialized".to_string())
        })?;
        Ok(())
    }

    pub fn global() -> Result<&'static DbPool> {
        POOL_CELL.get().ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(
                "Pool not initialized. Call DbPool::init() first.".to_string(),
            )
        })
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.connection.lock().unwrap()
    }
}
