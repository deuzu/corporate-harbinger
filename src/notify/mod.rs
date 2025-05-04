use std::error::Error;

use crate::models::Changes;

use self::notify::NotifyClient;

pub mod discord;
pub mod dryrun_notify;
pub mod notify;

pub struct Notifier {
    notify_client: Box<dyn NotifyClient>,
}

impl Notifier {
    pub fn new(notify_client: Box<dyn NotifyClient>) -> Self {
        Self { notify_client }
    }

    pub fn notify(&self, changes: &Changes) -> Result<(), Box<dyn Error>> {
        if &changes.newcomers.len() < &1 && &changes.departures.len() < &1 {
            log::info!("Corporate Harbinger had nothing to announce");

            return Ok(());
        }

        self.notify_client.send(&changes)?;

        log::info!("Corporate Harbinger made announcement(s)");

        Ok(())
    }
}
