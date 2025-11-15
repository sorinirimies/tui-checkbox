//! # tui-checkbox
//!
//! A customizable checkbox widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.
//!
//! ## Features
//!
//! - ☑️ Simple checkbox with label
//! - 🎨 Customizable styling for checkbox and label separately
//! - 🔤 Custom symbols (unicode, emoji, ASCII)
//! - 📦 Optional block wrapper
//! - ⚡ Zero-cost abstractions
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```no_run
//! use ratatui::style::{Color, Style};
//! use tui_checkbox::Checkbox;
//!
//! let checkbox = Checkbox::new("Enable feature", true);
//! ```
//!
//! With custom styling:
//!
//! ```no_run
//! use ratatui::style::{Color, Style, Modifier};
//! use tui_checkbox::Checkbox;
//!
//! let checkbox = Checkbox::new("Enable feature", true)
//!     .style(Style::default().fg(Color::White))
//!     .checkbox_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
//!     .label_style(Style::default().fg(Color::Gray));
//! ```
//!
//! With custom symbols:
//!
//! ```no_run
//! use tui_checkbox::Checkbox;
//!
//! let checkbox = Checkbox::new("Task", false)
//!     .checked_symbol("✅ ")
//!     .unchecked_symbol("⬜ ");
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)] // Terminal dimensions are always small

use std::borrow::Cow;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Styled};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Widget};

pub mod symbols;

/// Position of the label relative to the checkbox symbol.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LabelPosition {
    /// Label appears to the right of the checkbox (default)
    #[default]
    Right,
    /// Label appears to the left of the checkbox
    Left,
    /// Label appears above the checkbox
    Top,
    /// Label appears below the checkbox
    Bottom,
}

/// Horizontal alignment of content within its area.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum HorizontalAlignment {
    /// Align to the left (default)
    #[default]
    Left,
    /// Align to the center
    Center,
    /// Align to the right
    Right,
}

/// Vertical alignment of content within its area.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum VerticalAlignment {
    /// Align to the top (default)
    #[default]
    Top,
    /// Align to the center
    Center,
    /// Align to the bottom
    Bottom,
}

/// A widget that displays a checkbox with a label.
///
/// A `Checkbox` can be in a checked or unchecked state. The checkbox is rendered with a symbol
/// (default `☐` for unchecked and `☑` for checked) followed by a label.
///
/// The widget can be styled using [`Checkbox::style`] which affects both the checkbox symbol and
/// the label. You can also style just the checkbox symbol using [`Checkbox::checkbox_style`] or
/// the label using [`Checkbox::label_style`].
///
/// You can create a `Checkbox` using [`Checkbox::new`] or [`Checkbox::default`].
///
/// # Examples
///
/// ```
/// use ratatui::style::{Color, Style, Stylize};
/// use tui_checkbox::Checkbox;
///
/// Checkbox::new("Enable feature", true)
///     .style(Style::default().fg(Color::White))
///     .checkbox_style(Style::default().fg(Color::Green))
///     .label_style(Style::default().fg(Color::Gray));
/// ```
///
/// With a block:
/// ```
/// use ratatui::widgets::Block;
/// use tui_checkbox::Checkbox;
///
/// Checkbox::new("Accept terms", false).block(Block::bordered().title("Settings"));
/// ```
#[expect(clippy::struct_field_names)] // checkbox_style needs to be differentiated from style
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Checkbox<'a> {
    /// The label text displayed next to the checkbox
    label: Line<'a>,
    /// Whether the checkbox is checked
    checked: bool,
    /// Optional block to wrap the checkbox
    block: Option<Block<'a>>,
    /// Base style for the entire widget
    style: Style,
    /// Style specifically for the checkbox symbol
    checkbox_style: Style,
    /// Style specifically for the label text
    label_style: Style,
    /// Symbol to use when checked
    checked_symbol: Cow<'a, str>,
    /// Symbol to use when unchecked
    unchecked_symbol: Cow<'a, str>,
    /// Position of the label relative to the checkbox
    label_position: LabelPosition,
    /// Horizontal alignment of the checkbox symbol
    horizontal_alignment: HorizontalAlignment,
    /// Vertical alignment of the checkbox symbol
    vertical_alignment: VerticalAlignment,
    /// Minimum width constraint
    min_width: Option<u16>,
    /// Maximum width constraint
    max_width: Option<u16>,
    /// Whether to wrap label text to multiple lines
    wrap_label: bool,
}

