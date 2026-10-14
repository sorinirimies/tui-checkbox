#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use tui_checkbox::{Checkbox, Theme};

#[derive(Arbitrary, Debug)]
struct In {
    label: String,
    checked_sym: String,
    unchecked_sym: String,
    checked: bool,
    theme: u8,
    pos: Pos,
    wrap: bool,
    max_w: u16,
    size: Size,
}

fuzz_target!(|i: In| {
    let themes = Theme::all();
    let mut cb = Checkbox::new(i.label.as_str(), i.checked)
        .checked_symbol(i.checked_sym.as_str())
        .unchecked_symbol(i.unchecked_sym.as_str())
        .label_position(i.pos.get())
        .max_width(i.max_w)
        .wrap_label(i.wrap);
    if !themes.is_empty() {
        let t = themes[i.theme as usize % themes.len()].clone();
        cb = cb.theme(t);
    }
    render(cb, i.size);
});
