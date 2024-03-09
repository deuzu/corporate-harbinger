use std::{error::Error, rc::Rc};

use rusqlite::params;

use crate::models::Employee;

use super::{save_repository::SaveRepository, sqlite::SqliteClient};

pub struct WorkforceSaveRepository {
    sql_client: Rc<SqliteClient>,
}

impl WorkforceSaveRepository {
    pub fn new(sql_client: Rc<SqliteClient>) -> Self {
        Self { sql_client }
    }
}

impl SaveRepository for WorkforceSaveRepository {
    fn save(&self, snapshot: &Vec<Employee>) -> Result<(), Box<dyn Error>> {
        let connection = self.sql_client.get_connection()?;
        let snapshot: String = serde_json::to_string(snapshot)?;

        match connection.execute(
            "INSERT INTO workforce_snapshot (source, employees, snapshotted_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params!["ldap-v1", snapshot],
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}
