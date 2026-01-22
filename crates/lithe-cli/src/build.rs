use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::dev::ensure_cargo_bin_config;
use crate::generate;
use crate::server;

pub fn handle_build(static_site: bool, out_dir: &str) -> Result<()> {
    let project_dir = std::env::current_dir()?;
    info!("Generating routes...");

    generate::generate_all(&project_dir, 3000)?;
    sync_public_assets(&project_dir)?;
    build_wasm_unified(&project_dir)?;

    if static_site {
        build_static_site(&project_dir, out_dir)?;
    } else {
        build_binary(&project_dir, out_dir)?;
    }

    Ok(())
}

pub fn build_wasm_unified(project_dir: &Path) -> Result<()> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;
    let project_name = cargo_toml_content
        .lines()
        .find(|l| l.starts_with("name = "))
        .map(|l| l.split('=').nth(1).unwrap().trim().trim_matches('"'))
        .unwrap_or("app");
    info!("Scanning for client and server functions...");

    let (client_fns, server_fns) = server::discover_functions(project_dir, project_name)?;
    if client_fns.is_empty() && server_fns.is_empty() {
        info!("No client or server functions found, skipping WASM build");
        return Ok(());
    }

    info!(
        "Found {} client and {} server functions",
        client_fns.len(),
        server_fns.len()
    );
    server::generate_wasm_exports(project_dir, project_name, &client_fns, &server_fns)?;

    info!("Building unified WASM bundle for {}...", project_name);

    let pkg_out_dir = project_dir.join(".lithe/public/pkg");
    fs::create_dir_all(&pkg_out_dir)?;

    let status = Command::new("wasm-pack")
        .args([
            "build",
            "--target",
            "web",
            "--out-dir",
            pkg_out_dir.to_str().unwrap(),
        ])
        .current_dir(project_dir)
        .status()
        .context("Failed to build unified WASM bundle")?;
    if !status.success() {
        anyhow::bail!("wasm-pack build failed");
    }
    info!("WASM bundle built successfully to .lithe/public/pkg");
    Ok(())
}

pub fn sync_public_assets(project_dir: &Path) -> Result<()> {
    let src_public = project_dir.join("src/public");
    let lithe_public = project_dir.join(".lithe/public");

    if src_public.exists() {
        copy_dir_all(&src_public, &lithe_public)?;
        info!("Synced src/public to .lithe/public");
    }
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn build_binary(project_dir: &Path, out_dir: &str) -> Result<()> {
    info!("Building embedded binary to {}...", out_dir);
    ensure_cargo_bin_config(project_dir)?;

    let status = Command::new("cargo")
        .args(["build", "--release", "--bin", "lithe-app"])
        .current_dir(project_dir)
        .status()
        .context("Failed to build binary")?;

    if status.success() {
        fs::create_dir_all(out_dir)?;

        // Find the binary. It might be in project_dir/target or workspace_root/target
        let mut target_binary = project_dir.join("target/release/lithe-app");
        if !target_binary.exists() {
            // Try looking up for workspace target
            if let Some(parent) = project_dir.parent() {
                if let Some(ws_root) = parent.parent() {
                    let workspace_target = ws_root.join("target/release/lithe-app");
                    if workspace_target.exists() {
                        target_binary = workspace_target;
                    }
                }
            }
        }

        let dest_binary = Path::new(out_dir).join("lithe-app");
        if target_binary.exists() {
            fs::copy(&target_binary, &dest_binary)?;
            info!(
                "Binary written to {} (all assets embedded inside)",
                dest_binary.display()
            );
        } else {
            anyhow::bail!("Could not find built binary at {}", target_binary.display());
        }
    }

    Ok(())
}

fn build_static_site(project_dir: &Path, out_dir: &str) -> Result<()> {
    info!("Building static site to {}...", out_dir);

    let lithe_dir = project_dir.join(".lithe");

    let builder_content = generate_static_builder_content(out_dir);
    fs::write(lithe_dir.join("static_builder.rs"), &builder_content)?;

    let cargo_path = project_dir.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_path)?;

    let builder_bin = r#"

[[bin]]
name = "lithe-static-builder"
path = ".lithe/static_builder.rs"
"#;

    if !content.contains("lithe-static-builder") {
        fs::write(&cargo_path, format!("{}{}", content, builder_bin))?;
    }

    let status = Command::new("cargo")
        .args(["run", "--bin", "lithe-static-builder"])
        .current_dir(project_dir)
        .status()
        .context("Failed to run static builder")?;

    if status.success() {
        info!("Static site built successfully to {}", out_dir);
    } else {
        warn!("Static build failed");
    }

    Ok(())
}

fn generate_static_builder_content(out_dir: &str) -> String {
    format!(
        r#"// Auto-generated by lithe-cli - do not edit manually
#[path = "routes.rs"]
mod routes;
fn main() {{
    let out_dir = "{out_dir}";
    let _ = std::fs::remove_dir_all(out_dir);

    let public_dir = std::path::Path::new(".lithe/public");
    if public_dir.exists() {{
        let dest = std::path::Path::new(out_dir).join("public");
        copy_dir_all(public_dir, dest).unwrap();
        println!("Copied .lithe/public to {{}}/public", out_dir);
    }}
    let routes = routes::routes();
    for route in routes {{
        let content = routes::dispatch(route);
        let path = if route == "/" {{
            format!("{{}}/index.html", out_dir)
        }} else {{
            format!("{{}}{{}}/index.html", out_dir, route)
        }};
        let dir = std::path::Path::new(&path).parent().unwrap();
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, content).unwrap();
        println!("Wrote {{}}", path);
    }}
}}
fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> std::io::Result<()> {{
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {{
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {{
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }} else {{
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }}
    }}
    Ok(())
}}
"#,
        out_dir = out_dir
    )
}
