use std::error::Error;

use crate::models::{Changes, Employee};

pub struct ChangesDetector {
}

impl ChangesDetector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn detect(&self, previous_snapshot: &Vec<Employee>, current_snapshot: &Vec<Employee>) -> Result<Changes, Box<dyn Error>> {
        let newcomers: Vec<Employee> = current_snapshot
            .clone()
            .into_iter()
            .filter(|a| !previous_snapshot.iter().any(|b| b.alias == a.alias))
            .collect();

        let departures: Vec<Employee> = previous_snapshot
            .clone()
            .into_iter()
            .filter(|a| !current_snapshot.iter().any(|b| b.alias == a.alias))
            .collect();

        Ok(Changes {
            newcomers,
            departures,
        })
    }
}
