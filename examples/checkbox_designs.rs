//! Example: the four dodai-ui design systems side by side — Cyberpunk, Material, Dodai and
//! Neo-brutalism — each with checked, unchecked, indeterminate, focused and disabled boxes.
//!
//! Keys: `↑/↓` or `k/j` move, `Space` toggles, `s` inline/block, `h` shadow, `r` readout,
//! `q` quits.
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use tui_checkbox::Checkbox;
use tui_checkbox::cyber::{Cyber, CyberSize, Design};

/// One row of the demo: a label and its state.
struct Item {
    label: &'static str,
    checked: bool,
    mixed: bool,
    disabled: bool,
}

struct App {
    items: Vec<Item>,
    cursor: usize,
    block: bool,
    shadow: bool,
    readout: bool,
}

impl App {
    fn new() -> Self {
        let item = |label, checked, mixed, disabled| Item {
            label,
            checked,
            mixed,
            disabled,
        };
        Self {
            items: vec![
                item("Notifications", true, false, false),
                item("Auto update", false, false, false),
                item("Share data", false, true, false),
                item("Beta channel", true, false, true),
            ],
            cursor: 0,
            block: false,
            shadow: true,
            readout: false,
        }
    }

    fn look(&self, design: Design) -> Cyber {
        Cyber::for_design(design)
            .size(if self.block {
                CyberSize::Block
            } else {
                CyberSize::Inline
            })
            .shadow(self.shadow)
            .readout(self.readout)
            .glow(true)
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|f| render(f, &app))?;
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down | KeyCode::Char('j') => {
                    app.cursor = (app.cursor + 1).min(app.items.len() - 1);
                }
                KeyCode::Up | KeyCode::Char('k') => app.cursor = app.cursor.saturating_sub(1),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    let item = &mut app.items[app.cursor];
                    if !item.disabled {
                        item.checked = item.mixed || !item.checked;
                        item.mixed = false;
                    }
                }
                KeyCode::Char('s') => app.block = !app.block,
                KeyCode::Char('h') => app.shadow = !app.shadow,
                KeyCode::Char('r') => app.readout = !app.readout,
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let [body, footer] = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);
    let rows = Layout::vertical([Constraint::Fill(1); 2]).split(body);
    for (i, design) in Design::ALL.into_iter().enumerate() {
        let cols = Layout::horizontal([Constraint::Fill(1); 2]).split(rows[i / 2]);
        render_design(frame, cols[i % 2], app, design);
    }
    let key = |k: &'static str| {
        Span::styled(
            format!(" {k} "),
            Style::new().add_modifier(Modifier::REVERSED),
        )
    };
    let flag = |on: bool| if on { "on" } else { "off" };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            key("↑↓"),
            Span::raw(" move  "),
            key("SPACE"),
            Span::raw(" toggle  "),
            key("S"),
            Span::raw(format!(" {}  ", if app.block { "block" } else { "inline" })),
            key("H"),
            Span::raw(format!(" shadow {}  ", flag(app.shadow))),
            key("R"),
            Span::raw(format!(" readout {}  ", flag(app.readout))),
            key("Q"),
            Span::raw(" quit"),
        ])),
        footer,
    );
}

fn render_design(frame: &mut Frame, area: Rect, app: &App, design: Design) {
    let pal = design.palette();
    let block = Block::bordered()
        .border_style(Style::new().fg(pal.muted))
        .title(format!(" {} ", design.name()))
        .style(Style::new().bg(pal.ink).fg(pal.text));
    let inner = block.inner(area).inner(Margin::new(2, 1));
    frame.render_widget(block, area);

    let mut y = inner.y;
    for (i, item) in app.items.iter().enumerate() {
        let cb = Checkbox::new(item.label, item.checked)
            .indeterminate(item.mixed)
            .focused(app.cursor == i)
            .disabled(item.disabled)
            .cyber(app.look(design));
        let h = cb.height();
        if y + h > inner.bottom() {
            break;
        }
        cb.render(Rect::new(inner.x, y, inner.width, h), frame.buffer_mut());
        y += h + 1;
    }
}
