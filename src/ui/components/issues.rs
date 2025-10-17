// SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::client::Issue;

pub struct IssuesComponent {
    pub issues: Vec<Issue>,
    pub state: ListState,
    pub filter: String,
    pub show_details: bool,
}

impl IssuesComponent {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            state: ListState::default(),
            filter: String::new(),
            show_details: false,
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.issues.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.issues.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.issues.len().saturating_sub(1)
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };
        if !self.issues.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn selected_issue(&self) -> Option<&Issue> {
        if let Some(index) = self.state.selected() {
            self.issues.get(index)
        } else {
            None
        }
    }

    pub fn open_selected_issue(&self) -> anyhow::Result<()> {
        if let Some(issue) = self.selected_issue() {
            self.open_url(&issue.url)
        } else {
            Err(anyhow::anyhow!("No issue selected"))
        }
    }

    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    fn open_url(&self, url: &str) -> anyhow::Result<()> {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(url)
                .spawn()
                .map_err(|e| anyhow::anyhow!("Failed to open URL: {}", e))?;
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(url)
                .spawn()
                .map_err(|e| anyhow::anyhow!("Failed to open URL: {}", e))?;
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(&["/c", "start", url])
                .spawn()
                .map_err(|e| anyhow::anyhow!("Failed to open URL: {}", e))?;
        }
        Ok(())
    }

    pub fn update_issues(&mut self, issues: Vec<Issue>) {
        self.issues = issues;
        if !self.issues.is_empty() && self.state.selected().is_none() {
            self.state.select(Some(0));
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        if self.show_details {
            self.render_issue_details(f, area);
        } else {
            // Create 4-section layout similar to g1c dashboard
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title bar
                    Constraint::Length(6), // Overview panel
                    Constraint::Min(8),    // Issues list
                    Constraint::Length(3), // Status bar
                ])
                .split(area);

            // 1. Title Bar
            self.render_title_bar(f, main_chunks[0]);

            // 2. Overview Panel
            self.render_overview_panel(f, main_chunks[1]);

            // 3. Issues List
            self.render_issues_list(f, main_chunks[2]);

            // 4. Status Bar
            self.render_status_bar(f, main_chunks[3]);
        }
    }

    fn render_title_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let title_text = if !self.filter.is_empty() {
            format!("🎯 Linear Issues - Filter: '{}'", self.filter)
        } else {
            "🎯 Linear Issues".to_string()
        };

        let title = Paragraph::new(title_text)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            );
        f.render_widget(title, area);
    }

    fn render_overview_panel(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let total_issues = self.issues.len();

        // Count issues by state
        let completed = self
            .issues
            .iter()
            .filter(|i| i.state.state_type == "completed")
            .count();
        let started = self
            .issues
            .iter()
            .filter(|i| i.state.state_type == "started")
            .count();
        let unstarted = self
            .issues
            .iter()
            .filter(|i| i.state.state_type == "unstarted")
            .count();
        let canceled = self
            .issues
            .iter()
            .filter(|i| i.state.state_type == "canceled")
            .count();

        // Count by priority
        let urgent = self
            .issues
            .iter()
            .filter(|i| i.priority.unwrap_or(0) == 4)
            .count();
        let high = self
            .issues
            .iter()
            .filter(|i| i.priority.unwrap_or(0) == 3)
            .count();
        let medium = self
            .issues
            .iter()
            .filter(|i| i.priority.unwrap_or(0) == 2)
            .count();
        let low = self
            .issues
            .iter()
            .filter(|i| i.priority.unwrap_or(0) == 1)
            .count();

        let assigned = self.issues.iter().filter(|i| i.assignee.is_some()).count();
        let unassigned = total_issues - assigned;

        let overview_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Total Issues: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", total_issues),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  |  "),
                Span::styled("Assigned: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}", assigned), Style::default().fg(Color::Green)),
                Span::raw(" | "),
                Span::styled("Unassigned: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}", unassigned), Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("States - ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("✓{} ", completed),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(format!("▶{} ", started), Style::default().fg(Color::Yellow)),
                Span::styled(format!("○{} ", unstarted), Style::default().fg(Color::Gray)),
                Span::styled(format!("✗{}", canceled), Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("Priority - ", Style::default().fg(Color::Gray)),
                Span::styled(format!("🔥{} ", urgent), Style::default().fg(Color::Red)),
                Span::styled(format!("⚠{} ", high), Style::default().fg(Color::Yellow)),
                Span::styled(format!("●{} ", medium), Style::default().fg(Color::Blue)),
                Span::styled(format!("▪{}", low), Style::default().fg(Color::Gray)),
            ]),
        ]);

        let overview = Paragraph::new(overview_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Overview ")
                .border_style(Style::default().fg(Color::Gray)),
        );
        f.render_widget(overview, area);
    }

    fn render_issues_list(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        if self.issues.is_empty() {
            let empty_msg = Paragraph::new("No issues found")
                .style(Style::default().fg(Color::Gray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Issues ")
                        .border_style(Style::default().fg(Color::Gray)),
                );
            f.render_widget(empty_msg, area);
            return;
        }

        // Split area for header and list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5)])
            .split(area);

        // Calculate dynamic column widths based on available space
        let inner_width = area.width.saturating_sub(2); // Account for borders
        let id_width = 10; // Increased for better alignment
        let priority_width = 4; // For emoji symbols with padding
        let state_width = 14; // Increased for longer state names
        let assignee_width = 30; // Increased for longer names
        let title_width = inner_width
            .saturating_sub(id_width + priority_width + state_width + assignee_width + 12); // Account for 5 separators

        // Render column headers
        self.render_issues_header(
            f,
            chunks[0],
            id_width,
            priority_width,
            title_width,
            state_width,
            assignee_width,
        );

        let items: Vec<ListItem> = self
            .issues
            .iter()
            .enumerate()
            .map(|(i, issue)| {
                let is_selected = Some(i) == self.state.selected();
                let selection_indicator = if is_selected { "➤ " } else { "  " };

                let priority_color = match issue.priority.unwrap_or(0) {
                    4 => Color::Red,    // Urgent
                    3 => Color::Yellow, // High
                    2 => Color::Green,  // Medium
                    1 => Color::Blue,   // Low
                    _ => Color::White,
                };

                let priority_symbol = match issue.priority.unwrap_or(0) {
                    4 => "🔴",
                    3 => "🟠",
                    2 => "🟢",
                    1 => "🔵",
                    _ => "⚪",
                };

                let state_color = match issue.state.state_type.as_str() {
                    "completed" => Color::Green,
                    "started" => Color::Yellow,
                    "unstarted" => Color::Gray,
                    "canceled" => Color::Red,
                    _ => Color::White,
                };

                let assignee = issue
                    .assignee
                    .as_ref()
                    .map(|a| a.display_name.as_str())
                    .unwrap_or("Unassigned");

                let truncated_title = self.truncate_text(&issue.title, title_width as usize);
                let truncated_assignee = self.truncate_text(assignee, assignee_width as usize);

                let line = Line::from(vec![
                    Span::styled(selection_indicator, Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!(
                            "{:>width$} ",
                            issue.identifier,
                            width = (id_width - 1) as usize
                        ),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("│ "),
                    Span::styled(
                        format!(
                            "{:^width$} ",
                            priority_symbol,
                            width = (priority_width - 1) as usize
                        ),
                        Style::default().fg(priority_color),
                    ),
                    Span::raw("│ "),
                    Span::styled(
                        format!("{:<width$} ", truncated_title, width = title_width as usize),
                        Style::default().fg(Color::White),
                    ),
                    Span::raw("│ "),
                    Span::styled(
                        format!(
                            "{:>width$} ",
                            issue.state.name,
                            width = (state_width - 1) as usize
                        ),
                        Style::default()
                            .fg(state_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("│ "),
                    Span::styled(truncated_assignee, Style::default().fg(Color::Gray)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Issues ({}) ", self.issues.len()))
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(list, chunks[1], &mut self.state);
    }

    fn render_issues_header(
        &self,
        f: &mut Frame,
        area: ratatui::layout::Rect,
        id_width: u16,
        priority_width: u16,
        title_width: u16,
        state_width: u16,
        assignee_width: u16,
    ) {
        let header = Line::from(vec![
            Span::raw("  "), // Space for selection indicator
            Span::styled(
                format!("{:>width$}", "ID", width = (id_width - 1) as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:^width$}", "P", width = (priority_width - 1) as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:<width$}", "TITLE", width = title_width as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:>width$}", "STATE", width = (state_width - 1) as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:<width$}", "ASSIGNEE", width = assignee_width as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
        ]);

        let header_paragraph = Paragraph::new(header).block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::Gray)),
        );
        f.render_widget(header_paragraph, area);
    }

    fn render_status_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let status_text = if let Some(issue) = self.selected_issue() {
            let creator = &issue.creator.display_name;
            let team = &issue.team.key;
            format!(
                "Selected: {} - {} | Team: {} | Creator: {} | Press ? for help",
                issue.identifier, issue.title, team, creator
            )
        } else {
            "No issue selected | Press ? for help".to_string()
        };

        let status = Paragraph::new(
            self.truncate_text(&status_text, (area.width as usize).saturating_sub(4)),
        )
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Status ")
                .border_style(Style::default().fg(Color::Gray)),
        );
        f.render_widget(status, area);
    }

    fn truncate_text(&self, text: &str, max_width: usize) -> String {
        if text.len() <= max_width {
            text.to_string()
        } else if max_width > 3 {
            format!("{}...", &text[..max_width.saturating_sub(3)])
        } else {
            text.chars().take(max_width).collect()
        }
    }

    fn render_issue_details(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        if let Some(issue) = self.selected_issue() {
            // Create 3-section layout for details view
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),  // Header with issue info
                    Constraint::Min(8),     // Description/comments area
                    Constraint::Length(3),  // Status bar with navigation info
                ])
                .split(area);

            // 1. Issue Header
            self.render_issue_header(f, main_chunks[0], issue);

            // 2. Description and Comments
            self.render_issue_description(f, main_chunks[1], issue);

            // 3. Navigation Status Bar
            self.render_details_status_bar(f, main_chunks[2]);
        } else {
            let no_selection = Paragraph::new("No issue selected")
                .style(Style::default().fg(Color::Gray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Issue Details ")
                        .border_style(Style::default().fg(Color::Gray)),
                );
            f.render_widget(no_selection, area);
        }
    }

    fn render_issue_header(&self, f: &mut Frame, area: ratatui::layout::Rect, issue: &Issue) {
        let priority_color = match issue.priority.unwrap_or(0) {
            4 => Color::Red,
            3 => Color::Yellow,
            2 => Color::Green,
            1 => Color::Blue,
            _ => Color::White,
        };

        let priority_symbol = match issue.priority.unwrap_or(0) {
            4 => "🔴 URGENT",
            3 => "🟠 HIGH",
            2 => "🟢 MEDIUM",
            1 => "🔵 LOW",
            _ => "⚪ NO PRIORITY",
        };

        let state_color = match issue.state.state_type.as_str() {
            "completed" => Color::Green,
            "started" => Color::Yellow,
            "unstarted" => Color::Gray,
            "canceled" => Color::Red,
            _ => Color::White,
        };

        let assignee = issue
            .assignee
            .as_ref()
            .map(|a| a.display_name.as_str())
            .unwrap_or("Unassigned");

        let header_text = Text::from(vec![
            Line::from(vec![
                Span::styled(
                    format!("{} - ", issue.identifier),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    &issue.title,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::Gray)),
                Span::styled(priority_symbol, Style::default().fg(priority_color)),
                Span::raw("  |  "),
                Span::styled("State: ", Style::default().fg(Color::Gray)),
                Span::styled(&issue.state.name, Style::default().fg(state_color)),
            ]),
            Line::from(vec![
                Span::styled("Assignee: ", Style::default().fg(Color::Gray)),
                Span::styled(assignee, Style::default().fg(Color::White)),
                Span::raw("  |  "),
                Span::styled("Creator: ", Style::default().fg(Color::Gray)),
                Span::styled(&issue.creator.display_name, Style::default().fg(Color::White)),
            ]),
        ]);

        let header = Paragraph::new(header_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Issue Details ")
                .border_style(Style::default().fg(Color::Blue)),
        );
        f.render_widget(header, area);
    }

    fn render_issue_description(&self, f: &mut Frame, area: ratatui::layout::Rect, issue: &Issue) {
        let description = issue.description.as_deref().unwrap_or("No description available");
        
        // Split the description into lines that fit the available width
        let inner_width = area.width.saturating_sub(4) as usize; // Account for borders and padding
        let wrapped_lines: Vec<Line> = description
            .split('\n')
            .flat_map(|line| {
                if line.len() <= inner_width {
                    vec![Line::from(line.to_string())]
                } else {
                    // Simple word wrapping
                    let mut wrapped = Vec::new();
                    let words: Vec<&str> = line.split_whitespace().collect();
                    let mut current_line = String::new();
                    
                    for word in words {
                        if current_line.len() + word.len() + 1 <= inner_width {
                            if !current_line.is_empty() {
                                current_line.push(' ');
                            }
                            current_line.push_str(word);
                        } else {
                            if !current_line.is_empty() {
                                wrapped.push(Line::from(current_line.clone()));
                                current_line.clear();
                            }
                            current_line.push_str(word);
                        }
                    }
                    if !current_line.is_empty() {
                        wrapped.push(Line::from(current_line));
                    }
                    wrapped
                }
            })
            .collect();

        let description_text = Text::from(wrapped_lines);

        let description_widget = Paragraph::new(description_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Description ")
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(description_widget, area);
    }

    fn render_details_status_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let status_text = "Press 'd' to return to list view | Press 'v' to open in browser | Press ? for help";

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Navigation ")
                    .border_style(Style::default().fg(Color::Gray)),
            );
        f.render_widget(status, area);
    }
}
