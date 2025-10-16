use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub key: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub identifier: String,
    pub priority: Option<i32>,
    pub state: IssueState,
    pub assignee: Option<User>,
    pub creator: User,
    pub team: Team,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueState {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "type")]
    pub state_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub lead: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatus {
    pub name: String,
    pub color: String,
    #[serde(rename = "type")]
    pub status_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<Location>>,
    pub path: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssuesConnection {
    pub nodes: Vec<Issue>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}