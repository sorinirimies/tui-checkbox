#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use tui_checkbox::Checkbox;

#[derive(Arbitrary, Debug)]
struct In {
    label: String,
    checked: bool,
    pos: Pos,
    h: HAl,
    v: VAl,
    min_w: u16,
    max_w: u16,
    wrap: bool,
    size: Size,
}

fuzz_target!(|i: In| {
    let cb = Checkbox::new(i.label.as_str(), i.checked)
        .label_position(i.pos.get())
        .horizontal_alignment(i.h.get())
        .vertical_alignment(i.v.get())
        .min_width(i.min_w)
        .max_width(i.max_w)
        .wrap_label(i.wrap);
    render(cb, i.size);
});
