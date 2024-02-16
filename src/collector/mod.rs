use std::error::Error;

use crate::{provider::WorkforceProvider, repository::WorkforceRepository};

pub struct Collector<'a> {
    workforce_provider: &'a mut WorkforceProvider<'a>,
    workforce_repository: &'a WorkforceRepository<'a>,
}

impl <'a> Collector<'a> {
    pub fn new(workforce_provider: &'a mut WorkforceProvider<'a>, workforce_repository: &'a WorkforceRepository) -> Self {
        Self {
            workforce_provider,
            workforce_repository
        }
    }

    pub fn collect(&mut self) -> Result<(), Box<dyn Error>> {
        let snapshot = self.workforce_provider.provide()?;
        self.workforce_repository.save(snapshot)?;

        Ok(())
    }
}
