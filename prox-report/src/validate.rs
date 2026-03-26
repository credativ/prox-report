// SPDX-License-Identifier: GPL-3.0-or-later

use crate::audit::EnrichedCluster;
use log::{info, debug, error};

pub fn exec_validate(cluster: &EnrichedCluster) -> Result<(), Box<dyn std::error::Error>> {
    debug!("→ Starting Proxmox Cluster validation...");

    if cluster.quorate != Some(1) {
        error!("✗ Cluster is not quorate!");
        return Err("Cluster is not quorate!".into());
    } else {
        debug!("✓ Cluster is quorate!");
    }

    if cluster.nodes.len() < 3 {
        error!("✗ Not enough nodes: expected at least 3, got {}!", cluster.nodes.len());
        return Err(format!("Not enough nodes: expected at least 3, got {}!", cluster.nodes.len()).into());
    } else {
        debug!("✓ Cluster has {} nodes!", cluster.nodes.len());
    }

    if !cluster.nodes.iter().all(|n| n.online == Some(1)) {
        error!("✗ Not all nodes are online!");
        return Err("Not all nodes are online!".into());
    } else {
        debug!("✓ All nodes are online!");
    }

    let invalid_nodes: Vec<_> = cluster.nodes.iter().filter(|n| n.subscription_status.as_deref() == Some("notfound")).map(|n| n.name.clone()).collect();
    if !invalid_nodes.is_empty() {
        error!("✗ Nodes without subscription: {:?}", invalid_nodes);
        return Err(format!("Nodes without subscription: {:?}", invalid_nodes).into());
    } else {
        debug!("✓ All nodes have a valid subscription!");
    }

    debug!("✓ Cluster validation successful!");
    println!("✓ Cluster validation successful!");
    Ok(())
}