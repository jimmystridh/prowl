mod api;
mod cli;
mod commands;
mod config;
mod error;
mod output;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io;

use cli::{Cli, Command};
use config::ResolvedConfig;
use output::get_formatter;

#[tokio::main]
async fn main() {
    color_eyre::install().ok();

    let cli = Cli::parse();
    let formatter = get_formatter(cli.format);

    let result = run(cli, formatter.as_ref()).await;

    if let Err(e) = result {
        formatter.format_error(&e);
        std::process::exit(e.exit_code());
    }
}

async fn run(cli: Cli, formatter: &dyn output::OutputFormatter) -> error::Result<()> {
    let config = ResolvedConfig::resolve(
        cli.api_key.as_deref(),
        cli.provider_key.as_deref(),
        cli.application.as_deref(),
    )?;

    match &cli.command {
        Command::Send(args) => commands::send::execute(args, &config, formatter).await,
        Command::Verify => commands::verify::execute(&config, formatter).await,
        Command::Token => commands::token::execute(&config, formatter).await,
        Command::Register(args) => commands::register::execute(args, &config, formatter).await,
        Command::Config(cmd) => commands::config_cmd::execute(cmd, formatter),
        Command::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(*shell, &mut cmd, "prowl", &mut io::stdout());
            Ok(())
        }
    }
}
