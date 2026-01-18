use colored::Colorize;

use crate::api::{ApiResponse, SendRequest};
use crate::config::Config;
use crate::error::ProwlError;
use crate::output::OutputFormatter;

pub struct HumanOutput;

impl OutputFormatter for HumanOutput {
    fn format_send_success(&self, response: &ApiResponse) {
        println!("{} Notification sent successfully", "✓".green().bold());
        if let Some(remaining) = response.remaining {
            println!("  {} API calls remaining", remaining.to_string().cyan());
        }
    }

    fn format_verify_success(&self, response: &ApiResponse) {
        println!("{} API key is valid", "✓".green().bold());
        if let Some(remaining) = response.remaining {
            println!("  {} API calls remaining", remaining.to_string().cyan());
        }
        if let Some(ref reset) = response.reset_date {
            println!("  Resets at: {}", reset.cyan());
        }
    }

    fn format_token_success(&self, response: &ApiResponse) {
        println!("{} Registration token retrieved", "✓".green().bold());
        if let Some(ref token) = response.token {
            println!("\n  Token: {}", token.yellow().bold());
        }
        if let Some(ref url) = response.token_url {
            println!("\n  Approval URL: {}", url.cyan());
            println!(
                "\n  {}",
                "User must visit the URL above to approve the token.".dimmed()
            );
            println!(
                "  {}",
                "Then use 'prowl register --token <token>' to get the API key.".dimmed()
            );
        }
    }

    fn format_register_success(&self, response: &ApiResponse) {
        println!("{} API key retrieved successfully", "✓".green().bold());
        if let Some(ref apikey) = response.apikey {
            println!("\n  API Key: {}", apikey.yellow().bold());
            println!(
                "\n  {}",
                "Save this key securely. You can add it to your config with:".dimmed()
            );
            println!("  {}", format!("prowl config set api_key {apikey}").cyan());
        }
    }

    fn format_error(&self, error: &ProwlError) {
        eprintln!("{} {}", "Error:".red().bold(), error);
    }

    fn format_dry_run(&self, request: &SendRequest) {
        println!("{} Dry run - would send:", "[DRY RUN]".yellow().bold());
        println!("  Application: {}", request.application.cyan());
        println!("  Event:       {}", request.event.cyan());
        println!("  Description: {}", request.description.cyan());
        println!("  Priority:    {}", request.priority.to_string().cyan());
        if let Some(ref url) = request.url {
            println!("  URL:         {}", url.cyan());
        }
        println!(
            "  API Keys:    {}",
            format!("{} key(s)", request.apikey.split(',').count()).cyan()
        );
    }

    fn format_config_show(&self, config: &Config, path: &std::path::Path) {
        println!("{} Configuration", "●".cyan().bold());
        println!("  Path: {}", path.display().to_string().dimmed());
        println!();

        if let Some(ref key) = config.api_key {
            let masked = mask_key(key);
            println!("  api_key:      {}", masked.green());
        } else {
            println!("  api_key:      {}", "(not set)".dimmed());
        }

        if let Some(ref key) = config.provider_key {
            let masked = mask_key(key);
            println!("  provider_key: {}", masked.green());
        } else {
            println!("  provider_key: {}", "(not set)".dimmed());
        }

        if let Some(ref app) = config.application {
            println!("  application:  {}", app.green());
        } else {
            println!("  application:  {}", "(not set)".dimmed());
        }
    }

    fn format_config_init(&self, path: &std::path::Path) {
        println!(
            "{} Config file created at {}",
            "✓".green().bold(),
            path.display().to_string().cyan()
        );
        println!(
            "\n  {}",
            "Edit the file or use 'prowl config set' to configure.".dimmed()
        );
    }

    fn format_config_set(&self, key: &str, value: &str) {
        let display_value = if key.contains("key") {
            mask_key(value)
        } else {
            value.to_string()
        };
        println!(
            "{} Set {} = {}",
            "✓".green().bold(),
            key.cyan(),
            display_value.green()
        );
    }
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        "*".repeat(key.len())
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
