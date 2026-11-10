//! # tui-checkbox
//!
//! A customizable checkbox widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.
//!
//! ## Features
//!
//! - ☑️ Simple checkbox with label, plus an indeterminate (partly checked) state
//! - 🎯 Focused and disabled states with their own styles
//! - 🎨 Customizable styling for checkbox and label separately
//! - 🔤 Custom symbols (unicode, emoji, ASCII) and [`symbols`] presets
//! - 🌆 A [`cyber`] look: chamfered slabs, hard shadows, readouts, glow, glitch and decrypt
//! - 📍 Label position, alignment, min/max width and wrapping
//! - 📏 [`Checkbox::width`] / [`Checkbox::height`] for sizing layouts
//! - 🌈 Shared theming via `tui-nuances` (`theme` feature, on by default)
//! - 📦 Optional block wrapper
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
//!
//! Cyberpunk look:
//!
//! ```no_run
//! use tui_checkbox::Checkbox;
//! use tui_checkbox::cyber::{Chamfer, Cyber};
//!
//! let checkbox = Checkbox::new("Arm system", true)
//!     .focused(true)
//!     .cyber(Cyber::new().chamfer(Chamfer::Diagonal).readout(true).glow(true));
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)] // Terminal dimensions are always small

use std::borrow::Cow;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style, Styled};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Widget};

use crate::cyber::{Cyber, CyberPalette, Glyph, Mark, State};

pub mod cyber;
pub mod symbols;

#[cfg(feature = "theme")]
pub use tui_nuances::{Status, Theme, ThemeProvider, ThemeSwitcher, Themed};

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

impl HorizontalAlignment {
    /// Computes the horizontal offset for `content` width within an `available` width.
    const fn offset(self, available: u16, content: u16) -> u16 {
        match self {
            Self::Left => 0,
            Self::Center => available.saturating_sub(content) / 2,
            Self::Right => available.saturating_sub(content),
        }
    }
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

impl VerticalAlignment {
    /// Computes the vertical offset for `content` height within an `available` height.
    const fn offset(self, available: u16, content: u16) -> u16 {
        match self {
            Self::Top => 0,
            Self::Center => available.saturating_sub(content) / 2,
            Self::Bottom => available.saturating_sub(content),
        }
    }
}

/// Converts a borrowed [`Line`] into an owned `Line<'static>`.
///
/// The line's own style is baked into every span (span styles still win), so it survives
/// splitting the line into words and is not lost by rebuilding it from its spans.
fn to_owned_line(line: &Line<'_>) -> Line<'static> {
    Line::from(
        line.spans
            .iter()
            .map(|s| Span::styled(s.content.to_string(), line.style.patch(s.style)))
            .collect::<Vec<_>>(),
    )
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
#[expect(clippy::struct_excessive_bools)] // independent builder flags, each set by its own method
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
    /// Whether the checkbox is partly checked (wins over `checked` when drawing)
    indeterminate: bool,
    /// Symbol to use when indeterminate
    indeterminate_symbol: Cow<'a, str>,
    /// Whether the checkbox has keyboard focus
    focused: bool,
    /// Whether the checkbox is disabled
    disabled: bool,
    /// Style patched over symbol and label while focused
    focus_style: Style,
    /// Style patched over symbol and label while disabled
    disabled_style: Style,
    /// Cyberpunk look (replaces the symbols when set)
    cyber: Option<Cyber>,
    /// Styles derived from a shared theme (lowest precedence)
    #[cfg(feature = "theme")]
    theme: Option<ThemeStyles>,
}

/// Styles derived from a [`Theme`]; explicit per-field styles always win.
#[cfg(feature = "theme")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct ThemeStyles {
    base: Style,
    block: Style,
    block_focused: Style,
    checked: Style,
    unchecked: Style,
    indeterminate: Style,
    label: Style,
    focus: Style,
    disabled: Style,
    cyber: CyberPalette,
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
    /// - Not indeterminate, focused or disabled (`⊟` as the indeterminate symbol,
    ///   bold while focused, dim while disabled)
    /// - The classic look (no [`Cyber`])
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
            indeterminate: false,
            indeterminate_symbol: Cow::Borrowed(symbols::INDETERMINATE),
            focused: false,
            disabled: false,
            focus_style: Style::new().add_modifier(Modifier::BOLD),
            disabled_style: Style::new().add_modifier(Modifier::DIM),
            cyber: None,
            #[cfg(feature = "theme")]
            theme: None,
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

    /// Marks the checkbox as partly checked (e.g. a "select all" over a partial selection).
    ///
    /// An indeterminate checkbox draws [`indeterminate_symbol`](Self::indeterminate_symbol)
    /// whatever its checked state. The default is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let select_all = Checkbox::new("Select all", false).indeterminate(true);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Sets the symbol to use when the checkbox is indeterminate.
    ///
    /// The default is `⊟` ([`symbols::INDETERMINATE`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Some", false)
    ///     .indeterminate(true)
    ///     .indeterminate_symbol("[~]");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn indeterminate_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.indeterminate_symbol = symbol.into();
        self
    }

