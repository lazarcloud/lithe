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
    Init {
        #[arg(short, long, help = "Project directory", default_value = "my-app")]
        project_dir: String,
        #[arg(short, long, help = "Template to use", default_value = "Rust")]
        template: String,
    },
    #[command(about = "Starts the main application process")]
    Run {
        #[arg(short, long, help = "Print verbose logs")]
        verbose: bool,
    },
    #[command(about = "Shows the current configuration")]
    Config,
    #[command(about = "Starts the development server with hot reloading")]
    Dev {
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    #[command(about = "Builds the project for production")]
    Build {
        #[arg(short, long, help = "Build a standalone embedded binary")]
        bin: bool,
        #[arg(short, long, help = "Output directory", default_value = "dist")]
        out_dir: String,
    },
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let cfg: AppConfig =
        confy::load("lithe-cli", "config").context("Failed to load configuration")?;

    match args.command {
        Commands::Init {
            project_dir,
            template,
        } => {
            init::handle_init(&project_dir, &template)?;
        }
        Commands::Config => {
            println!("Current Configuration:");
            println!("  Version: {}", cfg.version);
            println!("  API Key: {}", cfg.api_key);
            let location = confy::get_configuration_file_path("lithe-cli", "config")?;
            println!("  Location: {:?}", location);
        }
        Commands::Dev { port } => {
            info!("Starting development server on port {}...", port);
            // TODO: Implement watcher and dev server
        }
        Commands::Build { bin, out_dir } => {
            if bin {
                info!("Building embedded binary to {}...", out_dir);
            } else {
                info!("Building static site to {}...", out_dir);
            }
            // TODO: Implement build logic
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
