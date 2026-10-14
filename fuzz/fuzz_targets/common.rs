//! Shared helpers: bounded arbitrary inputs mapped onto tui-checkbox types.
#![allow(dead_code)]

use arbitrary::Arbitrary;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_checkbox::{HorizontalAlignment, LabelPosition, VerticalAlignment};

pub const MAX_W: u16 = 200;
pub const MAX_H: u16 = 100;

#[derive(Arbitrary, Debug, Clone, Copy)]
pub struct Size {
    pub w: u8,
    pub h: u8,
    pub x: u8,
    pub y: u8,
}

impl Size {
    /// Area <= 200x100, offset from origin by up to 7 cells.
    pub fn area(&self) -> Rect {
        Rect::new(
            self.x as u16 % 8,
            self.y as u16 % 8,
            self.w as u16 % (MAX_W + 1),
            self.h as u16 % (MAX_H + 1),
        )
    }
}

pub fn render<W: Widget>(w: W, size: Size) {
    let a = size.area();
    let full = Rect::new(0, 0, a.x + a.width, a.y + a.height);
    let mut buf = Buffer::empty(full);
    w.render(a, &mut buf);
}

#[derive(Arbitrary, Debug, Clone, Copy)]
pub enum Pos {
    Right,
    Left,
    Top,
    Bottom,
}
impl Pos {
    pub fn get(self) -> LabelPosition {
        match self {
            Pos::Right => LabelPosition::Right,
            Pos::Left => LabelPosition::Left,
            Pos::Top => LabelPosition::Top,
            Pos::Bottom => LabelPosition::Bottom,
        }
    }
}

#[derive(Arbitrary, Debug, Clone, Copy)]
pub enum HAl {
    Left,
    Center,
    Right,
}
impl HAl {
    pub fn get(self) -> HorizontalAlignment {
        match self {
            HAl::Left => HorizontalAlignment::Left,
            HAl::Center => HorizontalAlignment::Center,
            HAl::Right => HorizontalAlignment::Right,
        }
    }
}

#[derive(Arbitrary, Debug, Clone, Copy)]
pub enum VAl {
    Top,
    Center,
    Bottom,
}
impl VAl {
    pub fn get(self) -> VerticalAlignment {
        match self {
            VAl::Top => VerticalAlignment::Top,
            VAl::Center => VerticalAlignment::Center,
            VAl::Bottom => VerticalAlignment::Bottom,
        }
    }
}
