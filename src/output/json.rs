use serde_json::json;

use crate::api::{ApiResponse, SendRequest};
use crate::config::Config;
use crate::error::ProwlError;
use crate::output::OutputFormatter;

pub struct JsonOutput;

impl OutputFormatter for JsonOutput {
    fn format_send_success(&self, response: &ApiResponse) {
        let output = json!({
            "success": true,
            "action": "send",
            "remaining": response.remaining,
            "reset_date": response.reset_date,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_verify_success(&self, response: &ApiResponse) {
        let output = json!({
            "success": true,
            "action": "verify",
            "valid": true,
            "remaining": response.remaining,
            "reset_date": response.reset_date,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_token_success(&self, response: &ApiResponse) {
        let output = json!({
            "success": true,
            "action": "token",
            "token": response.token,
            "approval_url": response.token_url,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_register_success(&self, response: &ApiResponse) {
        let output = json!({
            "success": true,
            "action": "register",
            "api_key": response.apikey,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_error(&self, error: &ProwlError) {
        let output = json!({
            "success": false,
            "error": error.to_string(),
            "exit_code": error.exit_code(),
        });
        eprintln!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_dry_run(&self, request: &SendRequest) {
        let output = json!({
            "dry_run": true,
            "request": {
                "application": request.application,
                "event": request.event,
                "description": request.description,
                "priority": request.priority,
                "url": request.url,
                "api_key_count": request.apikey.split(',').count(),
            }
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_config_show(&self, config: &Config, path: &std::path::Path) {
        let output = json!({
            "path": path.display().to_string(),
            "api_key": config.api_key.as_ref().map(|k| mask_key(k)),
            "provider_key": config.provider_key.as_ref().map(|k| mask_key(k)),
            "application": config.application,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_config_init(&self, path: &std::path::Path) {
        let output = json!({
            "success": true,
            "action": "config_init",
            "path": path.display().to_string(),
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_config_set(&self, key: &str, value: &str) {
        let display_value = if key.contains("key") {
            mask_key(value)
        } else {
            value.to_string()
        };
        let output = json!({
            "success": true,
            "action": "config_set",
            "key": key,
            "value": display_value,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        "*".repeat(key.len())
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
