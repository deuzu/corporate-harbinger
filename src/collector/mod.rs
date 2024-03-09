use std::error::Error;

use crate::{models::Employee, repository::workforce_read_repository::WorkforceReadRepository};

use self::workforce_provider::WorkforceProvider;

pub mod ldap_client;
pub mod workforce_provider;

pub struct Collector {
    workforce_provider: Box<WorkforceProvider>,
    workforce_repository: Box<WorkforceReadRepository>,
}

impl Collector {
    pub fn new(
        workforce_provider: Box<WorkforceProvider>,
        workforce_repository: Box<WorkforceReadRepository>,
    ) -> Self {
        Self {
            workforce_provider,
            workforce_repository,
        }
    }

    pub fn collect(&mut self) -> Result<(Vec<Employee>, Vec<Employee>), Box<dyn Error>> {
        let previous_snapshot = self.workforce_repository.find_snapshot()?;
        let current_snapshot = self.workforce_provider.provide()?;

        Ok((previous_snapshot, current_snapshot))
    }
}
