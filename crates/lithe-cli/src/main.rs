use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};

mod init;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    version: u8,
    api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            api_key: "default_key".to_string(),
        }
    }
}

#[derive(Parser)]
#[command(name = "lithe-cli")]
#[command(about = "A demonstration of the Rust CLI stack", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initializes a new lithe project")]
    Init,
    #[command(about = "Starts the main application process")]
    Run {
        #[arg(short, long, help = "Print verbose logs")]
        verbose: bool,
    },
    #[command(about = "Shows the current configuration")]
    Config,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let cfg: AppConfig =
        confy::load("lithe-cli", "config").context("Failed to load configuration")?;

    match args.command {
        Commands::Init => {
            init::handle_init()?;
        }
        Commands::Config => {
            println!("Current Configuration:");
            println!("  Version: {}", cfg.version);
            println!("  API Key: {}", cfg.api_key);
            let location = confy::get_configuration_file_path("lithe-cli", "config")?;
            println!("  Location: {:?}", location);
        }
        Commands::Run { verbose } => {
            if verbose {
                info!("Verbose mode enabled");
            }

            info!("Starting the application logic...");

            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to start the heavy process?")
                .interact()?
            {
                warn!("Operation cancelled by user");
                return Ok(());
            }

            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
            );

            for i in 0..100 {
                if i == 50 {
                    info!("Halfway there!");
                }
                pb.inc(1);
                thread::sleep(Duration::from_millis(20));
            }

            pb.finish_with_message("Done!");
            info!("Process completed successfully.");
        }
    }

    Ok(())
}
