// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Result, Context};
use chrono::Local;
use csv::Writer;
use glob::glob;
use log::{debug, error};
use std::{env, fs::File, path::PathBuf};
use std::io::Write;
use crate::cli::Cli;
use crate::audit::EnrichedCluster;

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

pub fn write_audit_csv(result: &EnrichedCluster) -> Result<PathBuf> {
    debug!("→ Writing audit CSV file.");

    let mut path = env::temp_dir();
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    path.push(format!("{}_proxmox_audit.csv", timestamp));
    let file = File::create(&path).inspect_err(|e| error!("✗ Failed to create CSV file ({}): {}", path.display(), e)).context("Failed to create CSV file")?;
    let mut wtr = Writer::from_writer(file);

    // wtr.write_record(&[
    //     "cluster_name",
    //     "quorate",
    //     "node_id",
    //     "node_name",
    //     "node_ip",
    //     "node_nodeid",
    //     "node_online",
    //     "subscription_status",
    //     "subscription_level",
    //     "subscription_productname",
    //     "subscription_key",
    //     "subscription_serverid",
    //     "subscription_sockets",
    //     "subscription_nextduedate",
    // ])?;

    // for node in &result.nodes {
    //     let quorate = result.quorate.map(|v| v.to_string()).unwrap_or_default();
    //     let nodeid = node.nodeid.map(|v| v.to_string()).unwrap_or_default();
    //     let online = node.online.map(|v| v.to_string()).unwrap_or_default();
    //     let sub = node.subscription.as_ref();

    //     wtr.write_record(&[
    //         result.cluster_name.as_deref().unwrap_or(""),
    //         quorate.as_str(),
    //         node.id.as_str(),
    //         node.name.as_str(),
    //         node.ip.as_deref().unwrap_or(""),
    //         nodeid.as_str(),
    //         online.as_str(),
    //         sub.map(|s| sub_str(&s.status)).unwrap_or(""),
    //         sub.map(|s| sub_str(&s.level)).unwrap_or(""),
    //         sub.map(|s| sub_str(&s.productname)).unwrap_or(""),
    //         sub.map(|s| sub_str(&s.key)).unwrap_or(""),
    //         sub.map(|s| sub_str(&s.serverid)).unwrap_or(""),
    //         sub.map(|s| sub_u8(s.sockets)).unwrap_or_default().as_str(),
    //         sub.map(|s| sub_str(&s.nextduedate)).unwrap_or(""),
    //     ])?;
    // }

    wtr.write_record(&[
        "id",
        "valid_server_id",
        "socket",
        "hostname",
        "clustername",
        "expiration_date",
        "registration_date",
        "partner/id",
        "sale_subscription/id",
        "sale_order/id",
    ])?;

    for node in &result.nodes {
        let sub = node.subscription.as_ref();

        wtr.write_record(&[
            "", // id
            sub.map(|s| sub_str(&s.serverid)).unwrap_or(""),
            &sub.map(|s| sub_u8(s.sockets)).unwrap_or_default(),
            node.name.as_str(),
            result.cluster_name.as_deref().unwrap_or(""),
            sub.map(|s| sub_str(&s.nextduedate)).unwrap_or(""),
            sub.map(|s| sub_str(&s.regdate)).unwrap_or(""),
            "", // partner/id
            "", // sale_subscription/id
            "", // sale_order/id
        ])?;
    }

    wtr.flush()?;

    debug!("✓ Wrote CSV file to: {}.", path.display());
    println!("✓ Wrote CSV file to: {}.", path.display());

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

fn sub_str(opt: &Option<String>) -> &str {
    opt.as_deref().unwrap_or("")
}

fn sub_u8(opt: Option<u8>) -> String {
    opt.map(|v| v.to_string()).unwrap_or_default()
}