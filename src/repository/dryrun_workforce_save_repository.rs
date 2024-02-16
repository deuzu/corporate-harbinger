use std::error::Error;

use crate::models::Employee;

use super::save_repository::SaveRepository;

pub struct DryRunWorkforceSaveRepository {}

impl DryRunWorkforceSaveRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl SaveRepository for DryRunWorkforceSaveRepository {
    fn save(&self, _snapshot: &Vec<Employee>) -> Result<(), Box<dyn Error>> {
        log::info!("Dry-run workforce save");

        Ok(())
    }
}
