// SPDX-License-Identifier: GPL-3.0-or-later

use crate::Cli;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::process::Command;
use log::{debug};

#[derive(Debug, Deserialize)]
struct Node {
    node: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub status: Option<String>,
    pub level: Option<String>,
    pub productname: Option<String>,
    pub key: Option<String>,
    pub serverid: Option<String>,
    pub sockets: Option<u8>,
    pub nextduedate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClusterStatusEntry {
    id: String,
    name: Option<String>,
    ip: Option<String>,
    nodeid: Option<u32>,
    online: Option<u8>,
    quorate: Option<u8>,
    r#type: String,
}

#[derive(Debug, Serialize)]
pub struct EnrichedCluster {
    pub cluster_name: Option<String>,
    pub quorate: Option<u8>,
    pub nodes: Vec<EnrichedClusterNode>,
}

#[derive(Debug, Serialize)]
pub struct EnrichedClusterNode {
    pub id: String,
    pub name: String,
    pub ip: Option<String>,
    pub nodeid: Option<u32>,
    pub online: Option<u8>,
    pub subscription: Option<Subscription>,
}

// #[derive(Debug, Serialize)]
// pub struct EnrichedClusterNodeSubscription {
//     pub status: Option<String>,
//     pub level: Option<String>,
//     pub productname: Option<String>,
//     pub key: Option<String>,
//     pub serverid: Option<String>,
//     pub sockets: Option<String>,
//     pub nextduedate: Option<String>,
// }

pub fn exec_audit(cli: &Cli) -> Result<EnrichedCluster, Box<dyn std::error::Error>> {
    debug!("→ Starting Proxmox Cluster audit...");

    if !cli.remote {
        return exec_audit_local();
    }

    exec_audit_remote(cli)
}

fn exec_audit_local() -> Result<EnrichedCluster, Box<dyn std::error::Error>> {
    debug!("→ Executing audit in local moe...");

    let nodes_json = exec_pvesh(&["get", "/nodes", "--output-format", "json"])?;
    let nodes: Vec<Node> = serde_json::from_str(&nodes_json)?;

    let mut subscriptions = std::collections::HashMap::new();

    for node in &nodes {
        let path = format!("/nodes/{}/subscription", node.node);
        let sub_json = exec_pvesh(&["get", &path, "--output-format", "json"])?;
        let sub: Subscription = serde_json::from_str(&sub_json)?;
        subscriptions.insert(node.node.clone(), sub);
    }

    let cluster_json =
        exec_pvesh(&["get", "/cluster/status", "--output-format", "json"])?;
    let cluster: Vec<ClusterStatusEntry> = serde_json::from_str(&cluster_json)?;

    let mut cluster_name = None;
    let mut quorate = None;
    let mut enriched_nodes = Vec::new();

    for entry in cluster {
        match entry.r#type.as_str() {
            "cluster" => {
                cluster_name = entry.name;
                quorate = entry.quorate;
            }
            "node" => {
                let node_name = entry.name.clone().unwrap_or_default();
                let sub = subscriptions.get(&node_name);

                enriched_nodes.push(EnrichedClusterNode {
                    id: entry.id,
                    name: node_name.clone(),
                    ip: entry.ip,
                    nodeid: entry.nodeid,
                    online: entry.online,
                    subscription: Some(Subscription {
                        status: sub.and_then(|s| s.status.clone()),
                        level: sub.and_then(|s| s.level.clone()),
                        productname: sub.and_then(|s| s.productname.clone()),
                        key: sub.and_then(|s| s.key.clone()),
                        serverid: sub.and_then(|s| s.serverid.clone()),
                        sockets: sub.and_then(|s| s.sockets.clone()),
                        nextduedate: sub.and_then(|s| s.nextduedate.clone()),
                    }),
                });
            }
            _ => {}
        }
    }

    let result = EnrichedCluster {
        cluster_name,
        quorate,
        nodes: enriched_nodes,
    };

    Ok(result)
}

fn exec_pvesh(args: &[&str]) -> Result<String, String> {
    debug!("→ Executing pvesh for {:?}", args);

    let output = Command::new("pvesh")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute pvesh: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn exec_audit_remote(cli: &Cli) -> Result<EnrichedCluster, Box<dyn std::error::Error>> {
    debug!("→ Executing audit in remote mode...");

    let host = cli.rhost.as_ref().ok_or("Missing rhost")?;
    let token_id = cli.rtokenid.as_ref().ok_or("Missing rtokenid")?;
    let token_secret = cli.rsecret.as_ref().ok_or("Missing rsecret")?;
    let client = Client::builder().danger_accept_invalid_certs(cli.rinsecure).build()?;
    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION,HeaderValue::from_str(&format!("PVEAPIToken={}={}",token_id, token_secret))?,);

    #[derive(Deserialize)]
    struct ApiResponse<T> {
        data: T,
    }

    let nodes_json = exec_api(&client, &headers, host, "/nodes")?;
    let nodes: Vec<Node> = serde_json::from_str::<ApiResponse<Vec<Node>>>(&nodes_json)?.data;
    let mut subscriptions = std::collections::HashMap::new();

    for node in &nodes {
        let path = format!("/nodes/{}/subscription", node.node);
        let sub_json = exec_api(&client, &headers, host, &path)?;
        let sub: Subscription = serde_json::from_str::<ApiResponse<Subscription>>(&sub_json)?.data;

        subscriptions.insert(node.node.clone(), sub);
    }

    let cluster_json = exec_api(&client, &headers, host, "/cluster/status")?;
    let cluster: Vec<ClusterStatusEntry> = serde_json::from_str::<ApiResponse<Vec<ClusterStatusEntry>>>(&cluster_json)?.data;
    let mut cluster_name = None;
    let mut quorate = None;
    let mut enriched_nodes = Vec::new();

    for entry in cluster {
        match entry.r#type.as_str() {
            "cluster" => {
                cluster_name = entry.name;
                quorate = entry.quorate;
            }
            "node" => {
                let node_name = entry.name.clone().unwrap_or_default();
                let sub = subscriptions.get(&node_name);

                enriched_nodes.push(EnrichedClusterNode {
                    id: entry.id,
                    name: node_name.clone(),
                    ip: entry.ip,
                    nodeid: entry.nodeid,
                    online: entry.online,
                    subscription: Some(Subscription {
                        status: sub.and_then(|s| s.status.clone()),
                        level: sub.and_then(|s| s.level.clone()),
                        productname: sub.and_then(|s| s.productname.clone()),
                        key: sub.and_then(|s| s.key.clone()),
                        serverid: sub.and_then(|s| s.serverid.clone()),
                        sockets: sub.and_then(|s| s.sockets.clone()),
                        nextduedate: sub.and_then(|s| s.nextduedate.clone()),
                    }),
                });
            }
            _ => {}
        }
    }

    Ok(EnrichedCluster {
        cluster_name,
        quorate,
        nodes: enriched_nodes,
    })
}

fn exec_api(client: &Client, headers: &HeaderMap, host: &str, path: &str) -> Result<String, String> {
    debug!("→ Executing API GET {}", path);

    let url = format!("https://{}:8006/api2/json{}", host, path);
    let resp = client.get(&url).headers(headers.clone()).send().map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("API error: {}", resp.status()));
    }

    resp.text()
        .map_err(|e| format!("Failed to read response: {}", e))
}