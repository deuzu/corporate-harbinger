use std::error::Error;

use crate::{client::DiscordClient, models::Employee, repository::WorkforceRepository};

pub struct Notifier<'a> {
    workforce_repository: &'a WorkforceRepository<'a>,
    discord_client: &'a DiscordClient<'a>,
}

impl<'a> Notifier<'a> {
    pub fn new(
        workforce_repository: &'a WorkforceRepository<'a>,
        discord_client: &'a DiscordClient<'a>,
    ) -> Self {
        Self {
            workforce_repository,
            discord_client,
        }
    }

    pub fn notify(&self) -> Result<(), Box<dyn Error>> {
        let (current_employees, previous_employees) = self
            .workforce_repository
            .find_current_previous_employees_snapshot()?;

        let newcomers: Vec<Employee> = current_employees
            .clone()
            .into_iter()
            .filter(|a| !previous_employees.iter().any(|b| b.alias == a.alias))
            .collect();

        let departures: Vec<Employee> = previous_employees
            .clone()
            .into_iter()
            .filter(|a| !current_employees.iter().any(|b| b.alias == a.alias))
            .collect();

        if &newcomers.len() < &1 && &departures.len() < &1 {
            log::info!("Corporate Harbinger had nothing to announce");

            return Ok(());
        }

        self.discord_client.send(&newcomers, &departures)?;

        log::info!("Corporate Harbinger made announcement(s)");

        Ok(())
    }
}
