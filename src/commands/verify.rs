use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::api::{ProwlClient, VerifyRequest};
use crate::config::ResolvedConfig;
use crate::error::Result;
use crate::output::OutputFormatter;

pub async fn execute(config: &ResolvedConfig, formatter: &dyn OutputFormatter) -> Result<()> {
    let api_key = config.require_api_key()?;

    let request = VerifyRequest {
        apikey: api_key.to_string(),
        providerkey: config.provider_key.clone(),
    };

    let spinner = create_spinner("Verifying API key...");

    let client = ProwlClient::new()?;
    let response = client.verify(&request).await;

    spinner.finish_and_clear();

    match response {
        Ok(resp) => {
            formatter.format_verify_success(&resp);
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