    /// Marks the checkbox as focused; [`focus_style`](Self::focus_style) is patched over the
    /// symbol and the label. The default is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let selected = 1;
    /// let checkbox = Checkbox::new("Option", false).focused(selected == 1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Marks the checkbox as disabled; [`disabled_style`](Self::disabled_style) is patched
    /// over the symbol and the label (after the focus style). The default is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Unavailable", false).disabled(true);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the style patched over symbol and label while [focused](Self::focused).
    ///
    /// The default is bold. With a theme it is patched over the theme's focus colour.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false)
    ///     .focused(true)
    ///     .focus_style(Style::default().fg(Color::Yellow));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn focus_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.focus_style = style.into();
        self
    }

    /// Sets the style patched over symbol and label while [disabled](Self::disabled).
    ///
    /// The default is dim. With a theme it is patched over the theme's disabled colour.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use tui_checkbox::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false)
    ///     .disabled(true)
    ///     .disabled_style(Style::default().fg(Color::DarkGray));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn disabled_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.disabled_style = style.into();
        self
    }

    /// Switches to the cyberpunk look (chamfered slabs, readouts, glitches).
    ///
    /// The look replaces the checked/unchecked/indeterminate symbols and
    /// [`checkbox_style`](Self::checkbox_style); its colours come from the
    /// [`CyberPalette`] (explicit, else the theme's, else [`CyberPalette::CYBERPUNK`]).
    /// [`style`](Self::style) and [`label_style`](Self::label_style) still apply.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    /// use tui_checkbox::cyber::{Chamfer, Cyber};
    ///
    /// let checkbox = Checkbox::new("Arm system", true)
    ///     .cyber(Cyber::new().chamfer(Chamfer::Diagonal).readout(true));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn cyber(mut self, cyber: Cyber) -> Self {
        self.cyber = Some(cyber);
        self
    }

    /// Whether the checkbox is checked.
    #[must_use]
    pub const fn is_checked(&self) -> bool {
        self.checked
    }

    /// Width in cells the checkbox needs to show its symbol and unwrapped label,
    /// including the block, for sizing a layout [`Constraint`](ratatui::layout::Constraint).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    ///
    /// assert_eq!(Checkbox::new("Hi", true).width(), 4); // "☑ Hi"
    /// ```
    #[must_use]
    pub fn width(&self) -> u16 {
        let (w, _) = self.content_size();
        w.saturating_add(self.block_margins().0)
    }

    /// Height in rows the checkbox needs (see [`width`](Self::width)).
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_checkbox::Checkbox;
    /// use ratatui::widgets::Block;
    ///
    /// assert_eq!(Checkbox::new("Hi", true).height(), 1);
    /// assert_eq!(Checkbox::new("Hi", true).block(Block::bordered()).height(), 3);
    /// ```
    #[must_use]
    pub fn height(&self) -> u16 {
        let (_, h) = self.content_size();
        h.saturating_add(self.block_margins().1)
    }
}

#[cfg(feature = "theme")]
impl Checkbox<'_> {
    /// Styles the checkbox from a shared [`Theme`].
    ///
    /// Role mapping: base = `theme.style()`, block border = `theme.border_style(false)`
    /// (`border_style(true)` while focused), checked symbol = `primary`, unchecked symbol =
    /// `muted`, indeterminate symbol = `secondary`, label = `foreground`, focus =
    /// `border_focused` + bold, disabled = `disabled`; the cyber look uses
    /// [`CyberPalette::from_theme`].
    ///
    /// **Precedence:** explicit [`style`](Self::style),
    /// [`checkbox_style`](Self::checkbox_style) and [`label_style`](Self::label_style)
    /// always win over theme defaults, regardless of call order. A theme border style
    /// is used as the block's base style, so a block's own `border_style` also wins.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "theme")] {
    /// use tui_checkbox::{Checkbox, Theme};
    ///
    /// let cb = Checkbox::new("Dark mode", true).theme(&Theme::nord());
    /// # }
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn theme(mut self, theme: impl Into<Theme>) -> Self {
        let t: Theme = theme.into();
        self.theme = Some(ThemeStyles {
            base: t.style(),
            block: t.border_style(false),
            block_focused: t.border_style(true),
            checked: Style::default().fg(t.primary),
            unchecked: Style::default().fg(t.muted),
            indeterminate: Style::default().fg(t.secondary),
            label: Style::default().fg(t.foreground),
            focus: t.border_style(true),
            disabled: t.disabled_style(),
            cyber: CyberPalette::from_theme(&t),
        });
        self
    }
}

#[cfg(feature = "theme")]
impl Themed for Checkbox<'_> {
    fn apply_theme(self, theme: &Theme) -> Self {
        self.theme(theme.clone())
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
        let base = self.base_style();
        buf.set_style(area, base);
        let inner = if let Some(ref block) = self.block {
            let inner_area = block.inner(area);
            #[cfg(feature = "theme")]
            if let Some(t) = &self.theme {
                let border = if self.focused {
                    t.block_focused
                } else {
                    t.block
                };
                block.clone().style(base.patch(border)).render(area, buf);
            } else {
                block.render(area, buf);
            }
            #[cfg(not(feature = "theme"))]
            block.render(area, buf);
            inner_area
        } else {
            area
        };
        self.render_checkbox(inner, buf);
    }
}

