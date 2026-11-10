//! Example: the cyberpunk look — chamfered slabs, readouts, glow, shadows, glitch and decrypt.
//!
//! Keys: `↑/↓` or `k/j` move, `Space` toggles (with a glitch + decrypt), `c` cycles the
//! chamfer, `p` the palette, `s` the size, `h` the shadow, `g` the glow, `f` flat, `q` quits.
use std::io;
use std::time::{Duration, Instant};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};
use tui_checkbox::Checkbox;
use tui_checkbox::cyber::{Chamfer, Cyber, CyberPalette, CyberSize};

/// Frames a toggle animates for.
const ANIM_FRAMES: u16 = 12;
/// Animation tick.
const TICK: Duration = Duration::from_millis(45);

struct Item {
    label: &'static str,
    checked: bool,
    disabled: bool,
    /// Frames since the last toggle, while animating.
    anim: Option<u16>,
}

struct App {
    items: Vec<Item>,
    /// 0 = "all systems", 1.. = items.
    cursor: usize,
    chamfer: usize,
    palette: usize,
    block: bool,
    shadow: bool,
    glow: bool,
    flat: bool,
    /// Animation of the "all systems" box.
    all_anim: Option<u16>,
}

impl App {
    fn new() -> Self {
        let item = |label, checked, disabled| Item {
            label,
            checked,
            disabled,
            anim: None,
        };
        Self {
            items: vec![
                item("Optical camo", true, false),
                item("Kerenzikov", false, false),
                item("Sandevistan", true, false),
                item("Mantis blades", false, false),
                item("Militech uplink", false, true),
            ],
            cursor: 1,
            chamfer: 0,
            palette: 0,
            block: false,
            shadow: false,
            glow: true,
            flat: false,
            all_anim: None,
        }
    }

    fn pal(&self) -> CyberPalette {
        CyberPalette::ALL[self.palette].1
    }

    fn cyber(&self, anim: Option<u16>) -> Cyber {
        let mut c = Cyber::new()
            .chamfer(Chamfer::ALL[self.chamfer])
            .size(if self.block {
                CyberSize::Block
            } else {
                CyberSize::Inline
            })
            .shadow(self.shadow)
            .glow(self.glow)
            .flat(self.flat)
            .readout(true)
            .palette(self.pal());
        if let Some(f) = anim {
            c = c
                .glitch((f < 5).then_some(f))
                .decrypt(Some((f * 100 / ANIM_FRAMES).min(100) as u8));
        }
        c
    }

    fn toggle(&mut self) {
        if self.cursor == 0 {
            let on = !self.items.iter().filter(|i| !i.disabled).all(|i| i.checked);
            for i in self.items.iter_mut().filter(|i| !i.disabled) {
                i.checked = on;
                i.anim = Some(0);
            }
            self.all_anim = Some(0);
        } else if let Some(i) = self.items.get_mut(self.cursor - 1)
            && !i.disabled
        {
            i.checked = !i.checked;
            i.anim = Some(0);
        }
    }

