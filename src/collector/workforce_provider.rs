use std::error::Error;

use ldap3::{Scope, SearchEntry};

use crate::{models::Employee, ConfigProviderSearchAttributes};

use super::ldap_client::LdapClient;

pub struct WorkforceProvider {
    ldap_base: String,
    ldap_filter: String,
    ldap_search_attributes: ConfigProviderSearchAttributes,
    ldap_client: Box<LdapClient>,
}

impl WorkforceProvider {
    pub fn new(
        ldap_base: String,
        ldap_filter: String,
        ldap_search_attributes: ConfigProviderSearchAttributes,
        ldap_client: Box<LdapClient>,
    ) -> Self {
        Self {
            ldap_base,
            ldap_filter,
            ldap_search_attributes,
            ldap_client,
        }
    }

    pub fn provide(&mut self) -> Result<Vec<Employee>, Box<dyn Error>> {
        let connection = self.ldap_client.get_connection()?;
        let (rs, _res) = connection
            .search(
                &self.ldap_base,
                Scope::Subtree,
                &self.ldap_filter,
                Self::get_attributes(&self.ldap_search_attributes),
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

        Ok(employees)
    }

    fn get_attributes(ldap_search_attributes: &ConfigProviderSearchAttributes) -> Vec<String> {
        let mut attributes = Vec::from([
            ldap_search_attributes.name.clone(),
            ldap_search_attributes.alias.clone(),
        ]);

        if let Some(w) = &ldap_search_attributes.workplace {
            attributes.push(w.to_string());
        }

        if let Some(bu) = &ldap_search_attributes.business_unit {
            attributes.push(bu.to_string());
        }

        if let Some(jt) = &ldap_search_attributes.job_title {
            attributes.push(jt.to_string());
        }

        attributes
    }
}
