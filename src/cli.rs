use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Database host
    #[arg(value_name = "HOST")]
    pub host: String,

    /// Database port
    #[arg(long, value_name = "PORT", default_value_t = 5432)]
    pub port: u16,

    /// Database username
    #[arg(short, long, value_name = "USERNAME")]
    pub username: String,

    /// Database password
    #[arg(short, long, value_name = "PASSWORD")]
    pub password: String,

    /// Database name
    #[arg(short, long, value_name = "DATABASE")]
    pub database: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate YAML configuration
    Generate {
        /// Include stats?
        #[arg(short, long, default_value_t = false)]
        stats: bool,

        /// Verify generated quieries?
        #[arg(short, long, default_value_t = false)]
        verify: bool,

        /// List of Node IDs to skip (=1,2,...)
        #[arg(long, value_parser, num_args = 1.., value_delimiter = ',', value_name = "1,2")]
        skip: Vec<u16>,

        /// Output file
        #[arg(short, long, value_name = "FILE")]
        out: Option<PathBuf>,
    },
}
