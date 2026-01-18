use crate::cli::ConfigCommand;
use crate::config::Config;
use crate::error::{ProwlError, Result};
use crate::output::OutputFormatter;

pub fn execute(cmd: &ConfigCommand, formatter: &dyn OutputFormatter) -> Result<()> {
    match cmd {
        ConfigCommand::Init { force } => init_config(*force, formatter),
        ConfigCommand::Show => show_config(formatter),
        ConfigCommand::Set { key, value } => set_config(key, value, formatter),
        ConfigCommand::Path => show_path(),
    }
}

fn init_config(force: bool, formatter: &dyn OutputFormatter) -> Result<()> {
    let config_path = Config::config_path()?;

    if config_path.exists() && !force {
        return Err(ProwlError::Config(format!(
            "Config file already exists at {}. Use --force to overwrite.",
            config_path.display()
        )));
    }

    let config = Config {
        api_key: None,
        provider_key: None,
        application: Some("prowl-cli".to_string()),
    };

    config.save()?;
    formatter.format_config_init(&config_path);
    Ok(())
}

fn show_config(formatter: &dyn OutputFormatter) -> Result<()> {
    let config_path = Config::config_path()?;
    let config = Config::load()?;
    formatter.format_config_show(&config, &config_path);
    Ok(())
}

fn set_config(key: &str, value: &str, formatter: &dyn OutputFormatter) -> Result<()> {
    let mut config = Config::load()?;

    match key {
        "api_key" => config.api_key = Some(value.to_string()),
        "provider_key" => config.provider_key = Some(value.to_string()),
        "application" => config.application = Some(value.to_string()),
        _ => {
            return Err(ProwlError::Config(format!(
                "Unknown config key: {key}. Valid keys are: api_key, provider_key, application"
            )));
        }
    }

    config.save()?;
    formatter.format_config_set(key, value);
    Ok(())
}

fn show_path() -> Result<()> {
    let config_path = Config::config_path()?;
    println!("{}", config_path.display());
    Ok(())
}
