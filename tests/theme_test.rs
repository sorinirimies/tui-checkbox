use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use tui_checkbox::{Checkbox, Theme, Themed};

fn draw(cb: Checkbox<'_>) -> Buffer {
    let area = Rect::new(0, 0, 20, 5);
    let mut buf = Buffer::empty(area);
    cb.render(area, &mut buf);
    buf
}

#[test]
fn every_theme_renders() {
    for t in Theme::all() {
        for checked in [true, false] {
            let _ = draw(
                Checkbox::new("Label", checked)
                    .block(Block::bordered())
                    .theme(&t),
            );
            let _ = draw(Checkbox::new("Label", checked).apply_theme(&t));
        }
    }
}

#[test]
fn theme_colors_and_precedence() {
    let t = Theme::dracula();
    let b = draw(Checkbox::new("L", true).theme(&t));
    assert_eq!(b[(0, 0)].fg, t.primary);
    let b = draw(
        Checkbox::new("L", true)
            .style(Style::default().fg(Color::Red))
            .theme(&t),
    );
    eprintln!(
        "{:?}",
        (0..4)
            .map(|x| (b[(x, 0)].symbol().to_string(), b[(x, 0)].fg))
            .collect::<Vec<_>>()
    );
    assert_eq!(b[(2, 0)].fg, Color::Red);
    // symbol theme color still applies over the explicit base style
    assert_eq!(b[(0, 0)].fg, t.primary);
}

#[test]
fn theme_by_value_and_default_unchanged() {
    let _ = draw(Checkbox::new("L", true).theme(Theme::nord()));
    assert_eq!(draw(Checkbox::new("L", true))[(0, 0)].fg, Color::Reset);
}
