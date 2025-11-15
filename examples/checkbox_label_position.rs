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
use tui_checkbox::{Checkbox, LabelPosition};

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
    let header_text = "Checkbox Label Positions Demo - Press 'q' to quit";
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

    render_label_positions(frame, chunks[1]);
}

fn render_label_positions(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Label Positions")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(inner);

    // Label on Right (default)
    let checkbox_right = Checkbox::new("Right", true)
        .label_position(LabelPosition::Right)
        .checkbox_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_right, chunks[0]);

    // Label on Left
    let checkbox_left = Checkbox::new("Left", true)
        .label_position(LabelPosition::Left)
        .checkbox_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_left, chunks[1]);

    // Label on Top
    let checkbox_top = Checkbox::new("Top", false)
        .label_position(LabelPosition::Top)
        .checkbox_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_top, chunks[2]);

    // Label on Bottom
    let checkbox_bottom = Checkbox::new("Bottom", false)
        .label_position(LabelPosition::Bottom)
        .checkbox_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White));
    frame.render_widget(checkbox_bottom, chunks[3]);
}
