use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::api::{ProwlClient, RegisterRequest};
use crate::cli::RegisterArgs;
use crate::config::ResolvedConfig;
use crate::error::Result;
use crate::output::OutputFormatter;

pub async fn execute(
    args: &RegisterArgs,
    config: &ResolvedConfig,
    formatter: &dyn OutputFormatter,
) -> Result<()> {
    let provider_key = config.require_provider_key()?;

    let request = RegisterRequest {
        providerkey: provider_key.to_string(),
        token: args.token.clone(),
    };

    let spinner = create_spinner("Retrieving API key...");

    let client = ProwlClient::new()?;
    let response = client.retrieve_apikey(&request).await;

    spinner.finish_and_clear();

    match response {
        Ok(resp) => {
            formatter.format_register_success(&resp);
            Ok(())
        }
        Err(e) => Err(e),
    }
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