    fn tick(&mut self) {
        let step = |a: &mut Option<u16>| {
            *a = a.and_then(|f| (f + 1 < ANIM_FRAMES).then_some(f + 1));
        };
        step(&mut self.all_anim);
        for i in &mut self.items {
            step(&mut i.anim);
        }
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
    let mut last = Instant::now();
    loop {
        terminal.draw(|f| render(f, &app))?;
        let timeout = TICK.saturating_sub(last.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down | KeyCode::Char('j') => {
                    app.cursor = (app.cursor + 1).min(app.items.len());
                }
                KeyCode::Up | KeyCode::Char('k') => app.cursor = app.cursor.saturating_sub(1),
                KeyCode::Char(' ') | KeyCode::Enter => app.toggle(),
                KeyCode::Char('c') => app.chamfer = (app.chamfer + 1) % Chamfer::ALL.len(),
                KeyCode::Char('p') => app.palette = (app.palette + 1) % CyberPalette::ALL.len(),
                KeyCode::Char('s') => app.block = !app.block,
                KeyCode::Char('h') => app.shadow = !app.shadow,
                KeyCode::Char('g') => app.glow = !app.glow,
                KeyCode::Char('f') => app.flat = !app.flat,
                _ => {}
            }
        }
        if last.elapsed() >= TICK {
            app.tick();
            last = Instant::now();
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let pal = app.pal();
    let area = frame.area();
    Block::new()
        .style(Style::new().bg(pal.ink).fg(pal.text))
        .render(area, frame.buffer_mut());

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .areas(area);
    render_header(frame, header, app);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)]).areas(body);
    render_systems(frame, left, app);
    render_gallery(frame, right, app);
    render_footer(frame, footer, app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let pal = app.pal();
    let title = Line::from(vec![
        Span::styled(" ◢", Style::new().fg(pal.primary)),
        Span::styled(
            " NETRUNNER // CYBERWARE CONTROL ",
            Style::new()
                .fg(pal.ink)
                .bg(pal.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("◤", Style::new().fg(pal.primary)),
        Span::styled(
            format!(
                "  {} · {}",
                CyberPalette::ALL[app.palette].0,
                Chamfer::ALL[app.chamfer].name()
            ),
            Style::new().fg(pal.muted),
        ),
    ]);
    frame.render_widget(Paragraph::new(title), area);
}

fn panel<'a>(title: &'a str, pal: CyberPalette) -> Block<'a> {
    Block::bordered()
        .border_style(Style::new().fg(pal.muted))
        .title(Line::from(vec![
            Span::styled("▐", Style::new().fg(pal.secondary)),
            Span::styled(
                format!(" {title} "),
                Style::new()
                    .fg(pal.ink)
                    .bg(pal.secondary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("◤", Style::new().fg(pal.secondary)),
        ]))
}

fn render_systems(frame: &mut Frame, area: Rect, app: &App) {
    let pal = app.pal();
    let block = panel("SYSTEMS", pal);
    let inner = block.inner(area).inner(ratatui::layout::Margin::new(1, 1));
    frame.render_widget(block, area);

    let enabled: Vec<&Item> = app.items.iter().filter(|i| !i.disabled).collect();
    let on = enabled.iter().filter(|i| i.checked).count();
    let all = Checkbox::new("All systems", on == enabled.len())
        .indeterminate(on > 0 && on < enabled.len())
        .focused(app.cursor == 0)
        .cyber(app.cyber(app.all_anim));

    // One blank row between inline boxes; blocks are tall enough without one.
    let row_h = if app.block { all.height().max(4) } else { 2 };
    let mut y = inner.y;
    let mut draw = |cb: Checkbox<'_>| {
        if y < inner.bottom() {
            let h = cb.height().min(inner.bottom() - y);
            frame.render_widget(cb, Rect::new(inner.x, y, inner.width, h));
        }
        y = y.saturating_add(row_h);
    };
    draw(all);
    for (i, item) in app.items.iter().enumerate() {
        draw(
            Checkbox::new(item.label, item.checked)
                .focused(app.cursor == i + 1)
                .disabled(item.disabled)
                .cyber(app.cyber(item.anim)),
        );
    }
}

fn render_gallery(frame: &mut Frame, area: Rect, app: &App) {
    let pal = app.pal();
    let block = panel("CHAMFERS", pal);
    let inner = block.inner(area).inner(ratatui::layout::Margin::new(1, 1));
    frame.render_widget(block, area);

    // Two columns, three rows, checked in a chequerboard.
    let rows = Layout::vertical([Constraint::Length(5); 3]).split(inner);
    for (i, chamfer) in Chamfer::ALL.into_iter().enumerate() {
        let cols = Layout::horizontal([Constraint::Fill(1); 2]).split(rows[i / 2]);
        let cb = Checkbox::new(chamfer.name(), (i / 2 + i % 2) % 2 == 0).cyber(
            Cyber::new()
                .chamfer(chamfer)
                .size(CyberSize::Block)
                .shadow(true)
                .flat(app.flat)
                .palette(pal),
        );
        frame.render_widget(cb, cols[i % 2]);
    }
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let pal = app.pal();
    let key =
        |k: &'static str| Span::styled(format!(" {k} "), Style::new().fg(pal.ink).bg(pal.primary));
    let txt = |t: &'static str| Span::styled(format!(" {t}  "), Style::new().fg(pal.muted));
    let flag = |on: bool| if on { "on" } else { "off" };
    let keys = Line::from(vec![
        key("↑↓"),
        txt("move"),
        key("SPACE"),
        txt("toggle"),
        key("C"),
        txt("chamfer"),
        key("P"),
        txt("palette"),
        key("Q"),
        txt("quit"),
    ]);
    let options = Line::from(vec![
        key("S"),
        Span::styled(
            format!(" size {}  ", if app.block { "block" } else { "inline" }),
            Style::new().fg(pal.muted),
        ),
        key("H"),
        Span::styled(
            format!(" shadow {}  ", flag(app.shadow)),
            Style::new().fg(pal.muted),
        ),
        key("G"),
        Span::styled(
            format!(" glow {}  ", flag(app.glow)),
            Style::new().fg(pal.muted),
        ),
        key("F"),
        Span::styled(
            format!(" flat {}  ", flag(app.flat)),
            Style::new().fg(pal.muted),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(vec![keys, options]),
        area.inner(ratatui::layout::Margin::new(1, 0)),
    );
}
