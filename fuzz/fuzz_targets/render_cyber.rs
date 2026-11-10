#![no_main]
mod common;
use arbitrary::Arbitrary;
use common::*;
use libfuzzer_sys::fuzz_target;
use tui_checkbox::Checkbox;
use tui_checkbox::cyber::{Chamfer, Cyber, CyberPalette, CyberSize};

#[derive(Arbitrary, Debug)]
struct In {
    label: String,
    checked: bool,
    indeterminate: bool,
    focused: bool,
    disabled: bool,
    chamfer: u8,
    palette: u8,
    block: bool,
    shadow: bool,
    flat: bool,
    uppercase: bool,
    readout: bool,
    glow: bool,
    glitch: Option<u16>,
    decrypt: Option<u8>,
    pos: Pos,
    h: HAl,
    v: VAl,
    wrap: bool,
    min_w: Option<u16>,
    max_w: Option<u16>,
    size: Size,
}

fuzz_target!(|i: In| {
    let cyber = Cyber::new()
        .chamfer(Chamfer::ALL[usize::from(i.chamfer) % Chamfer::ALL.len()])
        .size(if i.block { CyberSize::Block } else { CyberSize::Inline })
        .shadow(i.shadow)
        .flat(i.flat)
        .uppercase(i.uppercase)
        .readout(i.readout)
        .glow(i.glow)
        .glitch(i.glitch)
        .decrypt(i.decrypt)
        .palette(CyberPalette::ALL[usize::from(i.palette) % CyberPalette::ALL.len()].1);
    let mut cb = Checkbox::new(i.label.as_str(), i.checked)
        .indeterminate(i.indeterminate)
        .focused(i.focused)
        .disabled(i.disabled)
        .label_position(i.pos.get())
        .horizontal_alignment(i.h.get())
        .vertical_alignment(i.v.get())
        .wrap_label(i.wrap)
        .cyber(cyber);
    if let Some(w) = i.min_w {
        cb = cb.min_width(w);
    }
    if let Some(w) = i.max_w {
        cb = cb.max_width(w);
    }
    let _ = (cb.width(), cb.height());
    render(cb, i.size);
});