/// What a checkbox is laid out from: its symbol and its (wrapped) label lines.
struct Parts {
    glyph: Glyph,
    label: Vec<Line<'static>>,
}

impl Parts {
    fn label_width(&self) -> u16 {
        self.label
            .iter()
            .map(|l| l.width() as u16)
            .max()
            .unwrap_or(0)
    }

    /// Cells between symbol and label (none for an empty label).
    fn gap(&self) -> u16 {
        u16::from(self.label_width() > 0)
    }

    fn glyph_height(&self) -> u16 {
        self.glyph.lines.len() as u16
    }

    fn label_height(&self) -> u16 {
        self.label.len() as u16
    }
}

/// Renders `lines` from `(x, y)` downwards, clipped to `area`.
fn draw_lines(buf: &mut Buffer, area: Rect, x: u16, y: u16, lines: &[Line<'_>]) {
    if x >= area.right() {
        return;
    }
    for (i, line) in lines.iter().enumerate() {
        let ly = y.saturating_add(i as u16);
        if ly >= area.bottom() {
            break;
        }
        line.render(Rect::new(x, ly, area.right() - x, 1), buf);
    }
}

/// Display width of `text` in cells.
fn text_width(text: &str) -> usize {
    Span::raw(text).width()
}

impl Checkbox<'_> {
    /// Theme base style patched by the explicit style.
    fn base_style(&self) -> Style {
        #[cfg(feature = "theme")]
        let themed = self.theme.map_or(Style::default(), |t| t.base);
        #[cfg(not(feature = "theme"))]
        let themed = Style::default();
        themed.patch(self.style)
    }

    const fn state(&self) -> State {
        State {
            mark: if self.indeterminate {
                Mark::Mixed
            } else if self.checked {
                Mark::Checked
            } else {
                Mark::Unchecked
            },
            focused: self.focused,
            disabled: self.disabled,
        }
    }

    /// Theme styles for `(symbol, label, focus, disabled)`; empty without a theme.
    // Without the `theme` feature there is no theme to read.
    #[cfg_attr(not(feature = "theme"), expect(clippy::unused_self))]
    fn theme_styles(&self, mark: Mark) -> (Style, Style, Style, Style) {
        #[cfg(feature = "theme")]
        if let Some(t) = self.theme {
            let symbol = match mark {
                Mark::Checked => t.checked,
                Mark::Unchecked => t.unchecked,
                Mark::Mixed => t.indeterminate,
            };
            return (symbol, t.label, t.focus, t.disabled);
        }
        #[cfg(not(feature = "theme"))]
        let _ = mark;
        Default::default()
    }

    /// The cyber palette: explicit, else from the theme, else the default.
    // Without the `theme` feature there is no theme to read.
    #[cfg_attr(not(feature = "theme"), expect(clippy::unused_self))]
    fn cyber_palette(&self, cyber: &Cyber) -> CyberPalette {
        if let Some(p) = cyber.explicit_palette() {
            return p;
        }
        #[cfg(feature = "theme")]
        if let Some(t) = self.theme {
            return t.cyber;
        }
        CyberPalette::CYBERPUNK
    }

    /// Patches the focus and disabled styles over `style` as the state asks.
    fn decorate(&self, mut style: Style, focus: Style, disabled: Style) -> Style {
        if self.focused {
            style = style.patch(focus.patch(self.focus_style));
        }
        if self.disabled {
            style = style.patch(disabled.patch(self.disabled_style));
        }
        style
    }

    /// Symbol and label lines; with wrapping on, the label wraps to fit `available` columns.
    fn parts(&self, available: Option<u16>) -> Parts {
        let base = self.base_style();
        let state = self.state();
        let (sym_theme, label_theme, focus, disabled) = self.theme_styles(state.mark);
        let cyber = self.cyber.map(|c| (c, self.cyber_palette(&c)));

        let (glyph, label) = if let Some((c, pal)) = &cyber {
            let style = base
                .patch(cyber::label_style(pal, state))
                .patch(self.label_style);
            let style = self.decorate(style, focus, disabled);
            let label = to_owned_line(&self.label.clone().patch_style(style));
            (
                cyber::glyph(c, pal, state),
                cyber::transform_label(c, pal, label),
            )
        } else {
            let symbol = match state.mark {
                Mark::Checked => &self.checked_symbol,
                Mark::Unchecked => &self.unchecked_symbol,
                Mark::Mixed => &self.indeterminate_symbol,
            };
            let style = base.patch(sym_theme).patch(self.checkbox_style);
            let span = Span::styled(symbol.to_string(), self.decorate(style, focus, disabled));
            let glyph = Glyph {
                width: span.width() as u16,
                lines: vec![Line::from(span)],
                label_row: 0,
            };
            // The explicit base style wins over the theme's label colour.
            let style = base
                .patch(label_theme)
                .patch(self.style)
                .patch(self.label_style);
            let style = self.decorate(style, focus, disabled);
            (glyph, to_owned_line(&self.label.clone().patch_style(style)))
        };

        // The cyber bar pads each label line by one cell on both sides.
        let pad = if cyber.is_some() { 2 } else { 0 };
        let wrap_width = available.map(|w| {
            let w = match self.label_position {
                LabelPosition::Left | LabelPosition::Right => w.saturating_sub(glyph.width + 1),
                LabelPosition::Top | LabelPosition::Bottom => w,
            };
            w.saturating_sub(pad)
        });
        let mut label = match wrap_width {
            Some(w) if self.wrap_label => Self::wrap_text(&label, w),
            _ => vec![label],
        };
        if let Some((c, pal)) = &cyber {
            // The bar goes around the label only; the readout follows it.
            cyber::bar(c, pal, state, &mut label);
            if let Some(readout) = cyber::readout(c, pal, state) {
                match (c.is_block(), label.last_mut()) {
                    (false, Some(last)) => {
                        last.spans.push(Span::raw(" "));
                        last.spans.push(readout);
                    }
                    _ => label.push(Line::from(vec![Span::raw(" "), readout])),
                }
            }
        }
        Parts { glyph, label }
    }

    /// Unwrapped content size, without the block.
    fn content_size(&self) -> (u16, u16) {
        let p = self.parts(None);
        match self.label_position {
            LabelPosition::Left | LabelPosition::Right => (
                p.glyph
                    .width
                    .saturating_add(p.gap())
                    .saturating_add(p.label_width()),
                p.glyph_height()
                    .max(p.glyph.label_row.saturating_add(p.label_height())),
            ),
            LabelPosition::Top | LabelPosition::Bottom => (
                p.glyph.width.max(p.label_width()),
                p.glyph_height().saturating_add(p.label_height()),
            ),
        }
    }

    /// Columns and rows the block takes around the content.
    fn block_margins(&self) -> (u16, u16) {
        self.block.as_ref().map_or((0, 0), |b| {
            let outer = Rect::new(0, 0, 1000, 1000);
            let inner = b.inner(outer);
            (outer.width - inner.width, outer.height - inner.height)
        })
    }

    fn render_checkbox(&self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }
        let mut area = area;
        if let Some(max_width) = self.max_width {
            area.width = area.width.min(max_width);
        }
        let p = self.parts(Some(area.width));
        let min_width = self.min_width.unwrap_or(0);
        let (gw, lw) = (p.glyph.width, p.label_width());

        match self.label_position {
            LabelPosition::Right | LabelPosition::Left => {
                let gap = p.gap();
                let content_w = gw.saturating_add(gap).saturating_add(lw);
                // `min_width` widens the box the alignment places; content starts at its left.
                let box_w = content_w.max(min_width);
                let content_h = p
                    .glyph_height()
                    .max(p.glyph.label_row.saturating_add(p.label_height()));
                let x = area.x + self.horizontal_alignment.offset(area.width, box_w);
                let y = area.y + self.vertical_alignment.offset(area.height, content_h);
                let (gx, lx) = if self.label_position == LabelPosition::Right {
                    (x, x.saturating_add(gw.saturating_add(gap)))
                } else {
                    (x.saturating_add(lw.saturating_add(gap)), x)
                };
                draw_lines(buf, area, gx, y, &p.glyph.lines);
                draw_lines(buf, area, lx, y.saturating_add(p.glyph.label_row), &p.label);
            }
            LabelPosition::Top | LabelPosition::Bottom => {
                let box_w = gw.max(lw).max(min_width);
                let content_h = p.glyph_height().saturating_add(p.label_height());
                let x = area.x + self.horizontal_alignment.offset(area.width, box_w);
                let y = area.y + self.vertical_alignment.offset(area.height, content_h);
                let gx = x.saturating_add(self.horizontal_alignment.offset(box_w, gw));
                let (gy, ly) = if self.label_position == LabelPosition::Top {
                    (y.saturating_add(p.label_height()), y)
                } else {
                    (y, y.saturating_add(p.glyph_height()))
                };
                draw_lines(buf, area, gx, gy, &p.glyph.lines);
                for (i, line) in p.label.iter().enumerate() {
                    let lx = x.saturating_add(
                        self.horizontal_alignment.offset(box_w, line.width() as u16),
                    );
                    draw_lines(
                        buf,
                        area,
                        lx,
                        ly.saturating_add(i as u16),
                        std::slice::from_ref(line),
                    );
                }
            }
        }
    }

    /// Word-wraps `line` to `max_width` display cells.
    ///
    /// Words wider than the line are broken between characters; widths are display widths,
    /// so wide (CJK, emoji) characters count as two cells. A `max_width` of zero returns the
    /// line unwrapped.
    fn wrap_text(line: &Line<'_>, max_width: u16) -> Vec<Line<'static>> {
        if max_width == 0 {
            return vec![to_owned_line(line)];
        }
        let max = usize::from(max_width);
        let mut lines: Vec<Vec<Span<'static>>> = vec![Vec::new()];
        let mut width = 0usize;
        let push = |lines: &mut Vec<Vec<Span<'static>>>, text: &str, style: Style| {
            if !text.is_empty()
                && let Some(last) = lines.last_mut()
            {
                last.push(Span::styled(text.to_string(), style));
            }
        };

        for span in &line.spans {
            let style = line.style.patch(span.style);
            for (i, word) in span.content.split(' ').enumerate() {
                let word_width = text_width(word);
                let sep = usize::from(i > 0 && width > 0);
                if width > 0 && width + sep + word_width > max {
                    lines.push(Vec::new());
                    width = 0;
                } else if sep == 1 {
                    push(&mut lines, " ", style);
                    width += 1;
                }
                if word_width <= max.saturating_sub(width) {
                    push(&mut lines, word, style);
                    width += word_width;
                    continue;
                }
                // Longer than a whole line: break between characters.
                let mut chunk = String::new();
                for ch in word.chars() {
                    let ch_width = text_width(ch.encode_utf8(&mut [0; 4]));
                    if width > 0 && width + ch_width > max {
                        push(&mut lines, &chunk, style);
                        chunk.clear();
                        lines.push(Vec::new());
                        width = 0;
                    }
                    chunk.push(ch);
                    width += ch_width;
                }
                push(&mut lines, &chunk, style);
            }
        }

        let result: Vec<Line<'static>> = lines
            .into_iter()
            .filter(|spans| !spans.is_empty())
            .map(Line::from)
            .collect();
        if result.is_empty() {
            vec![to_owned_line(line)]
        } else {
            result
        }
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
        assert!(
            buffer
                .cell(buffer.area.as_position())
                .unwrap()
                .symbol()
                .starts_with('☐')
        );
    }

    #[test]
    fn checkbox_render_checked() {
        let checkbox = Checkbox::new("Test", true);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        // The buffer should contain the checked symbol followed by space and label
        assert!(
            buffer
                .cell(buffer.area.as_position())
                .unwrap()
                .symbol()
                .starts_with('☑')
        );
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

        assert!(
            buffer
                .cell(buffer.area.as_position())
                .unwrap()
                .symbol()
                .starts_with('[')
        );
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

    /// Generates a test asserting that custom checked/unchecked symbols are stored verbatim.
    macro_rules! symbol_tests {
        ($($name:ident: $checked:expr, $unchecked:expr;)*) => {
            $(
                #[test]
                fn $name() {
                    let checkbox = Checkbox::new("Test", true)
                        .checked_symbol($checked)
                        .unchecked_symbol($unchecked);
                    assert_eq!(checkbox.checked_symbol, $checked);
                    assert_eq!(checkbox.unchecked_symbol, $unchecked);
                }
            )*
        };
    }

    symbol_tests! {
        checkbox_emoji_symbols: "✅ ", "⬜ ";
        checkbox_unicode_symbols: "● ", "○ ";
        checkbox_arrow_symbols: "▶ ", "▷ ";
        checkbox_parenthesis_symbols: "(X)", "(O)";
        checkbox_minus_symbols: "[+]", "[-]";
        checkbox_predefined_symbols: symbols::CHECKED_X, symbols::UNCHECKED_SPACE;
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

    #[test]
    fn horizontal_alignment_offset() {
        assert_eq!(HorizontalAlignment::Left.offset(20, 6), 0);
        assert_eq!(HorizontalAlignment::Center.offset(20, 6), 7);
        assert_eq!(HorizontalAlignment::Right.offset(20, 6), 14);
        // Content wider than area saturates to zero.
        assert_eq!(HorizontalAlignment::Center.offset(4, 10), 0);
        assert_eq!(HorizontalAlignment::Right.offset(4, 10), 0);
    }

    #[test]
    fn vertical_alignment_offset() {
        assert_eq!(VerticalAlignment::Top.offset(10, 3), 0);
        assert_eq!(VerticalAlignment::Center.offset(10, 3), 3);
        assert_eq!(VerticalAlignment::Bottom.offset(10, 3), 7);
        assert_eq!(VerticalAlignment::Bottom.offset(2, 5), 0);
    }

    #[test]
    fn to_owned_line_clones_content_and_style() {
        let line = Line::from(vec![Span::styled("hi", Style::default().fg(Color::Red))]);
        let owned = to_owned_line(&line);
        assert_eq!(owned.spans[0].content, "hi");
        assert_eq!(owned.spans[0].style.fg, Some(Color::Red));
    }

    #[test]
    fn checkbox_builder_defaults() {
        let checkbox = Checkbox::default();
        assert_eq!(checkbox.label_position, LabelPosition::Right);
        assert_eq!(checkbox.horizontal_alignment, HorizontalAlignment::Left);
        assert_eq!(checkbox.vertical_alignment, VerticalAlignment::Top);
        assert_eq!(checkbox.min_width, None);
        assert_eq!(checkbox.max_width, None);
        assert!(!checkbox.wrap_label);
    }

    #[test]
    fn checkbox_layout_builders() {
        let checkbox = Checkbox::new("Test", true)
            .label_position(LabelPosition::Left)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Bottom)
            .min_width(10)
            .max_width(40)
            .wrap_label(true);

        assert_eq!(checkbox.label_position, LabelPosition::Left);
        assert_eq!(checkbox.horizontal_alignment, HorizontalAlignment::Center);
        assert_eq!(checkbox.vertical_alignment, VerticalAlignment::Bottom);
        assert_eq!(checkbox.min_width, Some(10));
        assert_eq!(checkbox.max_width, Some(40));
        assert!(checkbox.wrap_label);
    }

    #[test]
    fn checkbox_render_label_left() {
        let checkbox = Checkbox::new("Hi", true).label_position(LabelPosition::Left);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);
        // Label renders before checkbox, so first cell is the label.
        assert!(
            buffer
                .cell(buffer.area.as_position())
                .unwrap()
                .symbol()
                .starts_with('H')
        );
    }

    #[test]
    fn checkbox_render_label_top_and_bottom() {
        for position in [LabelPosition::Top, LabelPosition::Bottom] {
            let checkbox = Checkbox::new("Hi", true).label_position(position);
            let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
            // Should render both lines without panic.
            checkbox.render(buffer.area, &mut buffer);
        }
    }

    #[test]
    fn checkbox_render_wrapped_label() {
        let checkbox = Checkbox::new("one two three four five", false)
            .wrap_label(true)
            .max_width(12);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 4));
        // Wrapping across multiple lines should not panic.
        checkbox.render(buffer.area, &mut buffer);
    }

    #[test]
    fn checkbox_min_width_expands_and_max_width_limits() {
        // min_width does not panic when larger than the area.
        let checkbox = Checkbox::new("Test", true).min_width(50);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        // max_width smaller than the area constrains rendering.
        let checkbox = Checkbox::new("A long label here", false).max_width(5);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 30, 1));
        checkbox.render(buffer.area, &mut buffer);
    }

    #[test]
    fn wrap_text_zero_width_returns_single_line() {
        let line = Line::from("hello world");
        let wrapped = Checkbox::wrap_text(&line, 0);
        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0].width(), "hello world".len());
    }

    #[test]
    fn wrap_text_splits_on_width() {
        let line = Line::from("aaa bbb ccc");
        let wrapped = Checkbox::wrap_text(&line, 4);
        assert_eq!(wrapped.len(), 3);
    }

    #[test]
    fn checkbox_render_alignment_variants() {
        for h in [
            HorizontalAlignment::Left,
            HorizontalAlignment::Center,
            HorizontalAlignment::Right,
        ] {
            for v in [
                VerticalAlignment::Top,
                VerticalAlignment::Center,
                VerticalAlignment::Bottom,
            ] {
                let checkbox = Checkbox::new("Test", true)
                    .horizontal_alignment(h)
                    .vertical_alignment(v);
                let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 3));
                checkbox.render(buffer.area, &mut buffer);
            }
        }
    }

    /// Text of row `y` in `buf`, skipping the cells a wide character covers.
    fn row(buf: &Buffer, y: u16) -> String {
        let mut s = String::new();
        let mut skip = 0;
        for x in buf.area.left()..buf.area.right() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            let sym = buf[(x, y)].symbol();
            skip = text_width(sym).saturating_sub(1);
            s.push_str(sym);
        }
        s.trim_end().to_string()
    }

    fn draw(cb: &Checkbox<'_>, w: u16, h: u16) -> Buffer {
        let mut buf = Buffer::empty(Rect::new(0, 0, w, h));
        cb.render(buf.area, &mut buf);
        buf
    }

    #[test]
    fn regression_label_style_is_applied() {
        // `to_owned_line` used to drop the line style, so `label_style` never showed.
        let cb = Checkbox::new("Hi", true).label_style(Style::default().fg(Color::Red));
        let buf = draw(&cb, 10, 1);
        assert_eq!(buf[(2, 0)].fg, Color::Red);
        assert_eq!(buf[(3, 0)].fg, Color::Red);
        // A span's own style still wins over `label_style`.
        let cb = Checkbox::new("Hi".blue(), true).label_style(Style::default().fg(Color::Red));
        assert_eq!(draw(&cb, 10, 1)[(2, 0)].fg, Color::Blue);
    }

    #[test]
    fn regression_min_width_widens_the_aligned_box() {
        // `min_width` used to be clamped back to the area width, so it did nothing.
        let cb = Checkbox::new("ab", true).horizontal_alignment(HorizontalAlignment::Center);
        assert_eq!(row(&draw(&cb, 20, 1), 0), "        ☑ ab");
        let cb = cb.min_width(10);
        assert_eq!(row(&draw(&cb, 20, 1), 0), "     ☑ ab");
        // Larger than the area: the box fills the area, nothing panics.
        let cb = Checkbox::new("ab", true).min_width(50);
        assert_eq!(row(&draw(&cb, 10, 1), 0), "☑ ab");
    }

    #[test]
    fn regression_wrap_counts_display_width() {
        // Wide characters are two cells; counting chars overflowed the line.
        let wrapped = Checkbox::wrap_text(&Line::from("界界 界界"), 5);
        assert_eq!(wrapped.len(), 2);
        assert!(wrapped.iter().all(|l| l.width() <= 5));
    }

    #[test]
    fn regression_huge_min_width_does_not_overflow() {
        // Found by the render_cyber fuzz target: offsets inside a huge box overflowed u16.
        for position in [LabelPosition::Right, LabelPosition::Top] {
            let cb = Checkbox::new("x", true)
                .min_width(u16::MAX)
                .label_position(position)
                .horizontal_alignment(HorizontalAlignment::Center);
            let _ = draw(&cb, 10, 3);
        }
    }

    #[test]
    fn regression_wide_char_in_one_cell_line_does_not_underflow() {
        let wrapped = Checkbox::wrap_text(&Line::from("界 a"), 1);
        let texts: Vec<String> = wrapped.iter().map(ToString::to_string).collect();
        assert_eq!(texts, ["界", "a"]);
    }

    #[test]
    fn wrap_breaks_overlong_words_without_leading_spaces() {
        let wrapped = Checkbox::wrap_text(&Line::from("abcdefgh ij"), 3);
        let texts: Vec<String> = wrapped.iter().map(ToString::to_string).collect();
        assert_eq!(texts, ["abc", "def", "gh", "ij"]);
    }

    #[test]
    fn wrap_keeps_span_styles() {
        let line = Line::from(vec!["red ".red(), "blue".blue()]);
        let wrapped = Checkbox::wrap_text(&line, 4);
        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[1].spans[0].style.fg, Some(Color::Blue));
    }

    #[test]
    fn indeterminate_wins_over_checked() {
        for checked in [true, false] {
            let cb = Checkbox::new("x", checked).indeterminate(true);
            assert_eq!(row(&draw(&cb, 10, 1), 0), "⊟ x");
        }
        let cb = Checkbox::new("x", true)
            .indeterminate(true)
            .indeterminate_symbol(symbols::INDETERMINATE_TILDE);
        assert_eq!(row(&draw(&cb, 10, 1), 0), "[~] x");
    }

    #[test]
    fn focused_and_disabled_patch_their_styles() {
        let buf = draw(&Checkbox::new("x", true).focused(true), 10, 1);
        assert!(buf[(0, 0)].modifier.contains(Modifier::BOLD));
        assert!(buf[(2, 0)].modifier.contains(Modifier::BOLD));
        let buf = draw(&Checkbox::new("x", true).disabled(true), 10, 1);
        assert!(buf[(0, 0)].modifier.contains(Modifier::DIM));
        let cb = Checkbox::new("x", true)
            .focused(true)
            .disabled(true)
            .focus_style(Style::default().fg(Color::Yellow))
            .disabled_style(Style::default().fg(Color::DarkGray));
        // Disabled is patched last.
        assert_eq!(draw(&cb, 10, 1)[(2, 0)].fg, Color::DarkGray);
        // Neither flag set: no change.
        let buf = draw(&Checkbox::new("x", true), 10, 1);
        assert!(buf[(0, 0)].modifier.is_empty());
    }

    #[test]
    fn empty_label_has_no_gap() {
        let cb = Checkbox::new("", true).horizontal_alignment(HorizontalAlignment::Right);
        assert_eq!(cb.width(), 1);
        assert_eq!(row(&draw(&cb, 4, 1), 0), "   ☑");
    }

    #[test]
    fn width_and_height_cover_symbol_label_and_block() {
        assert_eq!(
            (
                Checkbox::new("Hi", true).width(),
                Checkbox::new("Hi", true).height()
            ),
            (4, 1)
        );
        let cb = Checkbox::new("Hi", true).label_position(LabelPosition::Top);
        assert_eq!((cb.width(), cb.height()), (2, 2));
        let cb = Checkbox::new("Hi", true).block(Block::bordered());
        assert_eq!((cb.width(), cb.height()), (6, 3));
        let cb = Checkbox::new("Hi", true).cyber(Cyber::new());
        assert_eq!((cb.width(), cb.height()), (8, 1));
        let cb = Checkbox::new("Hi", true).cyber(
            Cyber::new()
                .size(cyber::CyberSize::Block)
                .shadow(true)
                .readout(true),
        );
        // 6-cell slab with shadow, gap, ` //ONLINE`; label on row 1, readout on row 2.
        assert_eq!((cb.width(), cb.height()), (16, 4));
    }

    #[test]
    fn cyber_renders_slab_label_and_readout() {
        let cb = Checkbox::new("Arm", true).cyber(Cyber::new().readout(true));
        let buf = draw(&cb, 30, 1);
        assert_eq!(row(&buf, 0), "▐✓◤  ARM  //ONLINE");
        let pal = CyberPalette::CYBERPUNK;
        assert_eq!(buf[(1, 0)].bg, pal.primary);
        assert_eq!(buf[(1, 0)].fg, pal.ink);
        assert_eq!(buf[(0, 0)].fg, pal.primary);
        assert_eq!(buf[(5, 0)].fg, pal.text);
    }

    #[test]
    fn cyber_flat_focus_has_no_fill_and_keeps_the_layout() {
        let flat = Checkbox::new("Arm", true)
            .focused(true)
            .cyber(Cyber::new().flat(true).shadow(true).glow(true));
        let solid = Checkbox::new("Arm", true).focused(true).cyber(Cyber::new());
        assert_eq!(
            (flat.width(), flat.height()),
            (solid.width(), solid.height())
        );
        let buf = draw(&flat, 20, 2);
        assert_eq!(row(&buf, 0), "▐✓◤  ARM◤");
        assert_eq!(row(&buf, 1), "");
        let pal = CyberPalette::CYBERPUNK;
        // The slab is still filled; the label is not.
        assert_eq!(buf[(1, 0)].bg, pal.primary);
        assert_eq!(buf[(5, 0)].fg, pal.primary);
        assert_ne!(buf[(5, 0)].bg, pal.primary);
    }

    #[test]
    fn cyber_block_label_left_sits_on_the_middle_row() {
        let cb = Checkbox::new("Arm", true)
            .label_position(LabelPosition::Left)
            .cyber(Cyber::new().size(cyber::CyberSize::Block));
        let buf = draw(&cb, 12, 3);
        assert_eq!(row(&buf, 1), " ARM    ✓");
        assert_eq!(row(&buf, 2), "          ◤");
    }

    #[test]
    fn cyber_palette_explicit_wins_and_label_style_applies() {
        let cb = Checkbox::new("a", true)
            .label_style(Style::default().fg(Color::Green))
            .cyber(Cyber::new().palette(CyberPalette::NEON));
        let buf = draw(&cb, 10, 1);
        assert_eq!(buf[(1, 0)].bg, CyberPalette::NEON.primary);
        assert_eq!(buf[(5, 0)].fg, Color::Green);
    }
}

