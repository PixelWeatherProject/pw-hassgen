use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Database Host
    #[arg(short, long, value_name = "HOST")]
    pub host: String,

    /// Database Port
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

    /// Include stats?
    #[arg(short, long, default_value_t = false)]
    pub stats: bool,

    /// Verify generated quieries?
    #[arg(short, long, default_value_t = false)]
    pub verify: bool,

    /// List of Node IDs to skip (=1,2,...)
    #[arg(long, value_parser, num_args = 1.., value_delimiter = ',', value_name = "1,2")]
    pub skip: Vec<u16>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate YAML configuration
    Generate {
        /// Output file
        #[arg(short, long, value_name = "FILE")]
        out: Option<PathBuf>,
    },
}
