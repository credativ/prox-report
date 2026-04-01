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
### Proxmox 8.x/9.x
For Proxmox VE as well as Debian- and Ubuntu-based systems such as Debian and Ubuntu, the recommended way to install prox-report is via the official APT repository. This allows you to receive updates automatically through your system’s package manager:

```
curl https://git.gyptazy.com/api/packages/gyptazy/debian/repository.key -o /etc/apt/keyrings/gyptazy.asc
echo "deb [signed-by=/etc/apt/keyrings/gyptazy.asc] https://packages.gyptazy.com/api/packages/gyptazy/debian trixie main" | sudo tee -a /etc/apt/sources.list.d/gyptazy.list
apt-get update
apt-get install -y prox-report
```

### Linux
The latest precompiled Linux binary of prox-report is always available on the official release page:
* https://github.com/credativ/prox-report/releases

To download the most recent version, use the asset named:
* `prox-report-linux-latest`

Alternatively, if you need a specific version (for reproducibility or compatibility reasons), you can download a versioned release such as:
* `prox-report-linux-v1.0.0`

Each release contains ready-to-use binaries, so no compilation is required. Simply download the appropriate file, make it executable if necessary, and run it on your Linux system.

> [!TIP]
> For Proxmox, Debian & Ubuntu systems use the repository!

It is recommended to use the -latest variant for the most up-to-date features and fixes, unless you explicitly require a pinned version.

### macOS
The latest precompiled macOS binary of prox-report is always available on the official release page:
* https://github.com/credativ/prox-report/releases

To download the most recent version, use the asset named:
* `prox-report-macos-latest`

Alternatively, if you need a specific version (for reproducibility or compatibility reasons), you can download a versioned release such as:
* `prox-report-macos-v1.0.0`

Each release contains ready-to-use binaries, so no compilation is required. Simply download the appropriate file, make it executable if necessary, and run it on your macOS system.

It is recommended to use the -latest variant for the most up-to-date features and fixes, unless you explicitly require a pinned version.

> [!TIP]
> Starting the App on macOS with Gatekeeper Security
When running prox-report on macOS for the first time, the system may prevent the application from starting and display a warning that it “cannot be opened because it is from an unidentified developer.” This behavior is part of Apple’s Gatekeeper security mechanism, which automatically applies a quarantine attribute to files downloaded from the internet.

There are two straightforward ways to resolve this. The first option is to use the graphical interface shown above. After attempting to open the binary once, macOS will block it and register the event. You can then open System Settings, navigate to Privacy & Security, and scroll down to the security section. There, a message will appear indicating that prox-report was blocked. By clicking “Allow Anyway” and confirming the subsequent prompt, you can explicitly authorize the application to run. When launching it again, macOS will present a final confirmation dialog where you can select “Open” to proceed.

<img align="left" src="https://cdn.gyptazy.com/img/prox-report-macos-security.jpg"/>
Alternatively, the quarantine attribute can be removed directly via the command line. This approach is particularly convenient for developers and automation workflows. Simply execute the following command in the directory containing the binary:

```
xattr -d com.apple.quarantine prox-report
```

### Windows
The latest precompiled Windows binary of prox-report is always available on the official release page:
* https://github.com/credativ/prox-report/releases

To download the most recent version, use the asset named:
* `prox-report-windows-latest`

Alternatively, if you need a specific version (for reproducibility or compatibility reasons), you can download a versioned release such as:
* `prox-report-windows-v1.0.0`

Each release contains ready-to-use binaries, so no compilation is required. Simply download the appropriate file, make it executable if necessary, and run it on your Windows system.

It is recommended to use the -latest variant for the most up-to-date features and fixes, unless you explicitly require a pinned version.

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

### Merge Licenses
The `merge-licenses` command of prox-report is designed to consolidate individual license reports from multiple Proxmox nodes into a single, normalized output. This is particularly useful for compliance checks and for providing structured data to credativ. This command expects a directory path (specified via -i) that contains license report files exported from the Proxmox web interface. Each node in the cluster must provide its own license report, and all files must be placed together in the given directory.

**Example:**
```bash
prox-report -d -i /PATH/TO/FILES/ merge-licenses
```

### Options
The available command-line options allow you to control how prox-report is executed and how it connects to a Proxmox VE environment. By default, the tool operates in local mode, meaning it assumes it is executed directly on a Proxmox node and can access all required information without additional parameters. When running in remote mode, the corresponding connection details must be provided, including the target host and API credentials. The API token ID and secret are used to authenticate against the Proxmox API, enabling secure access to cluster data from an external system. The --rinsecure flag can be used in environments where TLS certificates are not trusted, although this should generally be avoided in production setups. Instead of setting values on the cli, you may also use env vars. For troubleshooting and development purposes, the debug flag increases verbosity and provides deeper insight into the internal execution of the tool.

| Option | ENV Var | Description |
|--------|--------|------------|
| `-l, --local` | `PROX_REPORT_LOCAL` | Run in local mode (default, on any PVE node) |
| `-r, --remote` | `PROX_REPORT_REMOTE` | Run in remote mode (requires API credentials) |
| `--rhost <HOST>` | `PROX_REPORT_RHOST` | Remote Proxmox host |
| `--rtokenid <TOKEN_ID>` | `PROX_REPORT_RTOKENID` | API token ID |
| `--rsecret <SECRET>` | `PROX_REPORT_RSECRET` | API token secret |
| `--rinsecure` | `PROX_REPORT_RINSECURE` | Allow insecure TLS (default: false) |
| `-i, --input-path` | `PROX_REPORT_INPUT_PAT` | Input directory of already present license files (convert) |
| `-d, --debug` | `PROX_REPORT_DEBUG` | Enable debug mode |


### Commands
The command structure of prox-report is designed to cover the most common operational and compliance-related use cases for Proxmox VE environments.

* The validate command performs a series of checks against a node or cluster to ensure it meets baseline requirements and highlights potential misconfigurations.
* The audit command generates a structured audit file that can be used for compliance verification or shared with third parties such as support providers.
* The support command collects relevant diagnostic and system information into a bundled output, making it easier to analyze issues or provide the necessary data for troubleshooting and vendor support.

| Command | Description |
|--------|------------|
| `validate` | Validates the PVE node/cluster for basic compliance |
| `audit` | Creates an audit license file |
| ` merge-licenses` | Merge and convert license files which were obtained by the Proxmox UI |
| `support` | Creates an support file |

## Community & Support
Have questions, ideas, or need help with `prox-report`?
There are multiple ways to get support and connect with the community.

[Join the ProxTools Discord server](https://discord.gg/p9UxdMnx) for real-time discussions, help, and exchange with other users.

If you found a bug, want to request a feature, or suggest an improvement, please [create](https://github.com/credativ/prox-report/issues) an issue on GitHub. Your feedback is invaluable in making `prox-report` better for everyone!