// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use log::{debug};
use crate::Cli;
use crate::audit::Subscription;
use crate::audit::EnrichedCluster;
use crate::audit::EnrichedClusterNode;
use crate::utils::create_file_glob;

pub fn exec_merge_licenses(cli: &Cli) -> Result<EnrichedCluster> {
    debug!("→ Starting Proxmox license merging...");

    let files = create_file_glob(cli)?;

    let mut nodes = Vec::new();
    let mut cluster_name = None;
    let mut quorate = None;

    for file in files {
        debug!("→ Including file: {}", file.display());

        let content = std::fs::read_to_string(&file)?;
        let node = parse_node_content(&content);nodes.push(node);
        let (c_name, q) = parse_cluster_info(&content);

        if cluster_name.is_none() {
            cluster_name = c_name;
        }

        if quorate.is_none() {
            quorate = q;
        }
    }

    Ok(EnrichedCluster {
        cluster_name,
        quorate,
        nodes,
    })
}

fn parse_node_content(content: &str) -> EnrichedClusterNode {
    debug!("→ Parsing node content from config...");

    let hostname = parse_hostname(content).unwrap_or_default();
    let subscription = parse_subscription(content);

    EnrichedClusterNode {
        id: hostname.clone(),
        name: hostname,
        ip: None,
        nodeid: None,
        online: Some(1),
        subscription: Some(subscription),
    }
}

fn parse_subscription(content: &str) -> Subscription {
    debug!("→ Parsing subscription from config...");

    let mut sub = Subscription {
        status: None,
        level: None,
        productname: None,
        key: None,
        serverid: None,
        sockets: None,
        nextduedate: None,
    };

    let mut in_section = false;

    for line in content.lines() {
        if line.trim() == "# pvesubscription get" {
            in_section = true;
            continue;
        }

        if in_section {
            if line.starts_with('#') || line.starts_with("====") {
                break;
            }

            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() != 2 {
                continue;
            }

            match parts[0] {
                "status" => sub.status = Some(parts[1].to_string()),
                "level" => sub.level = Some(parts[1].to_string()),
                "productname" => sub.productname = Some(parts[1].to_string()),
                "key" => sub.key = Some(parts[1].to_string()),
                "serverid" => sub.serverid = Some(parts[1].to_string()),
                "sockets" => sub.sockets = parts[1].parse().ok(),
                "nextduedate" => sub.nextduedate = Some(parts[1].to_string()),
                _ => {}
            }
        }
    }

    sub
}

fn parse_hostname(content: &str) -> Option<String> {
    debug!("→ Parsing hostname from config...");

    let mut lines = content.lines();

    while let Some(line) = lines.next() {
        if line.trim() == "# hostname" {
            return lines.next().map(|s| s.trim().to_string());
        }
    }

    None
}

fn parse_cluster_info(content: &str) -> (Option<String>, Option<u8>) {
    debug!("→ Parsing cluster information from config...");

    let mut cluster_name = None;
    let mut quorate = None;

    for line in content.lines() {
        if line.trim().starts_with("Name:") {
            cluster_name = line.split(':').nth(1).map(|s| s.trim().to_string());
        }

        if line.trim().starts_with("Quorate:") {
            quorate = Some(if line.contains("Yes") { 1 } else { 0 });
        }
    }

    (cluster_name, quorate)
}