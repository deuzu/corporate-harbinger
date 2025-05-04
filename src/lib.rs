use std::{env, error::Error};

use config::Config as Cfg;
use serde::Deserialize;

pub mod changes_detector;
pub mod collector;
pub mod models;
pub mod notify;
pub mod repository;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dry_run: bool,
    pub ldap_starttls: bool,
    pub ldap_url: String,
    pub ldap_dn: String,
    pub ldap_password: String,
    pub ldap_search_base: String,
    pub ldap_search_filter: String,
    pub ldap_search_attributes: ConfigProviderSearchAttributes,
    pub sqlite_database_path: String,
    pub discord_webhook_url: String,
    pub discord_bot_username: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file_path = env::var("CH_CONFIG_FILE_PATH")?;
        let settings = Cfg::builder()
            .set_default("dry_run", false)?
            .add_source(config::File::with_name(&file_path))
            .add_source(config::Environment::with_prefix("CH").try_parsing(true))
            .build()?;

        let config = settings.try_deserialize::<Self>()?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigProviderSearchAttributes {
    pub name: String,
    pub alias: String,
    pub workplace: Option<String>,
    pub business_unit: Option<String>,
    pub job_title: Option<String>,
}
