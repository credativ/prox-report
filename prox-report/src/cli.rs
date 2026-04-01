// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "A support and compliance tool for Proxmox nodes & clusters with credativ GmbH support.\nAuthor: Florian Paul Azim Hoberg @gyptazy <florian.hoberg@credativ.de>"
)]

pub struct Cli {
    #[arg(short = 'l', long, env = "PROX_REPORT_LOCAL", help = "Default: Run in local mode (on any PVE node)")]
    pub local: bool,

    #[arg(short = 'r', long, env = "PROX_REPORT_REMOTE", help = "Run in remote mode (requires API credentials)")]
    pub remote: bool,

    #[arg(long, env = "PROX_REPORT_RHOST")]
    pub rhost: Option<String>,

    #[arg(long, env = "PROX_REPORT_RTOKENID")]
    pub rtokenid: Option<String>,

    #[arg(long, env = "PROX_REPORT_RSECRET")]
    pub rsecret: Option<String>,

    #[arg(long, default_value_t = true, env = "PROX_REPORT_RINSECURE")]
    pub rinsecure: bool,

    #[arg(short = 'd', long, env = "PROX_REPORT_DEBUG", help = "Run prog-report in debug mode")]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Run validation on nodes or clusters")]
    Validate,

    #[command(about = "Run compliance audit checks")]
    Audit,

    #[command(about = "Collect support bundle and diagnostics")]
    Support,
}