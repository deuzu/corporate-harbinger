use std::{error::Error, time::Duration};

use ldap3::{LdapConn, LdapConnSettings};

pub struct LdapClient {
    ldap_url: String,
    ldap_starttls: bool,
    ldap_dn: String,
    ldap_password: String,
    connection: Option<LdapConn>,
}

impl LdapClient {
    pub fn new(
        ldap_url: String,
        ldap_starttls: bool,
        ldap_dn: String,
        ldap_password: String,
    ) -> Self {
        Self {
            ldap_url,
            ldap_starttls,
            ldap_dn,
            ldap_password,
            connection: None,
        }
    }

    pub fn get_connection(&mut self) -> Result<&mut LdapConn, Box<dyn Error>> {
        let ldap_setting = LdapConnSettings::new()
            .set_starttls(self.ldap_starttls)
            .set_conn_timeout(Duration::from_secs(5));
        let mut connection = LdapConn::with_settings(ldap_setting, &self.ldap_url)?;

        connection.simple_bind(&self.ldap_dn, &self.ldap_password)?;

        self.connection = Some(connection);

        Ok(self.connection.as_mut().unwrap())
    }

    pub fn close_connection(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(connection) = &mut self.connection {
            connection.unbind()?;
        }

        Ok(())
    }
}
