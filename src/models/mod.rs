use serde::{Deserialize, Serialize};

pub struct WorkforceSnapshot {
    pub source: String,
    pub employees: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Employee {
    pub name: String,
    pub alias: String,
    pub workplace: Option<String>,
    pub business_unit: Option<String>,
    pub job_title: Option<String>,
}

pub struct WorkforceNotification {
    pub newcomers: Vec<Employee>,
    pub departures: Vec<Employee>,
}
