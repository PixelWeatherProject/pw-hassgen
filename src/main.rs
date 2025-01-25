use crate::cli::Cli;
use clap::Parser;
use log::{debug, info};
use sqlx::Connection;
use std::{
    error::Error,
    fs::OpenOptions,
    io::{stdout, Write},
};

mod cli;
mod generator;
mod hass;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let args = Cli::parse();
    debug!("Arguments: {args:?}");

    info!("Connecting to database @ {}", args.host);
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        args.username, args.password, args.host, args.port, args.database
    );

    let db = sqlx::PgConnection::connect(&db_url).await?;
    debug!("Connected to database");

    match args.command {
        cli::Command::Generate {
            stats,
            verify,
            skip,
            out,
        } => {
            let out: &mut dyn Write = match out {
                Some(file) => &mut OpenOptions::new().write(true).open(file)?,
                None => &mut stdout(),
            };

            generator::run(stats, verify, &skip, db, &db_url, out).await?
        }
    }

    Ok(())
}
