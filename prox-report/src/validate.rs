// SPDX-License-Identifier: GPL-3.0-or-later

use crate::audit::EnrichedCluster;
use log::{debug, error};

pub fn exec_validate(cluster: &EnrichedCluster) -> Result<(), Box<dyn std::error::Error>> {
    let mut error_count = 0;

    debug!("→ Starting Proxmox Cluster validation...");

    if cluster.quorate != Some(1) {
        error_count +=1;
        error!("✗ Cluster is not quorate!");
    } else {
        debug!("✓ Cluster is quorate!");
    }

    if cluster.nodes.len() < 3 {
        error_count +=1;
        error!("✗ Not enough nodes: expected at least 3, got {}!", cluster.nodes.len());
    } else {
        debug!("✓ Cluster has {} nodes!", cluster.nodes.len());
    }

    if !cluster.nodes.iter().all(|n| n.online == Some(1)) {
        error_count +=1;
        error!("✗ Not all nodes are online!");
    } else {
        debug!("✓ All nodes are online!");
    }

    let invalid_nodes: Vec<_> = cluster.nodes.iter().filter(|n| n.subscription.as_ref().and_then(|s| s.status.as_deref()) == Some("notfound")).map(|n| n.name.clone()).collect();
    if !invalid_nodes.is_empty() {
        error_count +=1;
        error!("✗ Nodes without subscription: {:?}", invalid_nodes);
    } else {
        debug!("✓ All nodes have a valid subscription!");
    }

    if error_count > 0 {
        error!("✗ Cluster is not valid, having {} error(s)!", error_count);
        return Err(format!("✗ Cluster is not valid, having {} error(s)!", error_count).into());
    }

    debug!("✓ Cluster validation successful!");
    println!("✓ Cluster validation successful!");
    Ok(())

}