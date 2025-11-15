# tui-checkbox

[![Crates.io](https://img.shields.io/crates/v/tui-checkbox)](https://crates.io/crates/tui-checkbox)
[![Documentation](https://docs.rs/tui-checkbox/badge.svg)](https://docs.rs/tui-checkbox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/tui-checkbox/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/tui-checkbox/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/ci.yml)

A customizable checkbox widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.

## Preview
![checkbox](https://github.com/user-attachments/assets/51e2f5f7-4546-4809-a50a-e754f9ecd866)

## Features

- ☑️ Simple checkbox with label
- 🎨 Customizable styling for checkbox and label separately
- 🔤 Custom symbols (unicode, emoji, ASCII)
- 📦 Block wrapper - **optional**
- 📍 Label positioning (right, left, top, bottom) - **optional**
- ↔️ Horizontal & vertical alignment - **optional**
- 📏 Width constraints (min/max) - **optional**
- 📝 Text wrapping for long labels - **optional**
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

## Label Positioning

Control where the label appears relative to the checkbox symbol.

> **Note**: All layout features are **optional**! The checkbox works perfectly with sensible defaults. Only use these methods when you need to customize the layout.

```rust
use tui_checkbox::{Checkbox, LabelPosition};

// Simple checkbox - works great with defaults!
Checkbox::new("Enable feature", true);

// Label on the left (optional customization)
Checkbox::new("Enable feature", true)
    .label_position(LabelPosition::Left);

// Label on top (optional customization)
Checkbox::new("Enable feature", false)
    .label_position(LabelPosition::Top);

// Label on bottom (optional customization)
Checkbox::new("Enable feature", false)
    .label_position(LabelPosition::Bottom);
```

**Defaults**: Label on the right (standard checkbox style)

### Alignment (Optional)

Align the checkbox content within its area. Only needed for specific layouts.

```rust
use tui_checkbox::{Checkbox, HorizontalAlignment, VerticalAlignment};

// Horizontal alignment (optional)
Checkbox::new("Centered", true)
    .horizontal_alignment(HorizontalAlignment::Center);

Checkbox::new("Right", true)
    .horizontal_alignment(HorizontalAlignment::Right);

// Vertical alignment (optional)
Checkbox::new("Middle", false)
    .vertical_alignment(VerticalAlignment::Center);

Checkbox::new("Bottom", false)
    .vertical_alignment(VerticalAlignment::Bottom);
```

**Defaults**: Left and top aligned

### Width Constraints & Text Wrapping (Optional)

Set minimum and maximum width constraints, and enable text wrapping when needed.

```rust
// Minimum width (optional)
Checkbox::new("Small label", true)
    .min_width(30);

// Maximum width (optional)
Checkbox::new("This is a very long label that will be constrained", false)
    .max_width(25);

// Text wrapping (optional - for long labels)
Checkbox::new("This is a very long label that demonstrates text wrapping", true)
    .wrap_label(true)
    .max_width(30);
```

**Defaults**: No width constraints, no wrapping

## Layout Examples

### Label Position Example

See all four label positions in action:

```bash
cargo run --example checkbox_label_position
```

This demonstrates: right, left, top, and bottom label positions.

![Label Position Demo](examples/vhs/checkbox_label_position.gif)

### Alignment Example

See horizontal and vertical alignment:

```bash
cargo run --example checkbox_alignment
```

This demonstrates:
- Horizontal alignment: left, center, right
- Vertical alignment: top, center, bottom

![Alignment Demo](examples/vhs/checkbox_alignment_demo.gif)

### Width & Wrapping Example

See width constraints and text wrapping:

```bash
cargo run --example checkbox_width_wrapping
```

This demonstrates:
- Minimum and maximum width constraints
- Text wrapping (enabled vs disabled)
- Combined features

![Width & Wrapping Demo](examples/vhs/checkbox_width_wrapping.gif)

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

# Check all (format, clippy, tests)
just check-all

# Release check (format, clippy, test, build)
just release-check

# Generate demo GIF (requires VHS)
just vhs
```

### Release Workflow

```bash
# Release to GitHub only
just release 0.2.0

# Release to Gitea only (if configured)
just release-gitea 0.2.0

# Release to both GitHub and Gitea
just release-all 0.2.0
```

The release commands automatically:
1. Bump version in Cargo.toml
2. Update Cargo.lock
3. Generate changelog with git-cliff
4. Create git commit and tag
5. Push to the selected remote(s)

### Dual Hosting Commands

If you have a Gitea instance configured:

```bash
# Setup Gitea remote
just setup-gitea git@gitea.yourdomain.com:username/tui-checkbox.git

# Push to both GitHub and Gitea
just push-all

# Sync Gitea with GitHub
just sync-gitea

# Show configured remotes
just remotes
```

See [DUAL_HOSTING.md](DUAL_HOSTING.md) for complete dual hosting setup guide.

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
