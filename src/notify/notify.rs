use std::error::Error;

use crate::models::Changes;

pub trait NotifyClient {
    fn send(&self, changes: &Changes) -> Result<(), Box<dyn Error>>;
}
