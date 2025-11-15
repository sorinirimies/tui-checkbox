use std::io;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tui_checkbox::{Checkbox, HorizontalAlignment, VerticalAlignment};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        terminal.draw(render)?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let area = frame.area();

    // Create header
    let header_text = "Checkbox Alignment Demo - Press 'q' to quit";
    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();

    // Split screen into header and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(area);

    frame.render_widget(header, chunks[0]);

    // Split content into two sections
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_horizontal_alignment(frame, content_chunks[0]);
    render_vertical_alignment(frame, content_chunks[1]);
}

fn render_horizontal_alignment(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Horizontal Alignment")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(inner);

    // Left alignment
    let checkbox_left = Checkbox::new("Left Aligned", true)
        .horizontal_alignment(HorizontalAlignment::Left)
        .checkbox_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_left, chunks[0]);

    // Center alignment
    let checkbox_center = Checkbox::new("Center Aligned", true)
        .horizontal_alignment(HorizontalAlignment::Center)
        .checkbox_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_center, chunks[1]);

    // Right alignment
    let checkbox_right = Checkbox::new("Right Aligned", true)
        .horizontal_alignment(HorizontalAlignment::Right)
        .checkbox_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_right, chunks[2]);
}

fn render_vertical_alignment(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Vertical Alignment")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(inner);

    // Top alignment
    let checkbox_top = Checkbox::new("Top Aligned", false)
        .vertical_alignment(VerticalAlignment::Top)
        .checkbox_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_top, rows[0]);

    // Center alignment
    let checkbox_center = Checkbox::new("Center Aligned", false)
        .vertical_alignment(VerticalAlignment::Center)
        .checkbox_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_center, rows[1]);

    // Bottom alignment
    let checkbox_bottom = Checkbox::new("Bottom Aligned", false)
        .vertical_alignment(VerticalAlignment::Bottom)
        .checkbox_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_bottom, rows[2]);
}
