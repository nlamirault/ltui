// SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::client::Team;

pub struct TeamsComponent {
    pub teams: Vec<Team>,
    pub state: ListState,
}

impl TeamsComponent {
    pub fn new() -> Self {
        Self {
            teams: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.teams.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.teams.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.teams.len().saturating_sub(1)
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };
        if !self.teams.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn selected_team(&self) -> Option<&Team> {
        if let Some(index) = self.state.selected() {
            self.teams.get(index)
        } else {
            None
        }
    }

    pub fn update_teams(&mut self, teams: Vec<Team>) {
        self.teams = teams;
        if !self.teams.is_empty() && self.state.selected().is_none() {
            self.state.select(Some(0));
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        // Create 4-section layout similar to g1c dashboard
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title bar
                Constraint::Length(5), // Overview panel
                Constraint::Min(8),    // Teams list
                Constraint::Length(3), // Status bar
            ])
            .split(area);

        // 1. Title Bar
        self.render_title_bar(f, main_chunks[0]);

        // 2. Overview Panel
        self.render_overview_panel(f, main_chunks[1]);

        // 3. Teams List
        self.render_teams_list(f, main_chunks[2]);

        // 4. Status Bar
        self.render_status_bar(f, main_chunks[3]);
    }

    fn render_title_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let title = Paragraph::new("🏢 Linear Teams")
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
        let total_teams = self.teams.len();
        let teams_with_descriptions = self
            .teams
            .iter()
            .filter(|t| t.description.is_some() && !t.description.as_ref().unwrap().is_empty())
            .count();
        let teams_without_descriptions = total_teams - teams_with_descriptions;

        let overview_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Total Teams: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", total_teams),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("With Descriptions: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", teams_with_descriptions),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(" | "),
                Span::styled("Without: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", teams_without_descriptions),
                    Style::default().fg(Color::Red),
                ),
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

    fn render_teams_list(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        if self.teams.is_empty() {
            let empty_msg = Paragraph::new("No teams found")
                .style(Style::default().fg(Color::Gray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Teams ")
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
        let key_width = 10; // Increased for better alignment
        let name_width = 50; // Increased from 25 to 35 for longer team names
        let desc_width = inner_width.saturating_sub(key_width + name_width + 6); // Account for separators

        // Render column headers
        self.render_teams_header(f, chunks[0], key_width, name_width, desc_width);

        let items: Vec<ListItem> = self
            .teams
            .iter()
            .enumerate()
            .map(|(i, team)| {
                let is_selected = Some(i) == self.state.selected();
                let selection_indicator = if is_selected { "➤ " } else { "  " };

                let description = team.description.as_deref().unwrap_or("No description");
                let truncated_name = self.truncate_text(&team.name, name_width as usize);
                let truncated_desc = self.truncate_text(description, desc_width as usize);

                let line = Line::from(vec![
                    Span::styled(selection_indicator, Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("{:>width$} ", team.key, width = (key_width - 1) as usize),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("│ "),
                    Span::styled(
                        format!("{:<width$} ", truncated_name, width = name_width as usize),
                        Style::default().fg(Color::White),
                    ),
                    Span::raw("│ "),
                    Span::styled(truncated_desc, Style::default().fg(Color::Gray)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Teams ({}) ", self.teams.len()))
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(list, chunks[1], &mut self.state);
    }

    fn render_teams_header(
        &self,
        f: &mut Frame,
        area: ratatui::layout::Rect,
        key_width: u16,
        name_width: u16,
        desc_width: u16,
    ) {
        let header = Line::from(vec![
            Span::raw("  "), // Space for selection indicator
            Span::styled(
                format!("{:>width$}", "KEY", width = (key_width - 1) as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:<width$}", "NAME", width = name_width as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:<width$}", "DESCRIPTION", width = desc_width as usize),
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
        let status_text = if let Some(team) = self.selected_team() {
            let description = team.description.as_deref().unwrap_or("No description");
            format!(
                "Selected: {} - {} | Press ? for help",
                team.name, description
            )
        } else {
            "No team selected | Press ? for help".to_string()
        };

        let status = Paragraph::new(status_text)
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
}
