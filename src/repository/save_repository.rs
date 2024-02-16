use std::error::Error;

use crate::models::Employee;

pub trait SaveRepository {
    fn save(&self, snapshot: &Vec<Employee>) -> Result<(), Box<dyn Error>>;
}
