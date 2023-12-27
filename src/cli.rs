use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Database URL
    #[arg(long, value_name = "URL")]
    pub db_url: Option<String>,

    /// Path to PWMP server configuration
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Instance name
    #[arg(short, long, value_name = "NAME")]
    pub instance_name: Option<String>,

    /// Include stats?
    #[arg(short, long, default_value_t = false)]
    pub stats: bool,

    /// Verify generated quieries?
    #[arg(short, long, default_value_t = false)]
    pub verify: bool,

    /// Output
    pub out: PathBuf,
}
