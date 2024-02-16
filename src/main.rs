use corporate_harbinger::{
    client::{DiscordClient, LdapClient, SqliteClient},
    collector::Collector,
    notifier::Notifier,
    provider::{ProviderSearchAttributes, WorkforceProvider},
    repository::WorkforceRepository,
    Config,
};

fn main() {
    env_logger::init();
    log::info!("Corporate Harbinger has initiated its professional duties");

    let config = match Config::new() {
        Ok(c) => c,
        Err(err) => {
            log::error!("Configuration error, check the config file and the env var \"CH_CONFIG_FILE_PATH\". {}", err);

            return;
        }
    };

    let mut ldap_client = LdapClient::new(
        &config.ldap_url,
        &config.ldap_starttls,
        &config.ldap_dn,
        &config.ldap_password,
    );
    let sql_client = SqliteClient::new(&config.sqlite_database_path);
    let discord_client = DiscordClient::new(&config.discord_webhook_url, &config.discord_bot_username);

    let provider_search_attributes: ProviderSearchAttributes = config.ldap_search_attributes.into();
    let mut workforce_provider = WorkforceProvider::new(
        &config.ldap_search_base,
        &config.ldap_search_filter,
        &provider_search_attributes,
        &mut ldap_client,
    );
    let workforce_repository = WorkforceRepository::new(&sql_client);

    let mut collector = Collector::new(&mut workforce_provider, &workforce_repository);
    match collector.collect() {
        Ok(_) => (),
        Err(err) => {
            log::error!("Collect error: {}", err);

            return;
        },
    };

    let notifier = Notifier::new(&workforce_repository, &discord_client);
    match notifier.notify() {
        Ok(_) => (),
        Err(err) => {
            log::error!("Notify error: {}", err);

            return;
        }
    };

    log::info!("Corporate Harbinger has clocked out");

    ()
}
