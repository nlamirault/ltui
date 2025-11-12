// SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crossterm::event::{KeyCode, KeyModifiers};
use ratatouille::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::client::{LinearClient, Team};
use crate::ui::{
    components::{HelpComponent, IssuesComponent, ProjectsComponent, TeamsComponent},
    events::{AppEvent, EventHandler},
};

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Issues,
    Projects,
    Teams,
}

pub struct AppState {
    pub current_view: View,
    pub current_team: Option<Team>,
    pub show_help: bool,
    pub issues_component: IssuesComponent,
    pub projects_component: ProjectsComponent,
    pub teams_component: TeamsComponent,
    pub loading: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: View::Issues,
            current_team: None,
            show_help: false,
            issues_component: IssuesComponent::new(),
            projects_component: ProjectsComponent::new(),
            teams_component: TeamsComponent::new(),
            loading: false,
        }
    }
}

pub struct TuiApp {
    state: AppState,
    client: LinearClient,
    event_handler: EventHandler,
}

impl TuiApp {
    pub fn new(client: LinearClient) -> Self {
        Self {
            state: AppState::new(),
            client,
            event_handler: EventHandler::new(),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::cursor::Hide
        )?;

        let backend = ratatouille::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatouille::Terminal::new(backend)?;

        self.event_handler.start();
        self.load_initial_data().await?;

        let result = self.run_app(&mut terminal).await;

        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_app(
        &mut self,
        terminal: &mut ratatouille::Terminal<
            ratatouille::backend::CrosstermBackend<std::io::Stdout>,
        >,
    ) -> anyhow::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if let Some(event) = self.event_handler.next().await {
                match event {
                    AppEvent::Key(key_event) => match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            break;
                        }
                        (KeyCode::Char('?'), _) => {
                            self.state.show_help = !self.state.show_help;
                        }
                        (KeyCode::Char('r'), _) => {
                            self.refresh_current_view().await?;
                        }
                        (KeyCode::Char('1'), _) => {
                            self.state.current_view = View::Issues;
                        }
                        (KeyCode::Char('2'), _) => {
                            self.state.current_view = View::Projects;
                        }
                        (KeyCode::Char('3'), _) => {
                            self.state.current_view = View::Teams;
                        }
                        (KeyCode::Tab, _) => {
                            self.next_view();
                        }
                        (KeyCode::BackTab, _) => {
                            self.previous_view();
                        }
                        _ => {
                            if !self.state.show_help {
                                self.handle_view_input(key_event.code).await?;
                            }
                        }
                    },
                    AppEvent::Refresh => {
                        self.refresh_current_view().await?;
                    }
                    AppEvent::Tick => {}
                    AppEvent::Quit => break,
                }
            }
        }

        Ok(())
    }

    async fn handle_view_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        match self.state.current_view {
            View::Issues => match key_code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.state.issues_component.select_next();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state.issues_component.select_previous();
                }
                KeyCode::Char('v') => {
                    if let Err(e) = self.state.issues_component.open_selected_issue() {
                        eprintln!("Failed to open issue in browser: {}", e);
                    }
                }
                KeyCode::Char('d') => {
                    self.state.issues_component.toggle_details();
                }
                _ => {}
            },
            View::Projects => match key_code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.state.projects_component.select_next();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state.projects_component.select_previous();
                }
                _ => {}
            },
            View::Teams => match key_code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.state.teams_component.select_next();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state.teams_component.select_previous();
                }
                KeyCode::Enter => {
                    if let Some(team) = self.state.teams_component.selected_team() {
                        self.state.current_team = Some(team.clone());
                        self.state.current_view = View::Issues;
                        self.load_team_data().await?;
                    }
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn next_view(&mut self) {
        self.state.current_view = match self.state.current_view {
            View::Issues => View::Projects,
            View::Projects => View::Teams,
            View::Teams => View::Issues,
        };
    }

    fn previous_view(&mut self) {
        self.state.current_view = match self.state.current_view {
            View::Issues => View::Teams,
            View::Projects => View::Issues,
            View::Teams => View::Projects,
        };
    }

    async fn load_initial_data(&mut self) -> anyhow::Result<()> {
        self.state.loading = true;

        let teams = self.client.get_teams().await?;
        self.state.teams_component.update_teams(teams);

        if let Some(team) = self.state.teams_component.teams.first() {
            self.state.current_team = Some(team.clone());
            self.load_team_data().await?;
        }

        self.state.loading = false;
        Ok(())
    }

    async fn load_team_data(&mut self) -> anyhow::Result<()> {
        if let Some(ref team) = self.state.current_team {
            let issues = self.client.get_issues(Some(&team.id), None).await?;
            self.state.issues_component.update_issues(issues.nodes);

            let projects = self.client.get_projects(Some(&team.id)).await?;
            self.state.projects_component.update_projects(projects);
        }
        Ok(())
    }

    async fn refresh_current_view(&mut self) -> anyhow::Result<()> {
        match self.state.current_view {
            View::Teams => {
                let teams = self.client.get_teams().await?;
                self.state.teams_component.update_teams(teams);
            }
            _ => {
                self.load_team_data().await?;
            }
        }
        Ok(())
    }

    fn render(&mut self, f: &mut Frame) {
        if self.state.show_help {
            HelpComponent::render(f, f.area());
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        // Header with tabs
        let tab_titles = vec!["Issues", "Projects", "Teams"];
        let selected_tab = match self.state.current_view {
            View::Issues => 0,
            View::Projects => 1,
            View::Teams => 2,
        };

        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title(" Linear TUI "))
            .select(selected_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow));

        f.render_widget(tabs, chunks[0]);

        // Main content area
        match self.state.current_view {
            View::Issues => {
                self.state.issues_component.render(f, chunks[1]);
            }
            View::Projects => {
                self.state.projects_component.render(f, chunks[1]);
            }
            View::Teams => {
                self.state.teams_component.render(f, chunks[1]);
            }
        }

        // Status bar
        let team_name = self
            .state
            .current_team
            .as_ref()
            .map(|t| format!(" Team: {} ", t.name))
            .unwrap_or_else(|| " No team selected ".to_string());

        let status_line = Line::from(vec![
            Span::styled(team_name, Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::styled("Press '?' for help", Style::default().fg(Color::Gray)),
            Span::raw(" | "),
            Span::styled("Press 'q' to quit", Style::default().fg(Color::Gray)),
        ]);

        let status_bar = ratatouille::widgets::Paragraph::new(status_line)
            .block(Block::default().borders(Borders::TOP));

        f.render_widget(status_bar, chunks[2]);
    }
}
