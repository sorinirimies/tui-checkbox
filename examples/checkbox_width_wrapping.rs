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
use tui_checkbox::Checkbox;

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
    let header_text = "Checkbox Width & Wrapping Demo - Press 'q' to quit";
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

    // Split content into sections
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[1]);

    render_width_constraints(frame, content_chunks[0]);
    render_text_wrapping(frame, content_chunks[1]);
    render_combined_features(frame, content_chunks[2]);
}

fn render_width_constraints(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Width Constraints")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(inner);

    // Min width example
    let checkbox_min = Checkbox::new("Short", true)
        .min_width(30)
        .checkbox_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("min_width(30)"),
        );
    frame.render_widget(checkbox_min, chunks[0]);

    // Max width example
    let checkbox_max = Checkbox::new(
        "This is a very long label that will be constrained by max width",
        false,
    )
    .max_width(30)
    .checkbox_style(
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    )
    .label_style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("max_width(30)"),
    );
    frame.render_widget(checkbox_max, chunks[1]);
}

fn render_text_wrapping(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Text Wrapping")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    // Without wrapping
    let checkbox_no_wrap = Checkbox::new(
        "This is a very long label that demonstrates what happens without text wrapping enabled",
        true,
    )
    .max_width(25)
    .wrap_label(false)
    .checkbox_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .label_style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("wrap_label(false)"),
    );
    frame.render_widget(checkbox_no_wrap, chunks[0]);

    // With wrapping
    let checkbox_wrap = Checkbox::new(
        "This is a very long label that demonstrates text wrapping functionality when enabled",
        false,
    )
    .max_width(25)
    .wrap_label(true)
    .checkbox_style(
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )
    .label_style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("wrap_label(true)"),
    );
    frame.render_widget(checkbox_wrap, chunks[1]);
}

fn render_combined_features(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Combined: Width Constraints + Wrapping")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    // Combined example 1
    let checkbox1 = Checkbox::new(
        "A moderately long label with both minimum and maximum width constraints",
        true,
    )
    .min_width(20)
    .max_width(35)
    .wrap_label(true)
    .checkbox_style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .label_style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("min(20) + max(35) + wrap"),
    );
    frame.render_widget(checkbox1, chunks[0]);

    // Combined example 2
    let checkbox2 = Checkbox::new(
        "Another example showing how text wrapping works beautifully with width constraints for responsive layouts",
        false,
    )
    .max_width(30)
    .wrap_label(true)
    .checkbox_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
    .label_style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("max(30) + wrap"),
    );
    frame.render_widget(checkbox2, chunks[1]);
}
