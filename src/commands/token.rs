use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::api::{ProwlClient, TokenRequest};
use crate::config::ResolvedConfig;
use crate::error::Result;
use crate::output::OutputFormatter;

pub async fn execute(config: &ResolvedConfig, formatter: &dyn OutputFormatter) -> Result<()> {
    let provider_key = config.require_provider_key()?;

    let request = TokenRequest {
        providerkey: provider_key.to_string(),
    };

    let spinner = create_spinner("Retrieving registration token...");

    let client = ProwlClient::new()?;
    let response = client.retrieve_token(&request).await;

    spinner.finish_and_clear();

    match response {
        Ok(resp) => {
            formatter.format_token_success(&resp);
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
