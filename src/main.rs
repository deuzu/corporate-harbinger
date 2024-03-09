use std::rc::Rc;

use corporate_harbinger::{
    changes_detector::ChangesDetector,
    collector::{ldap_client::LdapClient, workforce_provider::WorkforceProvider, Collector},
    notify::{
        discord::DiscordClient, dryrun_notify::DryRunNotifyClient, notify::NotifyClient, Notifier,
    },
    repository::{
        dryrun_workforce_save_repository::DryRunWorkforceSaveRepository,
        save_repository::SaveRepository, sqlite::SqliteClient,
        workforce_read_repository::WorkforceReadRepository,
        workforce_save_repository::WorkforceSaveRepository,
    },
    Config,
};
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Corporate Harbinger has initiated its professional duties");

    let config = match Config::new() {
        Ok(c) => c,
        Err(err) => {
            log::error!("Configuration error, check the config file and/or the env var \"CH_CONFIG_FILE_PATH\". {}", err);

            return;
        }
    };

    let ldap_client = Box::new(LdapClient::new(
        config.ldap_url,
        config.ldap_starttls,
        config.ldap_dn,
        config.ldap_password,
    ));
    let collector_sql_client = Rc::new(SqliteClient::new(config.sqlite_database_path));
    let sql_client = Rc::clone(&collector_sql_client);

    let workforce_provider = Box::new(WorkforceProvider::new(
        config.ldap_search_base,
        config.ldap_search_filter,
        config.ldap_search_attributes,
        ldap_client,
    ));
    let collector_workforce_repository = Box::new(WorkforceReadRepository::new(collector_sql_client));
    let notifier_client: Box<dyn NotifyClient> = match &config.dry_run {
        false => Box::new(DiscordClient::new(
            config.discord_webhook_url,
            config.discord_bot_username,
        )),
        true => Box::new(DryRunNotifyClient::new()),
    };
    let workforce_save_repository: Box<dyn SaveRepository> = match &config.dry_run {
        false => Box::new(WorkforceSaveRepository::new(sql_client)),
        true => Box::new(DryRunWorkforceSaveRepository::new()),
    };

    let mut collector = Collector::new(
        workforce_provider,
        collector_workforce_repository,
    );
    let changes_detector = ChangesDetector::new();
    let notifier = Notifier::new(notifier_client);

    let (previous_snapshot, current_snapshot) = match collector.collect() {
        Ok(snapshots) => snapshots,
        Err(err) => {
            log::error!("Collect error: {}", err);

            return;
        }
    };

    let changes = match changes_detector.detect(&previous_snapshot, &current_snapshot) {
        Ok(c) => c,
        Err(err) => {
            log::error!("Changes detection error: {}", err);

            return;
        }
    };

    match notifier.notify(&changes) {
        Ok(_) => (),
        Err(err) => {
            log::error!("Notify error: {}", err);

            return;
        }
    };

    match workforce_save_repository.save(&current_snapshot) {
        Ok(_) => (),
        Err(err) => {
            log::error!("Save error: {}", err);

            return;
        }
    };

    log::info!("Corporate Harbinger has clocked out");

    ()
}
