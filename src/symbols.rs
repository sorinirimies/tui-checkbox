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

/// Unchecked checkbox symbol with a space in parenthesis, the pair of
/// [`CHECKED_PARENTHESIS_X`]
pub const UNCHECKED_PARENTHESIS_SPACE: &str = "( )";

/// Indeterminate (partly checked) checkbox symbol (⊟), the default for
/// [`Checkbox::indeterminate`](crate::Checkbox::indeterminate)
pub const INDETERMINATE: &str = "⊟";

/// Indeterminate checkbox symbol in the ASCII family (`[~]`)
pub const INDETERMINATE_TILDE: &str = "[~]";

/// Round checked symbol (◉)
pub const ROUND_CHECKED: &str = "◉";

/// Round unchecked symbol (○)
pub const ROUND_UNCHECKED: &str = "○";

/// Square checked symbol (■)
pub const SQUARE_CHECKED: &str = "■";

/// Square unchecked symbol (□)
pub const SQUARE_UNCHECKED: &str = "□";

/// Check mark (✓), e.g. with [`CROSS`] for done / failed lists
pub const CHECK: &str = "✓";

/// Cross mark (✗)
pub const CROSS: &str = "✗";
