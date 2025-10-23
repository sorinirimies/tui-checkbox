//! Symbols for checkbox widget
//!
//! This module provides predefined checkbox symbols that can be used with the [`Checkbox`] widget.
//!
//! [`Checkbox`]: crate::Checkbox

/// Checked checkbox symbol (☑)
pub const CHECKED: &str = "☑";

/// Unchecked checkbox symbol (☐)
pub const UNCHECKED: &str = "☐";

/// Alternative checked checkbox symbol with X
pub const CHECKED_X: &str = "[X]";

/// Alternative unchecked checkbox symbol with space
pub const UNCHECKED_SPACE: &str = "[ ]";

/// Alternative checked checkbox symbol with asterisk
pub const CHECKED_ASTERISK: &str = "[*]";

/// Alternative checked checkbox symbol with plus
///
/// # Examples
///
/// ```
/// use tui_checkbox::{Checkbox, symbols};
///
/// let checkbox = Checkbox::new("Task", true)
///     .checked_symbol(symbols::CHECKED_PLUS);
/// ```
pub const CHECKED_PLUS: &str = "[+]";

/// Alternative unchecked checkbox symbol with minus
///
/// # Examples
///
/// ```
/// use tui_checkbox::{Checkbox, symbols};
///
/// let checkbox = Checkbox::new("Task", false)
///     .unchecked_symbol(symbols::UNCHECKED_MINUS);
/// ```
pub const UNCHECKED_MINUS: &str = "[-]";

/// Alternative checked checkbox symbol with X in parenthesis
///
/// # Examples
///
/// ```
/// use tui_checkbox::{Checkbox, symbols};
///
/// let checkbox = Checkbox::new("Task", true)
///     .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
///     .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O);
/// ```
pub const CHECKED_PARENTHESIS_X: &str = "(X)";

/// Alternative unchecked checkbox symbol with O in parenthesis
///
/// # Examples
///
/// ```
/// use tui_checkbox::{Checkbox, symbols};
///
/// let checkbox = Checkbox::new("Task", false)
///     .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
///     .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O);
/// ```
pub const UNCHECKED_PARENTHESIS_O: &str = "(O)";