impl Default for Checkbox<'_> {
    /// Returns a default `Checkbox` widget.
    ///
    /// The default widget has:
    /// - Empty label
    /// - Unchecked state
    /// - No block
    /// - Default style for all elements
    /// - Unicode checkbox symbols (☐ and ☑)
    /// - Label position on the right
    /// - Left and top alignment
    /// - No width constraints
    /// - No label wrapping
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::default();
    /// ```
    fn default() -> Self {
        Self {
            label: Line::default(),
            checked: false,
            block: None,
            style: Style::default(),
            checkbox_style: Style::default(),
            label_style: Style::default(),
            checked_symbol: Cow::Borrowed(symbols::CHECKED),
            unchecked_symbol: Cow::Borrowed(symbols::UNCHECKED),
            label_position: LabelPosition::default(),
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
            min_width: None,
            max_width: None,
            wrap_label: false,
        }
    }
}

impl<'a> Checkbox<'a> {
    /// Creates a new `Checkbox` with the given label and checked state.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Enable feature", true);
    /// ```
    ///
    /// With styled label:
    /// ```
    /// use ratatui::style::Stylize;
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Enable feature".blue(), false);
    /// ```
    pub fn new<T>(label: T, checked: bool) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            label: label.into(),
            checked,
            ..Default::default()
        }
    }

    /// Sets the label of the checkbox.
    ///
    /// The label can be any type that converts into a [`Line`], such as a string or a styled span.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::default().label("My checkbox");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.label = label.into();
        self
    }

    /// Sets the checked state of the checkbox.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::default().checked(true);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Wraps the checkbox with the given block.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Block;
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).block(Block::bordered().title("Settings"));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to both the checkbox symbol and the label unless overridden by
    /// more specific styles.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).style(Style::default().fg(Color::White));
    /// ```
    ///
    /// [`Color`]: ratatui::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the style of the checkbox symbol.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be combined with the base style set by [`Checkbox::style`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", true).checkbox_style(Style::default().fg(Color::Green));
    /// ```
    ///
    /// [`Color`]: ratatui::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn checkbox_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.checkbox_style = style.into();
        self
    }

    /// Sets the style of the label text.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be combined with the base style set by [`Checkbox::style`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).label_style(Style::default().fg(Color::Gray));
    /// ```
    ///
    /// [`Color`]: ratatui::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.label_style = style.into();
        self
    }

    /// Sets the symbol to use when the checkbox is checked.
    ///
    /// The default is `☑` (U+2611).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", true).checked_symbol("[X]");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn checked_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.checked_symbol = symbol.into();
        self
    }

    /// Sets the symbol to use when the checkbox is unchecked.
    ///
    /// The default is `☐` (U+2610).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).unchecked_symbol("[ ]");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn unchecked_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.unchecked_symbol = symbol.into();
        self
    }

    /// Sets the position of the label relative to the checkbox symbol.
    ///
    /// The default is [`LabelPosition::Right`].
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::{Checkbox, LabelPosition};
    ///
    /// let checkbox = Checkbox::new("Option", false).label_position(LabelPosition::Left);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Sets the horizontal alignment of the checkbox content within its area.
    ///
    /// The default is [`HorizontalAlignment::Left`].
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::{Checkbox, HorizontalAlignment};
    ///
    /// let checkbox = Checkbox::new("Option", false)
    ///     .horizontal_alignment(HorizontalAlignment::Center);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the checkbox content within its area.
    ///
    /// The default is [`VerticalAlignment::Top`].
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::{Checkbox, VerticalAlignment};
    ///
    /// let checkbox = Checkbox::new("Option", false)
    ///     .vertical_alignment(VerticalAlignment::Center);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the minimum width constraint for the checkbox widget.
    ///
    /// The default is no minimum width.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).min_width(20);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn min_width(mut self, width: u16) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Sets the maximum width constraint for the checkbox widget.
    ///
    /// The default is no maximum width.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).max_width(40);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn max_width(mut self, width: u16) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Enables or disables label text wrapping.
    ///
    /// When enabled, the label will wrap to multiple lines if it exceeds the available width.
    /// The default is `false` (no wrapping).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("This is a very long label that should wrap", false)
    ///     .wrap_label(true)
    ///     .max_width(30);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn wrap_label(mut self, wrap: bool) -> Self {
        self.wrap_label = wrap;
        self
    }
}

