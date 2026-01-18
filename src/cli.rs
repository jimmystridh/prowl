use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Parser)]
#[command(
    name = "prowl",
    author,
    version,
    about = "A modern CLI for the Prowl push notification API",
    long_about = "Send push notifications to iOS devices via the Prowl API.\n\n\
        Configure your API key via the config file, PROWL_API_KEY environment variable, \
        or the --api-key flag."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output format
    #[arg(long, short = 'F', global = true, default_value = "human")]
    pub format: OutputFormat,

    /// API key (overrides config and env var)
    #[arg(long, short = 'k', global = true, env = "PROWL_API_KEY")]
    pub api_key: Option<String>,

    /// Provider key for higher rate limits
    #[arg(long, short = 'K', global = true, env = "PROWL_PROVIDER_KEY")]
    pub provider_key: Option<String>,

    /// Application name for notifications
    #[arg(long, short = 'a', global = true, env = "PROWL_APPLICATION")]
    pub application: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Send a push notification
    #[command(visible_alias = "s")]
    Send(SendArgs),

    /// Verify API key validity
    #[command(visible_alias = "v")]
    Verify,

    /// Get a registration token (for app developers)
    Token,

    /// Get API key from an approved registration token
    Register(RegisterArgs),

    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommand),

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(clap::Args)]
pub struct SendArgs {
    /// Message to send (use "-" to read from stdin)
    #[arg(default_value = "")]
    pub message: String,

    /// Event title
    #[arg(long, short = 'e', default_value = "Alert")]
    pub event: String,

    /// Priority level
    #[arg(long, short = 'p', default_value = "normal")]
    pub priority: Priority,

    /// URL to attach to notification
    #[arg(long, short = 'u')]
    pub url: Option<String>,

    /// Additional API keys to send to (comma-separated or repeated)
    #[arg(long, short = 't', value_delimiter = ',')]
    pub to: Vec<String>,

    /// Show what would be sent without actually sending
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(clap::Args)]
pub struct RegisterArgs {
    /// Registration token from `prowl token`
    #[arg(long, short = 't', required = true)]
    pub token: String,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Initialize config file with default values
    Init {
        /// Overwrite existing config file
        #[arg(long)]
        force: bool,
    },

    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key (api_key, provider_key, application)
        key: String,
        /// Configuration value
        value: String,
    },

    /// Show config file path
    Path,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Human,
    Json,
    Quiet,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Priority {
    /// Very low priority (-2)
    VeryLow,
    /// Moderate priority (-1)
    Moderate,
    #[default]
    /// Normal priority (0)
    Normal,
    /// High priority (1)
    High,
    /// Emergency priority (2)
    Emergency,
}

impl Priority {
    pub fn as_i8(self) -> i8 {
        match self {
            Priority::VeryLow => -2,
            Priority::Moderate => -1,
            Priority::Normal => 0,
            Priority::High => 1,
            Priority::Emergency => 2,
        }
    }
}
