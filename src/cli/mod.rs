// SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "ltui")]
#[command(about = "Terminal UI for Linear - A TUI client for managing Linear issues and projects")]
#[command(version)]
pub struct Cli {
    /// Linear Personal API Key (can also be set via LINEAR_API_KEY environment variable)
    #[arg(short, long, env = "LINEAR_API_KEY")]
    pub apikey: Option<String>,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    pub debug: bool,

    /// Test mode - don't start TUI, just test API calls
    #[arg(long)]
    pub test: bool,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        let config = Config::load(self.config.as_deref())?;

        let app = App::new(config, self.apikey.clone()).await?;

        if self.test {
            app.test_mode().await
        } else {
            app.run().await
        }
    }
}
