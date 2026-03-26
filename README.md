# Prox-Report - Support & compliance tool for Proxmox
<img align="left" src="https://cdn.gyptazy.com/img/credativ-prox-report.jpg"/>

<br clear="left">

<p float="center">
  <img src="https://img.shields.io/github/license/credativ/prox-report"/>
  <img src="https://img.shields.io/github/contributors/credativ/prox-report"/>
  <img src="https://img.shields.io/github/last-commit/credativ/prox-report/main"/>
  <img src="https://img.shields.io/github/issues-raw/credativ/prox-report"/>
  <img src="https://img.shields.io/github/issues-pr/credativ/prox-report"/>
</p>

# Table of Contents
- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Community & Support](#community--support)

## Overview
`prox-report` is a support and compliance tool for Proxmox VE nodes and clusters, designed to simplify diagnostics, validation, and auditing workflows in order to efficiently obtain support from <a href="https://credativ.de/">credativ GmbH</a>.

The tool is entirely written in Rust, ensuring high performance, reliability, and portability. It can be self-compiled from source or used via prebuilt binaries available for Linux, macOS, and Windows.
For seamless integration into Proxmox environments, prox-report can be installed directly on a Proxmox VE node using the provided Debian packages, allowing it to locally collect comprehensive system and cluster information.

Alternatively, it can be executed from any workstation in remote mode, where it securely retrieves all required data via the Proxmox API, making it suitable for centralized administration and support scenarios.

## Installation

## Build from Source
Building prox-report from source requires a properly configured development environment with the Rust toolchain installed. In particular, you need the Rust compiler (rustc) and the Cargo build system, which are typically installed together via the official Rust distribution. In addition, standard build tools for your operating system must be available, such as a C compiler and related system libraries, since some Rust dependencies rely on native components during compilation. Ensuring that your environment is up to date will help avoid build issues and guarantee a smooth compilation process.

```bash
git clone https://github.com/credativ/prox-report.git
cd prox-report
cargo build --release
```

The binary will be available at:
```bash
./target/release/prox-report
```

## Usage
The prox-report binary provides a flexible command-line interface that can be used both locally on a Proxmox VE node as well as remotely via the Proxmox API. Commands are structured in a straightforward way, allowing you to execute validation and reporting tasks with minimal configuration.

**Example:**
```bash
prox-report [OPTIONS] [COMMAND]
```

### Local Usage
When executed locally, prox-report directly accesses the underlying Proxmox VE node and gathers all required information without any additional configuration. This mode is particularly useful for administrators running the tool directly on a cluster node, as it leverages local system access and does not require API credentials.

**Example:**
```bash
prox-report validate
```

### Remote Usage
In remote mode, prox-report connects to a Proxmox VE cluster through its HTTPS API. This requires valid API credentials in the form of a token ID and secret. The --rhost parameter specifies the target API endpoint, while --rinsecure can be used to disable TLS certificate verification if necessary, for example in lab environments or when using self-signed certificates. Remote execution enables you to run the tool from a workstation or centralized management system, making it easier to audit multiple clusters without direct shell access to each node.

> [!TIP]
> For security reasons, only API tokens are accepted!

**Example:**
```bash
prox-report \
  --remote \
  --rhost pve01.example.com \
  --rtokenid 'root@pam!<TOKEN>' \
  --rsecret '<SECRET>' \
  validate
```

### Options
The available command-line options allow you to control how prox-report is executed and how it connects to a Proxmox VE environment. By default, the tool operates in local mode, meaning it assumes it is executed directly on a Proxmox node and can access all required information without additional parameters. When running in remote mode, the corresponding connection details must be provided, including the target host and API credentials. The API token ID and secret are used to authenticate against the Proxmox API, enabling secure access to cluster data from an external system. The --rinsecure flag can be used in environments where TLS certificates are not trusted, although this should generally be avoided in production setups. For troubleshooting and development purposes, the debug flag increases verbosity and provides deeper insight into the internal execution of the tool.

| Option | Description |
|--------|------------|
| `-l, --local` | Run in local mode (default, on any PVE node) |
| `-r, --remote` | Run in remote mode (requires API credentials) |
| `--rhost <HOST>` | Remote Proxmox host |
| `--rtokenid <TOKEN_ID>` | API token ID |
| `--rsecret <SECRET>` | API token secret |
| `--rinsecure` | Allow insecure TLS (default: false) |
| `-d, --debug` | Enable debug mode |


### Commands
The command structure of prox-report is designed to cover the most common operational and compliance-related use cases for Proxmox VE environments.

* The validate command performs a series of checks against a node or cluster to ensure it meets baseline requirements and highlights potential misconfigurations.
* The audit command generates a structured audit file that can be used for compliance verification or shared with third parties such as support providers.
* The support command collects relevant diagnostic and system information into a bundled output, making it easier to analyze issues or provide the necessary data for troubleshooting and vendor support.

| Command | Description |
|--------|------------|
| `validate` | Validates the PVE node/cluster for basic compliance |
| `audit` | Creates an audit license file |
| `support` | Creates an support file |

## Community & Support
Have questions, ideas, or need help with `prox-report`?
There are multiple ways to get support and connect with the community.

[Join the ProxTools Discord server](https://discord.gg/p9UxdMnx) for real-time discussions, help, and exchange with other users.

If you found a bug, want to request a feature, or suggest an improvement, please [create](https://github.com/credativ/prox-report/issues) an issue on GitHub. Your feedback is invaluable in making `prox-report` better for everyone!