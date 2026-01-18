mod human;
mod json;

use crate::api::ApiResponse;
use crate::cli::OutputFormat;

pub use human::HumanOutput;
pub use json::JsonOutput;

pub trait OutputFormatter {
    fn format_send_success(&self, response: &ApiResponse);
    fn format_verify_success(&self, response: &ApiResponse);
    fn format_token_success(&self, response: &ApiResponse);
    fn format_register_success(&self, response: &ApiResponse);
    fn format_error(&self, error: &crate::error::ProwlError);
    fn format_dry_run(&self, request: &crate::api::SendRequest);
    fn format_config_show(&self, config: &crate::config::Config, path: &std::path::Path);
    fn format_config_init(&self, path: &std::path::Path);
    fn format_config_set(&self, key: &str, value: &str);
}

pub fn get_formatter(format: OutputFormat) -> Box<dyn OutputFormatter> {
    match format {
        OutputFormat::Human => Box::new(HumanOutput),
        OutputFormat::Json => Box::new(JsonOutput),
        OutputFormat::Quiet => Box::new(QuietOutput),
    }
}

struct QuietOutput;

impl OutputFormatter for QuietOutput {
    fn format_send_success(&self, _response: &ApiResponse) {}
    fn format_verify_success(&self, _response: &ApiResponse) {}
    fn format_token_success(&self, _response: &ApiResponse) {}
    fn format_register_success(&self, _response: &ApiResponse) {}
    fn format_error(&self, _error: &crate::error::ProwlError) {}
    fn format_dry_run(&self, _request: &crate::api::SendRequest) {}
    fn format_config_show(&self, _config: &crate::config::Config, _path: &std::path::Path) {}
    fn format_config_init(&self, _path: &std::path::Path) {}
    fn format_config_set(&self, _key: &str, _value: &str) {}
}