#[cfg(all(test, feature = "theme"))]
mod theme_tests {
    use super::*;
    use ratatui::style::Color;

    fn render(cb: Checkbox<'_>) -> Buffer {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        cb.render(area, &mut buf);
        buf
    }

    #[test]
    fn no_theme_unchanged() {
        let a = render(Checkbox::new("x", true));
        assert_eq!(a[(0, 0)].fg, Color::Reset);
    }

    #[test]
    fn mapping() {
        let t = Theme::nord();
        let b = render(Checkbox::new("x", true).theme(&t));
        assert_eq!(b[(0, 0)].fg, t.primary);
        assert_eq!(b[(2, 0)].fg, t.foreground);
        let b = render(Checkbox::new("x", false).theme(&t));
        assert_eq!(b[(0, 0)].fg, t.muted);
    }

    #[test]
    fn explicit_wins_any_order() {
        let t = Theme::nord();
        let before = render(
            Checkbox::new("x", true)
                .checkbox_style(Style::default().fg(Color::Red))
                .theme(&t),
        );
        let after = render(
            Checkbox::new("x", true)
                .theme(&t)
                .checkbox_style(Style::default().fg(Color::Red)),
        );
        assert_eq!(before, after);
        assert_eq!(before[(0, 0)].fg, Color::Red);
    }

