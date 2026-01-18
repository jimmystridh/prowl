use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, BufRead};
use std::time::Duration;

use crate::api::{ProwlClient, SendRequest};
use crate::cli::SendArgs;
use crate::config::ResolvedConfig;
use crate::error::Result;
use crate::output::OutputFormatter;

pub async fn execute(
    args: &SendArgs,
    config: &ResolvedConfig,
    formatter: &dyn OutputFormatter,
) -> Result<()> {
    let description = if args.message == "-" {
        read_stdin()?
    } else {
        args.message.clone()
    };

    let mut api_keys = vec![config.require_api_key()?.to_string()];
    api_keys.extend(args.to.iter().cloned());
    let combined_keys = api_keys.join(",");

    let request = SendRequest {
        apikey: combined_keys,
        application: config.application.clone(),
        event: args.event.clone(),
        description,
        priority: args.priority.as_i8(),
        url: args.url.clone(),
        providerkey: config.provider_key.clone(),
    };

    if args.dry_run {
        formatter.format_dry_run(&request);
        return Ok(());
    }

    let spinner = create_spinner("Sending notification...");

    let client = ProwlClient::new()?;
    let response = client.send(&request).await;

    spinner.finish_and_clear();

    match response {
        Ok(resp) => {
            formatter.format_send_success(&resp);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn read_stdin() -> Result<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line?);
    }
    Ok(lines.join("\n"))
}

fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}
