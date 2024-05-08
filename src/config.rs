use std::fmt::Display;

use clap::{Parser, ValueEnum};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};

const BOOTSTRAP_SERVERS: &str = "bootstrap.servers";
const GROUP_ID: &str = "group.id";
const SESSION_TIMEOUT_MS: &str = "session.timeout.ms";
const ENABLE_AUTO_COMMIT: &str = "enable.auto.commit";
const STATS_INTERVAL_MS: &str = "statistics.interval.ms";

const DEFAULT_GROUP_ID: &str = "cg.krust";

#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Debug,
    Info,
    Error,
}

impl Into<RDKafkaLogLevel> for LogLevel {
    fn into(self) -> RDKafkaLogLevel {
        match self {
            Self::Debug => RDKafkaLogLevel::Debug,
            Self::Info => RDKafkaLogLevel::Info,
            Self::Error => RDKafkaLogLevel::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigError {
    message: String
}

impl ConfigError {
    fn new(message: &str) -> ConfigError {
        ConfigError {
            message: message.to_string(),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// TUI for kafka written in Rust
#[derive(Parser, Debug)]
#[command(name = "krust")]
#[command(about = "TUI for kafka written in Rust", long_about = None)]
pub struct Config {
    /// Log level to be set for kafka client
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    /// Bootstrap servers in kafka format
    #[arg(short, long, required=true)]
    pub bootstrap_servers: String,
    
    // Consumer group ID
    #[arg(short, long, default_value = DEFAULT_GROUP_ID)]
    pub group_id: String,
}

impl TryInto<ClientConfig> for Config {
    type Error = ConfigError;

    fn try_into(self) -> Result<ClientConfig, Self::Error> {
        let mut client_config = ClientConfig::new();
        client_config.log_level = self.log_level.into();

        // bootstrap server
        if self.bootstrap_servers != "" {
            client_config.set(BOOTSTRAP_SERVERS.to_string(), self.bootstrap_servers);
        } else {
            return Err(ConfigError::new("bootstrap servers cannot be empty"));
        }

        // group id
        if self.group_id != "" {
            client_config.set(GROUP_ID.to_string(), self.group_id);
        } else {
            client_config.set(GROUP_ID.to_string(), DEFAULT_GROUP_ID.to_string());
        }

        // stats interval
        client_config.set(STATS_INTERVAL_MS, "10000");
        
        Ok(client_config)
    }
}