    #[test]
    fn all_themes_render() {
        for t in Theme::all() {
            let _ = render(
                Checkbox::new("x", true)
                    .block(Block::bordered())
                    .apply_theme(&t),
            );
        }
    }

    #[test]
    fn theme_maps_indeterminate_focus_and_disabled() {
        let t = Theme::dracula();
        let buf = render(Checkbox::new("x", false).indeterminate(true).theme(&t));
        assert_eq!(buf[(0, 0)].fg, t.secondary);
        let buf = render(Checkbox::new("x", false).focused(true).theme(&t));
        assert_eq!(buf[(2, 0)].fg, t.border_focused);
        let buf = render(Checkbox::new("x", false).disabled(true).theme(&t));
        assert_eq!(buf[(2, 0)].fg, t.disabled);
        // Explicit focus style wins over the theme.
        let buf = render(
            Checkbox::new("x", false)
                .focused(true)
                .focus_style(Style::default().fg(Color::Red))
                .theme(&t),
        );
        assert_eq!(buf[(2, 0)].fg, Color::Red);
    }

    #[test]
    fn focused_block_uses_the_focused_border() {
        let t = Theme::nord();
        let buf = render(
            Checkbox::new("x", true)
                .block(Block::bordered())
                .focused(true)
                .theme(&t),
        );
        assert_eq!(buf[(0, 0)].fg, t.border_focused);
        let buf = render(Checkbox::new("x", true).block(Block::bordered()).theme(&t));
        assert_eq!(buf[(0, 0)].fg, t.border);
    }

    #[test]
    fn cyber_takes_its_palette_from_the_theme_unless_explicit() {
        let t = Theme::nord();
        let buf = render(
            Checkbox::new("x", true)
                .cyber(cyber::Cyber::new())
                .theme(&t),
        );
        assert_eq!(buf[(1, 0)].bg, t.primary);
        let pal = cyber::CyberPalette::SYNTHWAVE;
        let buf = render(
            Checkbox::new("x", true)
                .theme(&t)
                .cyber(cyber::Cyber::new().palette(pal)),
        );
        assert_eq!(buf[(1, 0)].bg, pal.primary);
        assert_eq!(cyber::CyberPalette::from_theme(&t).text, t.foreground);
    }
}
