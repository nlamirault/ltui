use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::client::Project;

pub struct ProjectsComponent {
    pub projects: Vec<Project>,
    pub state: ListState,
}

impl ProjectsComponent {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.projects.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.projects.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.projects.len().saturating_sub(1)
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };
        if !self.projects.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        if let Some(index) = self.state.selected() {
            self.projects.get(index)
        } else {
            None
        }
    }

    pub fn update_projects(&mut self, projects: Vec<Project>) {
        self.projects = projects;
        if !self.projects.is_empty() && self.state.selected().is_none() {
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
                Constraint::Min(8),    // Projects list
                Constraint::Length(3), // Status bar
            ])
            .split(area);

        // 1. Title Bar
        self.render_title_bar(f, main_chunks[0]);

        // 2. Overview Panel
        self.render_overview_panel(f, main_chunks[1]);

        // 3. Projects List
        self.render_projects_list(f, main_chunks[2]);

        // 4. Status Bar
        self.render_status_bar(f, main_chunks[3]);
    }

    fn render_title_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let title = Paragraph::new("🚀 Linear Projects")
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
        let total_projects = self.projects.len();

        // Count projects by status
        let completed = self
            .projects
            .iter()
            .filter(|p| p.status.status_type == "completed")
            .count();
        let started = self
            .projects
            .iter()
            .filter(|p| p.status.status_type == "started")
            .count();
        let planned = self
            .projects
            .iter()
            .filter(|p| p.status.status_type == "planned")
            .count();
        let paused = self
            .projects
            .iter()
            .filter(|p| p.status.status_type == "paused")
            .count();
        let canceled = self
            .projects
            .iter()
            .filter(|p| p.status.status_type == "canceled")
            .count();

        let with_leads = self.projects.iter().filter(|p| p.lead.is_some()).count();
        let without_leads = total_projects - with_leads;

        let overview_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Total Projects: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", total_projects),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  |  "),
                Span::styled("With Leads: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}", with_leads), Style::default().fg(Color::Green)),
                Span::raw(" | "),
                Span::styled("Without: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{}", without_leads),
                    Style::default().fg(Color::Red),
                ),
            ]),
            Line::from(vec![
                Span::styled("Status - ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("✓{} ", completed),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(format!("▶{} ", started), Style::default().fg(Color::Yellow)),
                Span::styled(format!("📋{} ", planned), Style::default().fg(Color::Blue)),
                Span::styled(format!("⏸{} ", paused), Style::default().fg(Color::Gray)),
                Span::styled(format!("✗{}", canceled), Style::default().fg(Color::Red)),
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

    fn render_projects_list(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        if self.projects.is_empty() {
            let empty_msg = Paragraph::new("No projects found")
                .style(Style::default().fg(Color::Gray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Projects ")
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
        let status_width = 12;
        let lead_width = 40; // Increased from 20 to 30 for longer names
        let name_width = inner_width.saturating_sub(status_width + lead_width + 6);

        // Render column headers
        self.render_projects_header(f, chunks[0], name_width, status_width, lead_width);

        let items: Vec<ListItem> = self
            .projects
            .iter()
            .enumerate()
            .map(|(i, project)| {
                let is_selected = Some(i) == self.state.selected();
                let selection_indicator = if is_selected { "➤ " } else { "  " };

                let state_color = match project.status.status_type.as_str() {
                    "completed" => Color::Green,
                    "started" => Color::Yellow,
                    "planned" => Color::Blue,
                    "paused" => Color::Gray,
                    "canceled" => Color::Red,
                    _ => Color::White,
                };

                let lead = project
                    .lead
                    .as_ref()
                    .map(|l| l.display_name.as_str())
                    .unwrap_or("No lead");

                let truncated_name = self.truncate_text(&project.name, name_width as usize);
                let truncated_lead = self.truncate_text(lead, lead_width as usize);

                let line = Line::from(vec![
                    Span::styled(selection_indicator, Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("{:<width$} ", truncated_name, width = name_width as usize),
                        Style::default().fg(Color::White),
                    ),
                    Span::raw("│ "),
                    Span::styled(
                        format!(
                            "{:>width$} ",
                            project.status.name,
                            width = (status_width - 1) as usize
                        ),
                        Style::default()
                            .fg(state_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("│ "),
                    Span::styled(truncated_lead, Style::default().fg(Color::Gray)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Projects ({}) ", self.projects.len()))
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(list, chunks[1], &mut self.state);
    }

    fn render_projects_header(
        &self,
        f: &mut Frame,
        area: ratatui::layout::Rect,
        name_width: u16,
        status_width: u16,
        lead_width: u16,
    ) {
        let header = Line::from(vec![
            Span::raw("  "), // Space for selection indicator
            Span::styled(
                format!("{:<width$}", "NAME", width = name_width as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:>width$}", "STATUS", width = (status_width - 1) as usize),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("{:<width$}", "LEAD", width = lead_width as usize),
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
        let status_text = if let Some(project) = self.selected_project() {
            let lead = project
                .lead
                .as_ref()
                .map(|l| l.display_name.as_str())
                .unwrap_or("No lead");
            format!(
                "Selected: {} - {} | Lead: {} | Press ? for help",
                project.name, project.status.name, lead
            )
        } else {
            "No project selected | Press ? for help".to_string()
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
}
