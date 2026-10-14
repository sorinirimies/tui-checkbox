#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};
use tui_checkbox::Checkbox;

#[derive(Arbitrary, Debug)]
struct In {
    label: String,
    checked: bool,
    title: String,
    borders: u8,
    fg: u8,
    bg: u8,
    r: u8,
    mods: u16,
    pos: Pos,
    wrap: bool,
    size: Size,
}

fuzz_target!(|i: In| {
    let style = Style::default()
        .fg(Color::Indexed(i.fg))
        .bg(Color::Rgb(i.r, i.fg, i.bg))
        .add_modifier(Modifier::from_bits_truncate(i.mods));
    let borders = Borders::from_bits_truncate(i.borders);
    let cb = Checkbox::new(i.label.as_str(), i.checked)
        .block(Block::default().borders(borders).title(i.title.as_str()))
        .style(style)
        .checkbox_style(style)
        .label_style(style)
        .label_position(i.pos.get())
        .wrap_label(i.wrap);
    render(cb, i.size);
});
