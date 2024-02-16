use std::{error::Error, rc::Rc};

use crate::models::Employee;

use super::sqlite::SqliteClient;

pub struct WorkforceReadRepository {
    sql_client: Rc<SqliteClient>,
}

impl WorkforceReadRepository {
    pub fn new(sql_client: Rc<SqliteClient>) -> Self {
        Self { sql_client }
    }

    pub fn find_snapshot(&self) -> Result<Vec<Employee>, Box<dyn Error>> {
        let connection = self.sql_client.get_connection()?;
        let mut stmt = match connection.prepare(
            "SELECT employees FROM workforce_snapshot ORDER BY snapshotted_at DESC LIMIT 1",
        ) {
            Ok(stmt) => stmt,
            Err(err) => return Err(Box::new(err)),
        };

        let snapshot: String = stmt.query([])?.next()?.unwrap().get(0)?;
        let snapshot: Vec<Employee> = serde_json::from_str(&snapshot)?;

        Ok(snapshot)
    }
}
