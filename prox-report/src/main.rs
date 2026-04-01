// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 credativ GmbH
// Author: Florian Paul Azim Hoberg @gyptazy <florian.hoberg@credativ.de>
// This file is part of prox-report.
//
// prox-report is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// prox-report is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.

mod audit;
mod cli;
mod logging;
mod utils;
mod validate;
use audit::exec_audit;
use validate::exec_validate;
use clap::Parser;
use cli::{Cli, Commands};
use utils::write_audit;
use log::{info, debug};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    logging::init(cli.debug)?;
    debug!("→ Starting {} v{} ({})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_HOMEPAGE"));

    if cli.debug {
        debug!("! Debug mode got enabled!");
    }

    match cli.command {
        Some(Commands::Validate) => {
            let result = exec_audit(&cli)?;
            exec_validate(&result)?;
        }

        Some(Commands::Audit) => {
            let result = exec_audit(&cli)?;
            write_audit(&result)?;
        }

        Some(Commands::Support) => {
            info!("Running support");
        }

        None => {
            debug!("No subcommand provided. Use --help for more information.");
            println!("No subcommand provided. Use --help for more information.");
        }
    }

    Ok(())

}