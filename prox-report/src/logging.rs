// SPDX-License-Identifier: GPL-3.0-or-later

pub fn init(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = env_logger::Builder::new();

    if debug {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }

    builder.try_init()?;

    Ok(())
}