impl Styled for Checkbox<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self::Item {
        self.style = style.into();
        self
    }
}

impl Widget for Checkbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Checkbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let inner = if let Some(ref block) = self.block {
            let inner_area = block.inner(area);
            block.render(area, buf);
            inner_area
        } else {
            area
        };
        self.render_checkbox(inner, buf);
    }
}

impl Checkbox<'_> {
    fn render_checkbox(&self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        // Determine which symbol to use based on checked state
        let symbol = if self.checked {
            &self.checked_symbol
        } else {
            &self.unchecked_symbol
        };

        // Calculate the combined styles
        let checkbox_style = self.style.patch(self.checkbox_style);
        let label_style = self.style.patch(self.label_style);

        // Apply width constraints
        let mut render_area = area;
        if let Some(min_width) = self.min_width {
            render_area.width = render_area.width.max(min_width);
        }
        if let Some(max_width) = self.max_width {
            render_area.width = render_area.width.min(max_width);
        }

        // Ensure render_area doesn't exceed original area
        render_area.width = render_area.width.min(area.width);

        // Create checkbox and label spans
        let checkbox_span = Span::styled(symbol.as_ref(), checkbox_style);
        let styled_label = self.label.clone().patch_style(label_style);
        let owned_label = Line::from(
            styled_label
                .spans
                .iter()
                .map(|s| Span::styled(s.content.to_string(), s.style))
                .collect::<Vec<_>>(),
        );

        // Calculate dimensions based on label position
        match self.label_position {
            LabelPosition::Right | LabelPosition::Left => {
                self.render_horizontal(render_area, buf, checkbox_span, owned_label);
            }
            LabelPosition::Top | LabelPosition::Bottom => {
                self.render_vertical(render_area, buf, checkbox_span, owned_label);
            }
        }
    }

    fn render_horizontal(
        &self,
        area: Rect,
        buf: &mut Buffer,
        checkbox_span: Span<'_>,
        label: Line<'static>,
    ) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let checkbox_width = checkbox_span.width() as u16;
        let space_width = 1u16;

        // Handle wrapping if enabled
        let label_lines = if self.wrap_label {
            let available_width = area.width.saturating_sub(checkbox_width + space_width);
            Self::wrap_text(&label, available_width)
        } else {
            vec![label]
        };

        let total_width = if label_lines.is_empty() {
            checkbox_width
        } else {
            checkbox_width
                + space_width
                + label_lines
                    .iter()
                    .map(|l| l.width() as u16)
                    .max()
                    .unwrap_or(0)
        };

        // Calculate horizontal offset based on alignment
        let x_offset = match self.horizontal_alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => area.width.saturating_sub(total_width) / 2,
            HorizontalAlignment::Right => area.width.saturating_sub(total_width),
        };

        // Calculate vertical offset based on alignment
        let content_height = label_lines.len() as u16;
        let y_offset = match self.vertical_alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => area.height.saturating_sub(content_height) / 2,
            VerticalAlignment::Bottom => area.height.saturating_sub(content_height),
        };

        // Render based on label position
        match self.label_position {
            LabelPosition::Right => {
                // Render checkbox first, then label
                if x_offset < area.width && y_offset < area.height {
                    let checkbox_area = Rect {
                        x: area.x + x_offset,
                        y: area.y + y_offset,
                        width: checkbox_width.min(area.width.saturating_sub(x_offset)),
                        height: 1,
                    };
                    Line::from(vec![checkbox_span]).render(checkbox_area, buf);

                    // Render label lines
                    for (i, label_line) in label_lines.iter().enumerate() {
                        let label_x = area.x + x_offset + checkbox_width + space_width;
                        let label_y = area.y + y_offset + i as u16;
                        if label_y < area.y + area.height && label_x < area.x + area.width {
                            let label_area = Rect {
                                x: label_x,
                                y: label_y,
                                width: area
                                    .width
                                    .saturating_sub(x_offset + checkbox_width + space_width),
                                height: 1,
                            };
                            label_line.clone().render(label_area, buf);
                        }
                    }
                }
            }
            LabelPosition::Left => {
                // Render label first, then checkbox
                let max_label_width = label_lines
                    .iter()
                    .map(|l| l.width() as u16)
                    .max()
                    .unwrap_or(0);

                // Render label lines
                for (i, label_line) in label_lines.iter().enumerate() {
                    let label_y = area.y + y_offset + i as u16;
                    if label_y < area.y + area.height && x_offset < area.width {
                        let label_area = Rect {
                            x: area.x + x_offset,
                            y: label_y,
                            width: max_label_width.min(area.width.saturating_sub(x_offset)),
                            height: 1,
                        };
                        label_line.clone().render(label_area, buf);
                    }
                }

                // Render checkbox
                let checkbox_x = area.x + x_offset + max_label_width + space_width;
                if checkbox_x < area.x + area.width && y_offset < area.height {
                    let checkbox_area = Rect {
                        x: checkbox_x,
                        y: area.y + y_offset,
                        width: checkbox_width.min(
                            area.width
                                .saturating_sub(x_offset + max_label_width + space_width),
                        ),
                        height: 1,
                    };
                    Line::from(vec![checkbox_span]).render(checkbox_area, buf);
                }
            }
            _ => {}
        }
    }

    fn render_vertical(
        &self,
        area: Rect,
        buf: &mut Buffer,
        checkbox_span: Span<'_>,
        label: Line<'static>,
    ) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        // Handle wrapping if enabled
        let label_lines = if self.wrap_label {
            Self::wrap_text(&label, area.width)
        } else {
            vec![label]
        };

        let checkbox_width = checkbox_span.width() as u16;
        let label_height = label_lines.len() as u16;
        let total_height = 1 + label_height; // checkbox + label lines

        // Calculate vertical offset
        let y_offset = match self.vertical_alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => area.height.saturating_sub(total_height) / 2,
            VerticalAlignment::Bottom => area.height.saturating_sub(total_height),
        };

        match self.label_position {
            LabelPosition::Top => {
                // Render label first
                for (i, label_line) in label_lines.iter().enumerate() {
                    let label_y = area.y + y_offset + i as u16;
                    if label_y < area.y + area.height {
                        let x_offset = match self.horizontal_alignment {
                            HorizontalAlignment::Left => 0,
                            HorizontalAlignment::Center => {
                                area.width.saturating_sub(label_line.width() as u16) / 2
                            }
                            HorizontalAlignment::Right => {
                                area.width.saturating_sub(label_line.width() as u16)
                            }
                        };
                        let label_area = Rect {
                            x: area.x + x_offset,
                            y: label_y,
                            width: area.width.saturating_sub(x_offset),
                            height: 1,
                        };
                        label_line.clone().render(label_area, buf);
                    }
                }

                // Render checkbox
                let checkbox_y = area.y + y_offset + label_height;
                if checkbox_y < area.y + area.height {
                    let x_offset = match self.horizontal_alignment {
                        HorizontalAlignment::Left => 0,
                        HorizontalAlignment::Center => {
                            area.width.saturating_sub(checkbox_width) / 2
                        }
                        HorizontalAlignment::Right => area.width.saturating_sub(checkbox_width),
                    };
                    let checkbox_area = Rect {
                        x: area.x + x_offset,
                        y: checkbox_y,
                        width: checkbox_width.min(area.width.saturating_sub(x_offset)),
                        height: 1,
                    };
                    Line::from(vec![checkbox_span]).render(checkbox_area, buf);
                }
            }
            LabelPosition::Bottom => {
                // Render checkbox first
                let x_offset = match self.horizontal_alignment {
                    HorizontalAlignment::Left => 0,
                    HorizontalAlignment::Center => area.width.saturating_sub(checkbox_width) / 2,
                    HorizontalAlignment::Right => area.width.saturating_sub(checkbox_width),
                };
                let checkbox_area = Rect {
                    x: area.x + x_offset,
                    y: area.y + y_offset,
                    width: checkbox_width.min(area.width.saturating_sub(x_offset)),
                    height: 1,
                };
                Line::from(vec![checkbox_span]).render(checkbox_area, buf);

                // Render label
                for (i, label_line) in label_lines.iter().enumerate() {
                    let label_y = area.y + y_offset + 1 + i as u16;
                    if label_y < area.y + area.height {
                        let x_offset = match self.horizontal_alignment {
                            HorizontalAlignment::Left => 0,
                            HorizontalAlignment::Center => {
                                area.width.saturating_sub(label_line.width() as u16) / 2
                            }
                            HorizontalAlignment::Right => {
                                area.width.saturating_sub(label_line.width() as u16)
                            }
                        };
                        let label_area = Rect {
                            x: area.x + x_offset,
                            y: label_y,
                            width: area.width.saturating_sub(x_offset),
                            height: 1,
                        };
                        label_line.clone().render(label_area, buf);
                    }
                }
            }
            _ => {}
        }
    }

    fn wrap_text(line: &Line<'_>, max_width: u16) -> Vec<Line<'static>> {
        if max_width == 0 {
            let owned = Line::from(
                line.spans
                    .iter()
                    .map(|s| Span::styled(s.content.to_string(), s.style))
                    .collect::<Vec<_>>(),
            );
            return vec![owned];
        }

        let mut result = Vec::new();
        let mut current_line = Vec::new();
        let mut current_width = 0u16;

        for span in &line.spans {
            let text = span.content.as_ref();
            let words: Vec<&str> = text.split(' ').collect();

            for (i, word) in words.iter().enumerate() {
                let word_width = word.chars().count() as u16;
                let space_width = u16::from(i > 0 || !current_line.is_empty());

                if current_width + space_width + word_width > max_width && !current_line.is_empty()
                {
                    result.push(Line::from(current_line.clone()));
                    current_line.clear();
                    current_width = 0;
                }

                if i > 0 {
                    current_line.push(Span::styled(String::from(" "), span.style));
                    current_width += 1;
                }

                current_line.push(Span::styled(String::from(*word), span.style));
                current_width += word_width;
            }
        }

        if !current_line.is_empty() {
            result.push(Line::from(current_line));
        }

        if result.is_empty() {
            let owned = Line::from(
                line.spans
                    .iter()
                    .map(|s| Span::styled(s.content.to_string(), s.style))
                    .collect::<Vec<_>>(),
            );
            result.push(owned);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier, Stylize};

    use super::*;

    #[test]
    fn checkbox_new() {
        let checkbox = Checkbox::new("Test", true);
        assert_eq!(checkbox.label, Line::from("Test"));
        assert!(checkbox.checked);
    }

    #[test]
    fn checkbox_default() {
        let checkbox = Checkbox::default();
        assert_eq!(checkbox.label, Line::default());
        assert!(!checkbox.checked);
    }

    #[test]
    fn checkbox_label() {
        let checkbox = Checkbox::default().label("New label");
        assert_eq!(checkbox.label, Line::from("New label"));
    }

    #[test]
    fn checkbox_checked() {
        let checkbox = Checkbox::default().checked(true);
        assert!(checkbox.checked);
    }

    #[test]
    fn checkbox_style() {
        let style = Style::default().fg(Color::Red);
        let checkbox = Checkbox::default().style(style);
        assert_eq!(checkbox.style, style);
    }

    #[test]
    fn checkbox_checkbox_style() {
        let style = Style::default().fg(Color::Green);
        let checkbox = Checkbox::default().checkbox_style(style);
        assert_eq!(checkbox.checkbox_style, style);
    }

    #[test]
    fn checkbox_label_style() {
        let style = Style::default().fg(Color::Blue);
        let checkbox = Checkbox::default().label_style(style);
        assert_eq!(checkbox.label_style, style);
    }

    #[test]
    fn checkbox_checked_symbol() {
        let checkbox = Checkbox::default().checked_symbol("[X]");
        assert_eq!(checkbox.checked_symbol, "[X]");
    }

    #[test]
    fn checkbox_unchecked_symbol() {
        let checkbox = Checkbox::default().unchecked_symbol("[ ]");
        assert_eq!(checkbox.unchecked_symbol, "[ ]");
    }

    #[test]
    fn checkbox_styled_trait() {
        let checkbox = Checkbox::default().red();
        assert_eq!(checkbox.style, Style::default().fg(Color::Red));
    }

    #[test]
    fn checkbox_render_unchecked() {
        let checkbox = Checkbox::new("Test", false);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        // The buffer should contain the unchecked symbol followed by space and label
        assert!(buffer
            .cell(buffer.area.as_position())
            .unwrap()
            .symbol()
            .starts_with('☐'));
    }

    #[test]
    fn checkbox_render_checked() {
        let checkbox = Checkbox::new("Test", true);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        // The buffer should contain the checked symbol followed by space and label
        assert!(buffer
            .cell(buffer.area.as_position())
            .unwrap()
            .symbol()
            .starts_with('☑'));
    }

    #[test]
    fn checkbox_render_empty_area() {
        let checkbox = Checkbox::new("Test", true);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 0));

        // Should not panic
        checkbox.render(buffer.area, &mut buffer);
    }

    #[test]
    fn checkbox_render_with_block() {
        let checkbox = Checkbox::new("Test", true).block(Block::bordered());
        let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 3));

        // Should not panic
        checkbox.render(buffer.area, &mut buffer);
    }

    #[test]
    fn checkbox_render_with_custom_symbols() {
        let checkbox = Checkbox::new("Test", true)
            .checked_symbol("[X]")
            .unchecked_symbol("[ ]");

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        assert!(buffer
            .cell(buffer.area.as_position())
            .unwrap()
            .symbol()
            .starts_with('['));
    }

    #[test]
    fn checkbox_with_styled_label() {
        let checkbox = Checkbox::new("Test".blue(), true);
        assert_eq!(checkbox.label.spans[0].style.fg, Some(Color::Blue));
    }

    #[test]
    fn checkbox_complex_styling() {
        let checkbox = Checkbox::new("Feature", true)
            .style(Style::default().fg(Color::White))
            .checkbox_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(Color::Gray));

        assert_eq!(checkbox.style.fg, Some(Color::White));
        assert_eq!(checkbox.checkbox_style.fg, Some(Color::Green));
        assert_eq!(checkbox.label_style.fg, Some(Color::Gray));
    }

    #[test]
    fn checkbox_emoji_symbols() {
        let checkbox = Checkbox::new("Test", true)
            .checked_symbol("✅ ")
            .unchecked_symbol("⬜ ");

        assert_eq!(checkbox.checked_symbol, "✅ ");
        assert_eq!(checkbox.unchecked_symbol, "⬜ ");
    }

    #[test]
    fn checkbox_unicode_symbols() {
        let checkbox = Checkbox::new("Test", false)
            .checked_symbol("● ")
            .unchecked_symbol("○ ");

        assert_eq!(checkbox.checked_symbol, "● ");
        assert_eq!(checkbox.unchecked_symbol, "○ ");
    }

    #[test]
    fn checkbox_arrow_symbols() {
        let checkbox = Checkbox::new("Test", true)
            .checked_symbol("▶ ")
            .unchecked_symbol("▷ ");

        assert_eq!(checkbox.checked_symbol, "▶ ");
        assert_eq!(checkbox.unchecked_symbol, "▷ ");
    }

    #[test]
    fn checkbox_parenthesis_symbols() {
        let checkbox = Checkbox::new("Test", false)
            .checked_symbol("(X)")
            .unchecked_symbol("(O)");

        assert_eq!(checkbox.checked_symbol, "(X)");
        assert_eq!(checkbox.unchecked_symbol, "(O)");
    }

    #[test]
    fn checkbox_minus_symbols() {
        let checkbox = Checkbox::new("Test", false)
            .checked_symbol("[+]")
            .unchecked_symbol("[-]");

        assert_eq!(checkbox.checked_symbol, "[+]");
        assert_eq!(checkbox.unchecked_symbol, "[-]");
    }

    #[test]
    fn checkbox_predefined_minus_symbol() {
        use crate::symbols;
        let checkbox = Checkbox::new("Test", false).unchecked_symbol(symbols::UNCHECKED_MINUS);

        assert_eq!(checkbox.unchecked_symbol, "[-]");
    }

    #[test]
    fn checkbox_predefined_parenthesis_symbols() {
        use crate::symbols;
        let checkbox = Checkbox::new("Test", true)
            .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
            .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O);

        assert_eq!(checkbox.checked_symbol, "(X)");
        assert_eq!(checkbox.unchecked_symbol, "(O)");
    }

    #[test]
    fn checkbox_render_emoji() {
        let checkbox = Checkbox::new("Emoji", true)
            .checked_symbol("✅ ")
            .unchecked_symbol("⬜ ");

        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));
        checkbox.render(buffer.area, &mut buffer);

        // Should render without panic
        assert!(buffer.area.area() > 0);
    }

    #[test]
    fn checkbox_label_style_overrides() {
        let checkbox = Checkbox::new("Test", true)
            .style(Style::default().fg(Color::White))
            .label_style(Style::default().fg(Color::Blue));

        assert_eq!(checkbox.style.fg, Some(Color::White));
        assert_eq!(checkbox.label_style.fg, Some(Color::Blue));
    }
}
