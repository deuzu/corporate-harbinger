use std::error::Error;

use rusqlite::Connection;

pub struct SqliteClient {
    database_path: String,
}

impl SqliteClient {
    pub fn new(database_path: String) -> Self {
        Self { database_path }
    }

    pub fn get_connection(&self) -> Result<Connection, Box<dyn Error>> {
        match Connection::open(self.database_path.clone()) {
            Ok(c) => Ok(c),
            Err(err) => Err(Box::new(err)),
        }
    }
}
