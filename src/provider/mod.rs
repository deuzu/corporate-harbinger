use std::error::Error;

use ldap3::{Scope, SearchEntry};
use serde::Deserialize;

use crate::{
    client::LdapClient,
    models::{Employee, WorkforceSnapshot},
};

pub struct WorkforceProvider<'a> {
    ldap_base: &'a str,
    ldap_filter: &'a str,
    ldap_search_attributes: &'a ProviderSearchAttributes,
    ldap_client: &'a mut LdapClient<'a>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderSearchAttributes {
    pub name: String,
    pub alias: String,
    pub workplace: Option<String>,
    pub business_unit: Option<String>,
    pub job_title: Option<String>,
}

impl ProviderSearchAttributes {
    fn get_attributes(&self) -> Vec<String> {
        let mut attributes = Vec::from([self.name.clone(), self.alias.clone()]);

        if let Some(w) = &self.workplace {
            attributes.push(w.to_string());
        }

        if let Some(bu) = &self.business_unit {
            attributes.push(bu.to_string());
        }

        if let Some(jt) = &self.job_title {
            attributes.push(jt.to_string());
        }

        attributes
    }
}

impl<'a> WorkforceProvider<'a> {
    pub fn new(
        ldap_base: &'a str,
        ldap_filter: &'a str,
        ldap_search_attributes: &'a ProviderSearchAttributes,
        ldap_client: &'a mut LdapClient<'a>,
    ) -> Self {
        Self {
            ldap_base,
            ldap_filter,
            ldap_search_attributes,
            ldap_client,
        }
    }

    pub fn provide(&mut self) -> Result<WorkforceSnapshot, Box<dyn Error>> {
        let connection = self.ldap_client.get_connection()?;
        let (rs, _res) = connection
            .search(
                self.ldap_base,
                Scope::Subtree,
                self.ldap_filter,
                self.ldap_search_attributes.get_attributes(),
            )?
            .success()?;

        let mut employees: Vec<Employee> = Vec::new();

        for entry in rs {
            let entry = SearchEntry::construct(entry.clone());
            let name = entry
                .attrs
                .get(&self.ldap_search_attributes.name)
                .unwrap()
                .join("");
            let alias = entry
                .attrs
                .get(&self.ldap_search_attributes.alias)
                .unwrap()
                .join("");
            let workplace = match &self.ldap_search_attributes.workplace {
                Some(v) => entry.attrs.get(v).and_then(|v| Some(v.join(""))),
                None => None,
            };
            let business_unit = match &self.ldap_search_attributes.business_unit {
                Some(v) => entry.attrs.get(v).and_then(|v| Some(v.join(""))),
                None => None,
            };
            let job_title = match &self.ldap_search_attributes.business_unit {
                Some(v) => entry.attrs.get(v).and_then(|v| Some(v.join(""))),
                None => None,
            };

            let employee = Employee {
                name,
                alias,
                workplace,
                business_unit,
                job_title,
            };

            employees.push(employee);
        }

        self.ldap_client.close_connection()?;

        Ok(WorkforceSnapshot {
            source: String::from("ldap-v1"),
            employees: serde_json::to_string(&employees)?,
        })
    }
}
