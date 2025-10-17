// SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

use super::types::*;

#[derive(Clone)]
pub struct LinearClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl LinearClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.linear.app/graphql".to_string(),
        }
    }

    async fn execute_query<T>(&self, query: &str, variables: Option<serde_json::Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {

        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send GraphQL request")?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("HTTP error: {} - {}", status, error_text));
        }

        let response_text = response
            .text()
            .await
            .context("Failed to get response text")?;

        let graphql_response: GraphQLResponse<T> = serde_json::from_str(&response_text)
            .context("Failed to parse GraphQL response")?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            return Err(anyhow::anyhow!(
                "GraphQL errors: {}",
                error_messages.join(", ")
            ));
        }

        graphql_response
            .data
            .context("GraphQL response contained no data")
    }

    pub async fn get_viewer(&self) -> Result<User> {
        let query = r#"
            query {
                viewer {
                    id
                    name
                    email
                    displayName
                    avatarUrl
                }
            }
        "#;

        #[derive(serde::Deserialize)]
        struct ViewerResponse {
            viewer: User,
        }

        let response: ViewerResponse = self.execute_query(query, None).await?;
        Ok(response.viewer)
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        let query = r#"
            query {
                teams {
                    nodes {
                        id
                        name
                        key
                        description
                    }
                }
            }
        "#;

        #[derive(serde::Deserialize)]
        struct TeamsResponse {
            teams: TeamsConnection,
        }

        #[derive(serde::Deserialize)]
        struct TeamsConnection {
            nodes: Vec<Team>,
        }

        let response: TeamsResponse = self.execute_query(query, None).await?;
        Ok(response.teams.nodes)
    }

    pub async fn get_issues(
        &self,
        team_id: Option<&str>,
        first: Option<i32>,
    ) -> Result<IssuesConnection> {
        let (query, variables) = if let Some(team_id) = team_id {
            let query = r#"
                query($teamId: ID, $first: Int) {
                    issues(filter: { team: { id: { eq: $teamId } } }, first: $first, orderBy: updatedAt) {
                        nodes {
                            id
                            title
                            description
                            identifier
                            priority
                            url
                            createdAt
                            updatedAt
                            state {
                                id
                                name
                                color
                                type
                            }
                            assignee {
                                id
                                name
                                email
                                displayName
                                avatarUrl
                            }
                            creator {
                                id
                                name
                                email
                                displayName
                                avatarUrl
                            }
                            team {
                                id
                                name
                                key
                                description
                            }
                        }
                        pageInfo {
                            hasNextPage
                            hasPreviousPage
                            startCursor
                            endCursor
                        }
                    }
                }
            "#;

            let variables = json!({
                "teamId": team_id,
                "first": first.unwrap_or(50)
            });

            (query, Some(variables))
        } else {
            let query = r#"
                query($first: Int) {
                    issues(first: $first, orderBy: updatedAt) {
                        nodes {
                            id
                            title
                            description
                            identifier
                            priority
                            url
                            createdAt
                            updatedAt
                            state {
                                id
                                name
                                color
                                type
                            }
                            assignee {
                                id
                                name
                                email
                                displayName
                                avatarUrl
                            }
                            creator {
                                id
                                name
                                email
                                displayName
                                avatarUrl
                            }
                            team {
                                id
                                name
                                key
                                description
                            }
                        }
                        pageInfo {
                            hasNextPage
                            hasPreviousPage
                            startCursor
                            endCursor
                        }
                    }
                }
            "#;

            let variables = json!({
                "first": first.unwrap_or(50)
            });

            (query, Some(variables))
        };

        #[derive(serde::Deserialize)]
        struct IssuesResponse {
            issues: IssuesConnection,
        }

        let response: IssuesResponse = self.execute_query(query, variables).await?;
        Ok(response.issues)
    }

    pub async fn get_projects(&self, _team_id: Option<&str>) -> Result<Vec<Project>> {
        // For now, get all projects - team filtering can be added later
        let query = r#"
            query {
                projects {
                    nodes {
                        id
                        name
                        description
                        status {
                            name
                            color
                            type
                        }
                        lead {
                            id
                            name
                            email
                            displayName
                            avatarUrl
                        }
                    }
                }
            }
        "#;

        #[derive(serde::Deserialize)]
        struct ProjectsResponse {
            projects: ProjectsConnection,
        }

        #[derive(serde::Deserialize)]
        struct ProjectsConnection {
            nodes: Vec<Project>,
        }

        let response: ProjectsResponse = self.execute_query(query, None).await?;
        Ok(response.projects.nodes)
    }

    pub async fn create_issue(
        &self,
        team_id: &str,
        title: &str,
        description: Option<&str>,
    ) -> Result<Issue> {
        let query = r#"
            mutation($input: IssueCreateInput!) {
                issueCreate(input: $input) {
                    success
                    issue {
                        id
                        title
                        description
                        identifier
                        priority
                        url
                        createdAt
                        updatedAt
                        state {
                            id
                            name
                            color
                            type
                        }
                        assignee {
                            id
                            name
                            email
                            displayName
                            avatarUrl
                        }
                        creator {
                            id
                            name
                            email
                            displayName
                            avatarUrl
                        }
                        team {
                            id
                            name
                            key
                            description
                        }
                    }
                }
            }
        "#;

        let variables = json!({
            "input": {
                "teamId": team_id,
                "title": title,
                "description": description
            }
        });

        #[derive(serde::Deserialize)]
        struct CreateIssueResponse {
            #[serde(rename = "issueCreate")]
            issue_create: IssueCreatePayload,
        }

        #[derive(serde::Deserialize)]
        struct IssueCreatePayload {
            success: bool,
            issue: Option<Issue>,
        }

        let response: CreateIssueResponse = self.execute_query(query, Some(variables)).await?;

        if !response.issue_create.success {
            return Err(anyhow::anyhow!("Failed to create issue"));
        }

        response
            .issue_create
            .issue
            .context("Issue creation succeeded but no issue data returned")
    }
}
