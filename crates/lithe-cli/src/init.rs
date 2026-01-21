use anyhow::{Context, Result};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;
use std::{thread, time::Duration};

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Asset;

pub fn handle_init() -> Result<()> {
    let theme = ColorfulTheme::default();

    let project_dir: String = Input::with_theme(&theme)
        .with_prompt("Project directory")
        .default(".".to_string())
        .interact_text()?;

    let backends = vec!["Rust", "None", "Go", "Typescript", "Python"];
    let backend_idx = Select::with_theme(&theme)
        .with_prompt("Select a backend")
        .items(&backends)
        .default(0)
        .interact()?;

    let backend = backends[backend_idx];
    let backend_dir = backend.to_lowercase();

    let package_managers = vec!["bun", "npm"];
    let pkg_manager_idx = Select::with_theme(&theme)
        .with_prompt("Which package manager do you use?")
        .items(&package_managers)
        .default(0)
        .interact()?;
    let pkg_manager = package_managers[pkg_manager_idx];

    info!(
        "Initializing project in {} with {} backend and {}...",
        project_dir, backend, pkg_manager
    );

    let target_path = Path::new(&project_dir);
    if !target_path.exists() {
        fs::create_dir_all(target_path).context("Failed to create project directory")?;
    }

    let files_to_copy: Vec<_> = Asset::iter()
        .filter(|file| file.starts_with(&backend_dir))
        .collect();

    let total_files = files_to_copy.len();

    if total_files == 0 {
        warn!("No template files found for backend: {}", backend);
        return Ok(());
    }

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut files_written = 0;
    for file in files_to_copy {
        let relative_path = file
            .strip_prefix(&backend_dir)
            .unwrap()
            .trim_start_matches('/');
        if relative_path.is_empty() {
            pb.inc(1);
            continue;
        }

        let file_content = Asset::get(&file).context("Failed to get asset")?;
        let file_path = target_path.join(relative_path);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).context("Failed to create subdirectory")?;
        }

        fs::write(&file_path, file_content.data).context("Failed to write template file")?;
        files_written += 1;
        pb.inc(1);
        // Artificial delay to make progress bar visible for small templates
        thread::sleep(Duration::from_millis(100));
    }

    pb.finish_with_message("Done!");

    info!(
        "Project initialized successfully with {} files!",
        files_written
    );

    Ok(())
}
