# tui-checkbox

[![Crates.io](https://img.shields.io/crates/v/tui-checkbox)](https://crates.io/crates/tui-checkbox)
[![tui-checkbox installs](https://img.shields.io/crates/d/tui-checkbox?label=tui-checkbox%20installs&logo=rust&color=orange)](https://crates.io/crates/tui-checkbox)
[![Documentation](https://docs.rs/tui-checkbox/badge.svg)](https://docs.rs/tui-checkbox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/tui-checkbox/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/tui-checkbox/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/ci.yml)
[![Audit](https://github.com/sorinirimies/tui-checkbox/actions/workflows/security.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/security.yml)
[![Fuzz](https://github.com/sorinirimies/tui-checkbox/actions/workflows/fuzz.yml/badge.svg)](https://github.com/sorinirimies/tui-checkbox/actions/workflows/fuzz.yml)

A customizable checkbox widget for [Ratatui](https://github.com/ratatui/ratatui) TUI applications.

## Preview
![checkbox](https://github.com/user-attachments/assets/51e2f5f7-4546-4809-a50a-e754f9ecd866)

## Features

- ☑️ Simple checkbox with label, plus an **indeterminate** (partly checked) state
- 🎯 **Focused** and **disabled** states with their own styles
- 🎨 Customizable styling for checkbox and label separately
- 🔤 Custom symbols (unicode, emoji, ASCII) and presets in `symbols`
- 🌆 **Cyberpunk look** - chamfered slabs, hard shadows, readouts, glow, glitch & decrypt - **optional**
- 📐 `width()` / `height()` for sizing layout constraints
- 📦 Block wrapper - **optional**
- 📍 Label positioning (right, left, top, bottom) - **optional**
- ↔️ Horizontal & vertical alignment - **optional**
- 📏 Width constraints (min/max) - **optional**
- 📝 Text wrapping for long labels - **optional**
- 🌈 Shared theming via [`tui-nuances`](https://github.com/sorinirimies/tui-nuances) - **optional** (`theme` feature)
- ⚡ Zero-cost abstractions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tui-checkbox = "0.5"
ratatui = "0.30"
```

Or install with cargo:

```bash
cargo add tui-checkbox
```

See the latest version and full metadata on [crates.io](https://crates.io/crates/tui-checkbox).

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

// States: partly checked, focused, disabled
let select_all = Checkbox::new("Select all", false).indeterminate(true);
let current = Checkbox::new("Option", true).focused(true);
let locked = Checkbox::new("Unavailable", false).disabled(true);

// Size a layout to fit
let constraint = ratatui::layout::Constraint::Length(checkbox.width());
```

## Cyberpunk Look

![Cyberpunk Demo](examples/vhs/checkbox_cyberpunk.gif)

`Checkbox::cyber` swaps the symbol for a filled slab with cut ("chamfered") corners, in the
shape language of the dodai-ui cyberpunk design system:

```text
▐✓◤  ARM SYSTEM  //ONLINE      checked: a neon slab, bottom-right corner cut
▐ ◤  ARM SYSTEM  //OFFLINE     unchecked: a dark slab
▐−◤  ARM SYSTEM  //PARTIAL     indeterminate: the secondary colour
◢✓◤  ARM SYSTEM◤               focused: the label turns into a bar with a cut tip

◢   ◣                          CyberSize::Block (5×3), Tab chamfer,
  ✓  ▌  ARM SYSTEM              with a hard offset shadow
     ▌  //ONLINE
 ▀▀▀▀▘
```

```rust
use tui_checkbox::Checkbox;
use tui_checkbox::cyber::{Chamfer, Cyber, CyberPalette, CyberSize};

let checkbox = Checkbox::new("Arm system", true)
    .focused(true)
    .cyber(
        Cyber::new()
            .chamfer(Chamfer::Diagonal)    // Slab, Diagonal, AntiDiagonal, Tab, Bracket, Sharp
            .size(CyberSize::Block)        // Inline (1 row, default) or Block (3 rows)
            .shadow(true)                  // hard offset shadow
            .readout(true)                 // //ONLINE · //OFFLINE · //PARTIAL
            .glow(true)                    // tinted bar behind a checked label
            .palette(CyberPalette::NEON),  // CYBERPUNK (default), NEON, ARASAKA, SYNTHWAVE
    );
```

| Option | What it does |
|---|---|
| `chamfer` | Which corners are cut: `Slab` (bottom-right), `Diagonal`, `AntiDiagonal`, `Tab` (top), `Bracket` (left), `Sharp` |
| `size` | `Inline` (`▐✓◤`, one row) or `Block` (5×3 slab, label on the middle row) |
| `shadow` | Hard offset shadow below (and right of) the slab; adds a row and a column |
| `uppercase` | Uppercases the label (default `true`) |
| `readout` / `readout_texts` | A status readout after the label (block: on the row below) |
| `glow` | Tints the label background while checked or partly checked (RGB palettes) |
| `glitch(Some(frame))` | Glitches the mark and two label cells for this frame; deterministic |
| `decrypt(Some(percent))` | Scrambles the label and reveals it left to right as `percent` goes 0 → 100 |
| `palette` | Colours; otherwise the theme's (`CyberPalette::from_theme`), else `CYBERPUNK` |

`glitch` and `decrypt` are pure functions of the frame / percent you pass, so the app owns
the animation: after a toggle, count a frame up on every tick for a few hundred milliseconds
and pass it in (see `examples/checkbox_cyberpunk.rs`). `style` and `label_style` still apply;
`checkbox_style` and the symbols are replaced by the slab.

```bash
cargo run --example checkbox_cyberpunk
```

Keys: `↑/↓` move, `Space` toggle (glitch + decrypt), `c` chamfer, `p` palette, `s` size,
`h` shadow, `g` glow, `q` quit.

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

`min_width` widens the box that alignment places, so a centered checkbox with a short label
lines up with wider ones. Wrapping uses display widths (wide CJK and emoji characters count as
two cells) and breaks words longer than a line.

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
- **`focus_style()`**: Patched over symbol and label while `focused(true)` (default: bold)
- **`disabled_style()`**: Patched over symbol and label while `disabled(true)` (default: dim)

Styles are applied in order: base style, then specific styles, then focus, then disabled.

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
- `symbols::UNCHECKED_PARENTHESIS_SPACE` - ( )
- `symbols::INDETERMINATE` - ⊟ (the default indeterminate symbol)
- `symbols::INDETERMINATE_TILDE` - [~]
- `symbols::ROUND_CHECKED` / `symbols::ROUND_UNCHECKED` - ◉ / ○
- `symbols::SQUARE_CHECKED` / `symbols::SQUARE_UNCHECKED` - ■ / □
- `symbols::CHECK` / `symbols::CROSS` - ✓ / ✗

## Theming

`tui-checkbox` uses the shared [`tui-nuances`](https://github.com/sorinirimies/tui-nuances) `Theme`,
so all your TUI components can share one look. The `theme` feature is **on by default**; to drop
the `tui-nuances` dependency, turn default features off:

```toml
[dependencies]
tui-checkbox = { version = "0.5", default-features = false }
```

Without a theme the default look is unchanged.

```rust
use tui_checkbox::{Checkbox, Theme, Themed};

let theme = Theme::by_name("tokyo-night").unwrap_or_default();
let cb = Checkbox::new("Dark mode", true).theme(&theme); // or .apply_theme(&theme)
```

Mapping: base = `theme.style()`, block border = `border` (`border_focused` while focused),
checked symbol = `primary`, unchecked symbol = `muted`, indeterminate symbol = `secondary`,
label = `foreground`, focus = `border_focused` + bold, disabled = `disabled`. The cyber look
derives its palette from the theme (`CyberPalette::from_theme`) unless one is set.
**Precedence:** explicit `style` / `checkbox_style` / `label_style` / `focus_style` /
`disabled_style` always win over theme defaults, regardless of call order.

Pass your app's theme with `ThemeProvider` (`Checkbox::new(..).themed(&app_theme)`) or
`impl From<&AppTheme> for tui_nuances::Theme` and `.theme(&app_theme)`.
The `checkbox` example cycles themes with `t` / `T` when run with `--features theme`.

## Development

### Prerequisites

- Rust 1.88 or later
- [just](https://github.com/casey/just) - command runner (optional)
- [git-cliff](https://github.com/orhun/git-cliff) - changelog generator (optional)

Install tools:

```bash
just install-tools
```

### Common Tasks

```bash
# Run example
just example checkbox

# Run tests
just test

# Format and lint
just fmt
just clippy

# Check all (format, clippy, tests)
just check-all

# Release check (check-all plus a release build and the docs)
just check-release

# Generate the demo GIFs (requires VHS)
just vhs-all
```

### Release Workflow

```bash
# Release to GitHub only
just release 0.2.0

# Release to Gitea only (if configured)
just release-gitea-starscream 0.2.0

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
# Add the missing remotes (GitHub and Gitea)
just setup-remotes

# Push to both GitHub and Gitea
just push-all

# Force-sync Gitea with this checkout
just sync-gitea-starscream

# Show configured remotes
just remotes
```

See all available commands:

```bash
just --list
```

## Fuzzing

Fuzz targets live in `fuzz/` (needs nightly and `cargo install cargo-fuzz`):

```bash
cargo +nightly fuzz list
cargo +nightly fuzz run render_layout
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`
3. **Make your changes** with clear commit messages
4. **Submit a pull request** to the main repository

All contributions must be made through GitHub pull requests from your fork. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Acknowledgments

This widget was created for the [Ratatui](https://github.com/ratatui/ratatui) ecosystem.

Special thanks to the Ratatui team for creating such an amazing TUI framework.
