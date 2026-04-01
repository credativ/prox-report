// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Result, Context};
use chrono::Local;
use crate::cli::Cli;
use crate::audit::EnrichedCluster;
use glob::glob;
use log::{debug, error};
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::PathBuf;

pub fn write_audit(result: &EnrichedCluster) -> Result<PathBuf> {
    debug!("→ Writing audit file.");

    let mut path = env::temp_dir();
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    path.push(format!("{}_proxmox_audit.json", timestamp));

    let mut file = File::create(&path).inspect_err(|e| error!("✗ Failed to create audit file ({}): {}", path.display(), e)).context("Failed to create audit file")?;
    let json = serde_json::to_string_pretty(result).inspect_err(|e| error!("✗ Failed to serialize audit JSON: {}", e)).context("Failed to serialize audit JSON")?;
    file.write_all(json.as_bytes()).inspect_err(|e| error!("✗ Failed to write audit JSON ({}): {}", path.display(), e)).context("Failed to write audit JSON")?;

    debug!("✓ Wrote audit file to: {}.", path.display());
    println!("✓ Wrote audit file to: {}.", path.display());

    Ok(path)
}

pub fn create_file_glob(cli: &Cli) -> Result<Vec<PathBuf>> {
    debug!("→ Creating file glob of Proxmox licenses.");

    let input_path = cli.input_path.as_ref().context("No input path to license files provided")?;
    let pattern = format!("{}/*.txt", input_path);
    let mut files = Vec::new();

    for entry in glob(&pattern).context("Failed to read glob pattern")? {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => eprintln!("✗ Glob error: {}", e),
        }
    }

    Ok(files)
}