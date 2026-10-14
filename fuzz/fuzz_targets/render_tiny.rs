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
    w: u8,
    h_: u8,
}

// Very small areas (0..=5) where layout arithmetic is most likely to underflow.
fuzz_target!(|i: In| {
    let size = Size { w: i.w % 6, h: i.h_ % 6, x: i.w, y: i.h_ };
    let cb = Checkbox::new(i.label.as_str(), i.checked)
        .label_position(i.pos.get())
        .horizontal_alignment(i.h.get())
        .vertical_alignment(i.v.get())
        .min_width(i.min_w)
        .max_width(i.max_w)
        .wrap_label(i.wrap);
    render(cb, size);
});
