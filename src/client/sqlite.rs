use std::error::Error;

use rusqlite::Connection;

pub struct SqliteClient<'a> {
    database_path: &'a str,
}

impl<'a> SqliteClient<'a> {
    pub fn new(database_path: &'a str) -> Self {
        Self { database_path }
    }

    pub fn get_connection(&self) -> Result<Connection, Box<dyn Error>> {
        match Connection::open(self.database_path) {
            Ok(c) => Ok(c),
            Err(err) => Err(Box::new(err)),
        }
    }
}
