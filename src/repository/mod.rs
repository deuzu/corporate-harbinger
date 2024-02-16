use std::error::Error;

use rusqlite::params;

use crate::{
    client::SqliteClient,
    models::{Employee, WorkforceSnapshot},
};

pub struct WorkforceRepository<'a> {
    sql_client: &'a SqliteClient<'a>,
}

impl<'a> WorkforceRepository<'a> {
    pub fn new(sql_client: &'a SqliteClient) -> Self {
        Self { sql_client }
    }

    pub fn save(&self, snapshot: WorkforceSnapshot) -> Result<(), Box<dyn Error>> {
        let connection = self.sql_client.get_connection()?;

        match connection.execute(
            "INSERT INTO workforce_snapshot (source, employees, snapshotted_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![&snapshot.source, &snapshot.employees],
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn find_current_previous_employees_snapshot(
        &self,
    ) -> Result<(Vec<Employee>, Vec<Employee>), Box<dyn Error>> {
        let connection = self.sql_client.get_connection()?;
        let mut stmt = match connection.prepare(
            "SELECT employees FROM workforce_snapshot ORDER BY snapshotted_at DESC LIMIT 2",
        ) {
            Ok(stmt) => stmt,
            Err(err) => return Err(Box::new(err)),
        };

        let track_iter = stmt.query_map([], |row| Ok(row.get(0)?))?;
        let tracks = track_iter.map(|r| r.unwrap()).collect::<Vec<String>>();
        let current = tracks.get(0).unwrap();
        let previous = tracks.get(1).unwrap();
        let current_employees: Vec<Employee> = serde_json::from_str(&current)?;
        let previous_employees: Vec<Employee> = serde_json::from_str(&previous)?;

        Ok((current_employees, previous_employees))
    }
}
