use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HelpComponent;

impl HelpComponent {
    pub fn render(f: &mut Frame, area: ratatui::layout::Rect) {
        let help_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Navigation:", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Tab/Shift+Tab", Style::default().fg(Color::Cyan)),
                Span::raw("   Switch between views"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("j/k, ↓/↑", Style::default().fg(Color::Cyan)),
                Span::raw("      Navigate up/down"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Actions:", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("r", Style::default().fg(Color::Cyan)),
                Span::raw("             Refresh data"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Enter", Style::default().fg(Color::Cyan)),
                Span::raw("         Select/Open"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("v", Style::default().fg(Color::Cyan)),
                Span::raw("             Open issue in browser (in issues view)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("d", Style::default().fg(Color::Cyan)),
                Span::raw("             Toggle issue details view (in issues view)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("n", Style::default().fg(Color::Cyan)),
                Span::raw("             Create new issue (in issues view)"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Views:", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("1", Style::default().fg(Color::Cyan)),
                Span::raw("             Issues view"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("2", Style::default().fg(Color::Cyan)),
                Span::raw("             Projects view"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("3", Style::default().fg(Color::Cyan)),
                Span::raw("             Teams view"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Other:", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("?", Style::default().fg(Color::Cyan)),
                Span::raw("             Show/hide this help"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("q/Ctrl+C", Style::default().fg(Color::Cyan)),
                Span::raw("      Quit application"),
            ]),
        ]);

        let paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(" Help "));

        f.render_widget(paragraph, area);
    }
}