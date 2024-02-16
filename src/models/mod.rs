use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Employee {
    pub name: String,
    pub alias: String,
    pub workplace: Option<String>,
    pub business_unit: Option<String>,
    pub job_title: Option<String>,
}

#[derive(Debug)]
pub struct Changes {
    pub newcomers: Vec<Employee>,
    pub departures: Vec<Employee>,
}
