use std::error::Error;

use crate::models::{Changes, Employee};

use super::NotifyClient;

pub struct DryRunNotifyClient {}

impl DryRunNotifyClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl NotifyClient for DryRunNotifyClient {
    fn send(&self, changes: &Changes) -> Result<(), Box<dyn Error>> {
        let fmt_employee = |n: Employee| {
                format!(
                    "{} ({} - {} - {})",
                    n.name,
                    n.workplace.or(Some(String::from("None"))).unwrap(),
                    n.business_unit.or(Some(String::from("None"))).unwrap(),
                    n.job_title.or(Some(String::from("None"))).unwrap(),
                )
            };
        let newcomers: Vec<String> = changes
            .newcomers
            .clone()
            .into_iter()
            .map(fmt_employee)
            .collect();

        let departures: Vec<String> = changes
            .departures
            .clone()
            .into_iter()
            .map(fmt_employee)
            .collect();

        log::info!("Dry-run notification");
        log::info!("Newcommers: {:#?}", newcomers);
        log::info!("Departures: {:#?}", departures);

        Ok(())
    }
}
