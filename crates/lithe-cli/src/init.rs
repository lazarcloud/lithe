use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Asset;

pub fn handle_init(default_project_dir: &str, default_template: &str) -> Result<()> {
    let theme = ColorfulTheme::default();

    let project_dir: String = Input::with_theme(&theme)
        .with_prompt("Project directory")
        .default(default_project_dir.to_string())
        .interact_text()?;

    let templates = vec!["Rust"];
    let default_template_index = templates
        .iter()
        .position(|&t| t.eq_ignore_ascii_case(default_template))
        .unwrap_or(0);
    let template_idx = Select::with_theme(&theme)
        .with_prompt("Select a template")
        .items(&templates)
        .default(default_template_index)
        .interact()?;

    let template = templates[template_idx];
    let template_dir = template.to_lowercase();

    let target_path = Path::new(&project_dir);
    if target_path.exists() {
        let has_content = fs::read_dir(target_path)
            .map(|mut entries| entries.next().is_some())
            .unwrap_or(false);

        if has_content {
            let should_replace = Confirm::with_theme(&theme)
                .with_prompt(format!(
                    "Directory '{}' already exists and is not empty. Replace contents?",
                    project_dir
                ))
                .default(false)
                .interact()?;

            if !should_replace {
                info!("Initialization cancelled.");
                return Ok(());
            }

            // Remove existing contents
            for entry in fs::read_dir(target_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(&path).context("Failed to remove existing directory")?;
                } else {
                    fs::remove_file(&path).context("Failed to remove existing file")?;
                }
            }
        }
    } else {
        fs::create_dir_all(target_path).context("Failed to create project directory")?;
    }

    let files_to_copy: Vec<_> = Asset::iter()
        .filter(|file| file.starts_with(&template_dir))
        .collect();

    let total_files = files_to_copy.len();

    if total_files == 0 {
        warn!("No template files found for template: {}", template);
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
            .strip_prefix(&template_dir)
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
    }

    pb.finish_with_message("Done!");

    info!(
        "Project initialized successfully with {} files and Lithe.rs structure!",
        files_written
    );

    Ok(())
}
