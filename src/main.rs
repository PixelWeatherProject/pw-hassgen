use crate::{cfg_parse::ServerConfig, cli::Cli};
use clap::Parser;
use sqlx::Connection;
use std::{error::Error, fs, process::exit};

mod cfg_gen;
mod cfg_parse;
mod cli;
mod hass_cfg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let db_url = match (args.db_url, args.config) {
        (Some(db_url), None) => db_url,
        (None, Some(config_path)) => {
            let config = fs::read_to_string(config_path)?;
            let config: ServerConfig = serde_yaml::from_str(&config)?;

            format!(
                "postgresql://{}:{}@{}:{}/{}",
                config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
            )
        }
        (Some(_), Some(_)) => {
            eprintln!(
                "Invalid arguments: `db_url` and `config` are both set, please use either one"
            );
            exit(1);
        }
        (None, None) => {
            eprintln!("No database URL set: Please set `db_url` or `config`");
            exit(1);
        }
    };

    let db = sqlx::PgConnection::connect(&db_url).await?;
    let config = cfg_gen::generate_config(&db_url, db, args.stats, args.verify, &args.skip).await?;
    let mut yaml = serde_yaml::to_string(&config)?;

    if let Some(name) = args.instance_name {
        yaml.replace_range(..4, &format!("sql {name}:"));
    }

    if args.out.as_os_str() == "-" {
        println!("{yaml}");
    } else {
        fs::write(args.out, yaml)?;
    }

    Ok(())
}
