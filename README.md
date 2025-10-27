# tui-checkbox

[![Crates.io](https://img.shields.io/crates/v/tui-checkbox)](https://crates.io/crates/tui-checkbox)
[![Documentation](https://docs.rs/tui-checkbox/badge.svg)](https://docs.rs/tui-checkbox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/tui-checkbox/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/tui-checkbox/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/ci.yml)

A customizable checkbox widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.

## Demo
![checkbox](https://github.com/user-attachments/assets/51e2f5f7-4546-4809-a50a-e754f9ecd866)

## Features

- ☑️ Simple checkbox with label
- 🎨 Customizable styling for checkbox and label separately
- 🔤 Custom symbols (unicode, emoji, ASCII)
- 📦 Optional block wrapper
- ⚡ Zero-cost abstractions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tui-checkbox = "0.2.0"
ratatui = "0.29"
```

Or install with cargo:

```bash
cargo add tui-checkbox
```

## Quick Start

```rust
use ratatui::style::{Color, Style, Modifier};
use tui_checkbox::Checkbox;

// Basic checkbox
let checkbox = Checkbox::new("Enable feature", true);

// Styled checkbox
let checkbox = Checkbox::new("Enable feature", true)
    .style(Style::default().fg(Color::White))
    .checkbox_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    .label_style(Style::default().fg(Color::Gray));

// Custom symbols
let checkbox = Checkbox::new("Task", false)
    .checked_symbol("✅ ")
    .unchecked_symbol("⬜ ");

// With a block
use ratatui::widgets::Block;

let checkbox = Checkbox::new("Accept terms", false)
    .block(Block::bordered().title("Settings"));
```

## Examples

Run the included example with two modes:

```bash
cargo run --example checkbox
```

### Interactive Mode (Default)

Navigate and toggle checkboxes with your keyboard:
- **↑/↓** or **k/j** - Navigate between checkboxes
- **Space** - Toggle checkbox state
- **Tab** - Switch to API Showcase mode
- **q** or **Esc** - Quit

The interactive mode demonstrates:
- Basic checkbox with default styling
- Styled checkboxes with colors (Info, Warning, Error)
- Emoji and Unicode symbols (✅, ●, ◆)
- Custom ASCII symbols ([X], [*], [+]/[-], (X)/(O))

### API Showcase Mode

Press **Tab** to switch to the API showcase view, which displays all available features:
- All core API methods (`new()`, `default()`, `label()`, `checked()`)
- All styling methods (`style()`, `checkbox_style()`, `label_style()`)
- Symbol customization (`checked_symbol()`, `unchecked_symbol()`)
- All predefined symbols from the `symbols` module
- Advanced usage (blocks, Styled trait, method chaining)

## Styling

The widget supports multiple styling options:

- **`style()`**: Sets the base style for the entire widget
- **`checkbox_style()`**: Sets the style specifically for the checkbox symbol
- **`label_style()`**: Sets the style specifically for the label text

Styles are applied in order: base style, then specific styles override it.

## Custom Symbols

The widget comes with default Unicode checkbox symbols (☐ and ☑), but you can use any symbols:

```rust
// Emoji style
Checkbox::new("Task", true)
    .checked_symbol("✅ ")
    .unchecked_symbol("⬜ ");

// ASCII style
Checkbox::new("Task", false)
    .checked_symbol("[X]")
    .unchecked_symbol("[ ]");

// Circle style
Checkbox::new("Task", true)
    .checked_symbol("● ")
    .unchecked_symbol("○ ");
```

## Predefined Symbols

The `symbols` module provides some common checkbox symbols:

- `symbols::CHECKED` - ☑
- `symbols::UNCHECKED` - ☐
- `symbols::CHECKED_X` - [X]
- `symbols::UNCHECKED_SPACE` - [ ]
- `symbols::CHECKED_ASTERISK` - [*]
- `symbols::CHECKED_PLUS` - [+]
- `symbols::UNCHECKED_MINUS` - [-]
- `symbols::CHECKED_PARENTHESIS_X` - (X)
- `symbols::UNCHECKED_PARENTHESIS_O` - (O)

## Development

### Prerequisites

- Rust 1.74.0 or later
- [just](https://github.com/casey/just) - command runner (optional)
- [git-cliff](https://github.com/orhun/git-cliff) - changelog generator (optional)

Install tools:

```bash
just install-tools
```

### Common Tasks

```bash
# Run example
just run

# Run tests
just test

# Format and lint
just fmt
just clippy

# Check all
just check-all

# Generate demo GIF (requires VHS)
just vhs

# Bump version
just bump 0.2.0
```

See all available commands:

```bash
just --list
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Acknowledgments

This widget was created for the [Ratatui](https://github.com/ratatui/ratatui) ecosystem.

Special thanks to the Ratatui team for creating such an amazing TUI framework.
