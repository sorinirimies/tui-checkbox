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
    size: Size,
}

fuzz_target!(|i: In| {
    render(Checkbox::new(i.label.as_str(), i.checked), i.size);
});
