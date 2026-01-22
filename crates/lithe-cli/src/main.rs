use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod build;
mod dev;
mod generate;
mod init;
mod server;

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
#[command(name = "lithe")]
#[command(about = "The Lithe.rs framework CLI", long_about = None)]
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
    #[command(about = "Shows the current configuration")]
    Config,
    #[command(about = "Starts the development server with hot reloading")]
    Dev {
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    #[command(about = "Builds the project for production")]
    Build {
        #[arg(short, long, help = "Output directory", default_value = "dist")]
        out_dir: String,
        #[arg(long, help = "Build as a static site only")]
        static_site: bool,
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
            dev::handle_dev(port)?;
        }
        Commands::Build {
            static_site,
            out_dir,
        } => {
            build::handle_build(static_site, &out_dir)?;
        }
    }

    Ok(())
}
