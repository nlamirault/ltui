// SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};

use crate::client::LinearClient;
use crate::config::Config;
use crate::ui::TuiApp;

pub struct App {
    client: LinearClient,
}

impl App {
    pub async fn new(config: Config, api_key: Option<String>) -> Result<Self> {
        let token = api_key
            .or_else(|| config.api_key.clone())
            .context("Linear Personal API Key is required. Set LINEAR_API_KEY environment variable or provide --token")?;

        let client = LinearClient::new(token);

        // Test the connection
        client.get_viewer().await.context(
            "Failed to authenticate with Linear API. Please check your Personal API Key.",
        )?;

        Ok(Self { client })
    }

    pub async fn run(&self) -> Result<()> {
        let mut tui_app = TuiApp::new(self.client.clone());
        tui_app.run().await
    }

    pub async fn test_mode(&self) -> Result<()> {
        println!("🧪 Testing Linear API connection...\n");

        println!("1. Testing authentication...");
        match self.client.get_viewer().await {
            Ok(user) => {
                println!("✅ Authentication successful!");
                println!("   User: {} ({})", user.display_name, user.name);
                if let Some(email) = &user.email {
                    println!("   Email: {}", email);
                }
            }
            Err(e) => {
                println!("❌ Authentication failed: {}", e);
                return Ok(());
            }
        }

        println!("\n2. Testing teams fetch...");
        match self.client.get_teams().await {
            Ok(teams) => {
                println!("✅ Teams fetch successful! Found {} teams:", teams.len());
                for team in teams.iter().take(3) {
                    println!("   - {} ({})", team.name, team.key);
                    if let Some(desc) = &team.description {
                        println!("     Description: {}", desc);
                    }
                }
            }
            Err(e) => {
                println!("❌ Teams fetch failed: {}", e);
                return Ok(());
            }
        }

        println!("\n3. Testing issues fetch...");
        match self.client.get_issues(None, Some(5)).await {
            Ok(issues) => {
                println!(
                    "✅ Issues fetch successful! Found {} issues:",
                    issues.nodes.len()
                );
                // for issue in issues.nodes.iter().take(3) {
                //     println!("   - {} ({})", issue.title, issue.identifier);
                //     println!("     State: {}", issue.state.name);
                //     if let Some(priority) = issue.priority {
                //         println!("     Priority: {}", priority);
                //     }
                //     if let Some(assignee) = &issue.assignee {
                //         println!("     Assignee: {}", assignee.display_name);
                //     }
                // }
            }
            Err(e) => {
                println!("❌ Issues fetch failed: {}", e);
                return Ok(());
            }
        }

        // println!("\n4. Testing projects fetch...");
        // match self.client.get_projects(None).await {
        //     Ok(projects) => {
        //         println!(
        //             "✅ Projects fetch successful! Found {} projects:",
        //             projects.len()
        //         );
        //         for project in projects.iter().take(3) {
        //             println!("   - {} ({})", project.name, project.state);
        //             if let Some(desc) = &project.description {
        //                 println!("     Description: {}", desc);
        //             }
        //             if let Some(lead) = &project.lead {
        //                 println!("     Lead: {}", lead.display_name);
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         println!("❌ Projects fetch failed: {}", e);
        //         return Ok(());
        //     }
        // }

        println!("\n🎉 All API tests completed!");
        Ok(())
    }
}
