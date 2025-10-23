//! # Checkbox Widget Example
//!
//! This example demonstrates the `tui-checkbox` widget with two modes:
//!
//! ## Interactive Mode (Default)
//! - Navigate with ↑/↓ or k/j
//! - Toggle checkboxes with Space
//! - Press Tab to switch to API Showcase
//! - Press q or Esc to quit
//!
//! ## API Showcase Mode
//! - View all public API methods and symbols
//! - Press Tab to return to Interactive mode
//!
//! Run with: cargo run --example checkbox

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use tui_checkbox::{symbols, Checkbox};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Interactive,
    ApiShowcase,
}

struct App {
    // Sections: Basic (1), Styled (3), Emoji (3), Custom (4) = 11 total
    checkboxes: Vec<bool>,
    selected: usize,
    mode: ViewMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            checkboxes: vec![
                true,  // Basic: Notifications
                true,  // Styled: Info
                false, // Styled: Warning
                false, // Styled: Error
                true,  // Emoji: Check mark
                false, // Emoji: Circle
                true,  // Emoji: Diamond
                true,  // Custom: ASCII [X]
                false, // Custom: Asterisk [ ]
                true,  // Custom: Plus [+] / Minus [-]
                false, // Custom: X/O style (O)
            ],
            selected: 0,
            mode: ViewMode::Interactive,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::default();
    let terminal = ratatui::init();
    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Tab => {
                    app.mode = match app.mode {
                        ViewMode::Interactive => ViewMode::ApiShowcase,
                        ViewMode::ApiShowcase => ViewMode::Interactive,
                    };
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.mode == ViewMode::Interactive && app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.mode == ViewMode::Interactive && app.selected < app.checkboxes.len() - 1
                    {
                        app.selected += 1;
                    }
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if app.mode == ViewMode::Interactive {
                        app.checkboxes[app.selected] = !app.checkboxes[app.selected];
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    match app.mode {
        ViewMode::Interactive => render_interactive(frame, app),
        ViewMode::ApiShowcase => render_api_showcase(frame, app),
    }
}

fn render_interactive(frame: &mut Frame, app: &App) {
    let main_layout = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // Header
    render_header(frame, main_layout[0], app);

    // Content area
    let content_layout = Layout::vertical([
        Constraint::Length(7), // Basic
        Constraint::Length(8), // Styled
        Constraint::Length(8), // Emoji
        Constraint::Length(9), // Custom
    ])
    .spacing(1)
    .split(main_layout[1]);

    render_basic_section(frame, content_layout[0], app);
    render_styled_section(frame, content_layout[1], app);
    render_emoji_section(frame, content_layout[2], app);
    render_custom_section(frame, content_layout[3], app);

    // Footer
    render_footer(frame, main_layout[2], app);
}

fn render_api_showcase(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Length(3), // Core API
        Constraint::Length(3), // Styling API
        Constraint::Length(3), // Symbol Customization
        Constraint::Length(3), // Predefined Symbols (Unicode)
        Constraint::Length(3), // Predefined Symbols (ASCII)
        Constraint::Length(3), // Predefined Symbols (Special)
        Constraint::Length(3), // Advanced Usage
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    render_header(frame, layout[0], app);
    render_showcase_core_api(frame, layout[1]);
    render_showcase_styling_api(frame, layout[2]);
    render_showcase_symbol_customization(frame, layout[3]);
    render_showcase_predefined_unicode(frame, layout[4]);
    render_showcase_predefined_ascii(frame, layout[5]);
    render_showcase_predefined_special(frame, layout[6]);
    render_showcase_advanced_usage(frame, layout[7]);
    render_footer(frame, layout[8], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(match app.mode {
            ViewMode::Interactive => " tui-checkbox Demo - Interactive Mode ",
            ViewMode::ApiShowcase => " tui-checkbox Demo - API Showcase ",
        })
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));

    let text = Paragraph::new("A customizable checkbox widget for Ratatui")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(text.block(block), area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let text = match app.mode {
        ViewMode::Interactive => Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Navigate  "),
            Span::styled("Space", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Toggle  "),
            Span::styled("Tab", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" API Showcase  "),
            Span::styled("q", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Quit"),
        ]),
        ViewMode::ApiShowcase => Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Back to Interactive  "),
            Span::styled("q", Style::default().fg(Color::Cyan).bold()),
            Span::raw(" Quit"),
        ]),
    };

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph.block(block), area);
}

fn render_basic_section(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(" Basic Checkbox ")
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let checkbox_area = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .split(inner)[1];

    let is_selected = app.selected == 0;
    let checkbox = Checkbox::new("Enable notifications", app.checkboxes[0]).style(if is_selected {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    });

    frame.render_widget(checkbox, checkbox_area);
}

fn render_styled_section(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(" Styled Checkboxes ")
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .spacing(1)
    .split(inner);

    // Info
    let is_selected = app.selected == 1;
    let checkbox = Checkbox::new("Info state", app.checkboxes[1])
        .checkbox_style(Style::default().fg(Color::Blue))
        .label_style(Style::default().fg(Color::Blue))
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[0]);

    // Warning
    let is_selected = app.selected == 2;
    let checkbox = Checkbox::new("Warning state", app.checkboxes[2])
        .checkbox_style(Style::default().fg(Color::Yellow))
        .label_style(Style::default().fg(Color::Yellow))
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[1]);

    // Error
    let is_selected = app.selected == 3;
    let checkbox = Checkbox::new("Error state", app.checkboxes[3])
        .checkbox_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::Red))
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[2]);
}

fn render_emoji_section(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(" Emoji & Unicode Symbols ")
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .spacing(1)
    .split(inner);

    // Check mark emoji (using custom emoji symbols)
    let is_selected = app.selected == 4;
    let checkbox = Checkbox::new("Check mark style", app.checkboxes[4])
        .checked_symbol("✅ ")
        .unchecked_symbol("⬜ ")
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[0]);

    // Circle (using custom unicode symbols)
    let is_selected = app.selected == 5;
    let checkbox = Checkbox::new("Circle style", app.checkboxes[5])
        .checked_symbol("● ")
        .unchecked_symbol("○ ")
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[1]);

    // Diamond (using custom unicode symbols)
    let is_selected = app.selected == 6;
    let checkbox = Checkbox::new("Diamond style", app.checkboxes[6])
        .checked_symbol("◆ ")
        .unchecked_symbol("◇ ")
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[2]);
}

fn render_custom_section(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered()
        .title(" Custom ASCII Symbols ")
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .spacing(1)
    .split(inner);

    // ASCII [X] - using predefined symbols
    let is_selected = app.selected == 7;
    let checkbox = Checkbox::new("ASCII style", app.checkboxes[7])
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE)
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[0]);

    // Asterisk - using predefined symbols
    let is_selected = app.selected == 8;
    let checkbox = Checkbox::new("Asterisk", app.checkboxes[8])
        .checked_symbol(symbols::CHECKED_ASTERISK)
        .unchecked_symbol(symbols::UNCHECKED_SPACE)
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[1]);

    // Plus/Minus - using predefined symbols
    let is_selected = app.selected == 9;
    let label = if app.checkboxes[9] {
        "Plus sign"
    } else {
        "Minus sign"
    };
    let checkbox = Checkbox::new(label, app.checkboxes[9])
        .checked_symbol(symbols::CHECKED_PLUS)
        .unchecked_symbol(symbols::UNCHECKED_MINUS)
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[2]);

    // X/O style - using predefined symbols
    let is_selected = app.selected == 10;
    let checkbox = Checkbox::new("X/O style", app.checkboxes[10])
        .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
        .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O)
        .style(if is_selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_widget(checkbox, items_layout[3]);
}

// ============================================================================
// API Showcase Rendering Functions
// ============================================================================

fn render_showcase_core_api(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title(" Core API: new(), default(), label(), checked() ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ])
    .split(inner);

    let checkbox = Checkbox::new("new()", true);
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::default().label("default()");
    frame.render_widget(checkbox, layout[1]);

    let checkbox = Checkbox::default().label("label()").checked(true);
    frame.render_widget(checkbox, layout[2]);

    let checkbox = Checkbox::new("checked()", false).checked(true);
    frame.render_widget(checkbox, layout[3]);

    let checkbox = Checkbox::new("unchecked()", true).checked(false);
    frame.render_widget(checkbox, layout[4]);
}

fn render_showcase_styling_api(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title(" Styling: style(), checkbox_style(), label_style() ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    let checkbox = Checkbox::new("style()", true)
        .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray));
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::new("checkbox_style()", true).checkbox_style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(checkbox, layout[1]);

    let checkbox = Checkbox::new("label_style()", false).label_style(
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::ITALIC),
    );
    frame.render_widget(checkbox, layout[2]);
}

fn render_showcase_symbol_customization(frame: &mut Frame, area: Rect) {
    let block =
        Block::bordered().title(" Symbol Customization: checked_symbol(), unchecked_symbol() ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    let checkbox = Checkbox::new("Custom checked", true).checked_symbol("✓ ");
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::new("Custom unchecked", false).unchecked_symbol("✗ ");
    frame.render_widget(checkbox, layout[1]);

    let checkbox = Checkbox::new("Both custom", true)
        .checked_symbol("✅ ")
        .unchecked_symbol("⬜ ");
    frame.render_widget(checkbox, layout[2]);
}

fn render_showcase_predefined_unicode(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title(" Predefined Symbols: CHECKED (☑) / UNCHECKED (☐) ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(inner);

    let checkbox = Checkbox::new("symbols::CHECKED", true)
        .checked_symbol(symbols::CHECKED)
        .unchecked_symbol(symbols::UNCHECKED);
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::new("symbols::UNCHECKED", false)
        .checked_symbol(symbols::CHECKED)
        .unchecked_symbol(symbols::UNCHECKED);
    frame.render_widget(checkbox, layout[1]);
}

fn render_showcase_predefined_ascii(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Predefined ASCII: CHECKED_X [X] / UNCHECKED_SPACE [ ] / CHECKED_ASTERISK [*] ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    let checkbox = Checkbox::new("CHECKED_X", true)
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::new("UNCHECKED_SPACE", false)
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
    frame.render_widget(checkbox, layout[1]);

    let checkbox = Checkbox::new("CHECKED_ASTERISK", true)
        .checked_symbol(symbols::CHECKED_ASTERISK)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
    frame.render_widget(checkbox, layout[2]);
}

fn render_showcase_predefined_special(frame: &mut Frame, area: Rect) {
    let block =
        Block::bordered().title(" Special Symbols: PLUS [+] / MINUS [-] / PARENTHESIS (X)/(O) ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    let checkbox = Checkbox::new("CHECKED_PLUS", true)
        .checked_symbol(symbols::CHECKED_PLUS)
        .unchecked_symbol(symbols::UNCHECKED_MINUS);
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::new("UNCHECKED_MINUS", false)
        .checked_symbol(symbols::CHECKED_PLUS)
        .unchecked_symbol(symbols::UNCHECKED_MINUS);
    frame.render_widget(checkbox, layout[1]);

    let checkbox = Checkbox::new("PARENTHESIS_X", true)
        .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
        .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O);
    frame.render_widget(checkbox, layout[2]);
}

fn render_showcase_advanced_usage(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title(" Advanced: block(), Styled trait, method chaining ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .split(inner);

    let checkbox = Checkbox::new("With Block", true).block(
        Block::bordered()
            .title("Inner")
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(checkbox, layout[0]);

    let checkbox = Checkbox::new("Styled trait", false).cyan().bold();
    frame.render_widget(checkbox, layout[1]);

    let checkbox = Checkbox::default()
        .label("Chained")
        .checked(true)
        .style(Color::White)
        .checkbox_style(Style::default().fg(Color::Green))
        .label_style(Style::default().fg(Color::Gray));
    frame.render_widget(checkbox, layout[2]);
}

// ============================================================================</parameter>
// Examples for all public API methods
// ============================================================================
//
// The following functions demonstrate the usage of all public methods
// and symbols available in the tui-checkbox crate.
//
// ## Table of Contents
//
// ### Core Methods:
// - `example_new()` - Checkbox::new() - Create with label and checked state
// - `example_default()` - Checkbox::default() - Create with default values
// - `example_label()` - Checkbox::label() - Set checkbox label
// - `example_checked()` - Checkbox::checked() - Set checked state
// - `example_block()` - Checkbox::block() - Wrap in a block
//
// ### Styling Methods:
// - `example_style()` - Checkbox::style() - Set base style
// - `example_checkbox_style()` - Checkbox::checkbox_style() - Style checkbox symbol
// - `example_label_style()` - Checkbox::label_style() - Style label text
// - `example_styled_trait()` - Using Styled trait from ratatui
//
// ### Symbol Methods:
// - `example_checked_symbol()` - Checkbox::checked_symbol() - Custom checked symbol
// - `example_unchecked_symbol()` - Checkbox::unchecked_symbol() - Custom unchecked symbol
//
// ### Predefined Symbols:
// - `example_symbols_checked_unchecked()` - symbols::CHECKED and symbols::UNCHECKED (☑/☐)
// - `example_symbols_x_space()` - symbols::CHECKED_X and symbols::UNCHECKED_SPACE ([X]/[ ])
// - `example_symbols_asterisk()` - symbols::CHECKED_ASTERISK ([*])
// - `example_symbols_plus_minus()` - symbols::CHECKED_PLUS and symbols::UNCHECKED_MINUS ([+]/[-])
// - `example_symbols_parenthesis()` - symbols::CHECKED_PARENTHESIS_X and symbols::UNCHECKED_PARENTHESIS_O ((X)/(O))
//
// ### Practical Examples:
// - `example_combined_styling()` - Combining multiple styling methods
// - `example_render_widget()` - Rendering with Widget trait
// - `example_method_chaining()` - Fluent API method chaining
// - `example_dynamic_state()` - Dynamic state updates
// - `example_contextual_symbols()` - Different symbols for different contexts
// - `example_emoji_symbols()` - Using emoji symbols
// - `example_checkbox_list()` - List of checkboxes in an application
//

/// Example: Checkbox::new() - Creates a new checkbox with label and checked state
#[allow(dead_code)]
fn example_new() {
    use tui_checkbox::Checkbox;

    // Basic checkbox
    let _checkbox = Checkbox::new("Enable feature", true);

    // With styled label using Stylize trait
    let _checkbox = Checkbox::new("Enable feature".blue(), false);
}

/// Example: Checkbox::default() - Creates a checkbox with default values
#[allow(dead_code)]
fn example_default() {
    use tui_checkbox::Checkbox;

    // Default checkbox: unchecked, empty label, unicode symbols
    let _checkbox = Checkbox::default();
}

/// Example: Checkbox::label() - Sets the checkbox label
#[allow(dead_code)]
fn example_label() {
    use tui_checkbox::Checkbox;

    // Simple string label
    let _checkbox = Checkbox::default().label("My checkbox");

    // Styled label
    let _checkbox = Checkbox::default().label("My checkbox".cyan());
}

/// Example: Checkbox::checked() - Sets the checked state
#[allow(dead_code)]
fn example_checked() {
    use tui_checkbox::Checkbox;

    // Checked checkbox
    let _checkbox = Checkbox::default().checked(true);

    // Unchecked checkbox
    let _checkbox = Checkbox::default().checked(false);
}

/// Example: Checkbox::block() - Wraps checkbox in a block
#[allow(dead_code)]
fn example_block() {
    use ratatui::widgets::Block;
    use tui_checkbox::Checkbox;

    // With bordered block
    let _checkbox = Checkbox::new("Option", false).block(Block::bordered().title("Settings"));

    // With styled block
    let _checkbox = Checkbox::new("Option", true).block(
        Block::bordered()
            .title("Configuration")
            .border_style(Style::default().fg(Color::Cyan)),
    );
}

/// Example: Checkbox::style() - Sets the base style for the entire widget
#[allow(dead_code)]
fn example_style() {
    use ratatui::style::{Color, Style};
    use tui_checkbox::Checkbox;

    // Using Style
    let _checkbox =
        Checkbox::new("Option", false).style(Style::default().fg(Color::White).bg(Color::DarkGray));

    // Using Color (implements Into<Style>)
    let _checkbox = Checkbox::new("Option", true).style(Color::Green);
}

/// Example: Checkbox::checkbox_style() - Sets style specifically for the checkbox symbol
#[allow(dead_code)]
fn example_checkbox_style() {
    use ratatui::style::{Color, Modifier, Style};
    use tui_checkbox::Checkbox;

    // Green checkbox symbol
    let _checkbox = Checkbox::new("Option", true).checkbox_style(Style::default().fg(Color::Green));

    // Bold red checkbox symbol
    let _checkbox = Checkbox::new("Option", false)
        .checkbox_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
}

/// Example: Checkbox::label_style() - Sets style specifically for the label text
#[allow(dead_code)]
fn example_label_style() {
    use ratatui::style::{Color, Modifier, Style};
    use tui_checkbox::Checkbox;

    // Gray label
    let _checkbox = Checkbox::new("Option", false).label_style(Style::default().fg(Color::Gray));

    // Italic blue label
    let _checkbox = Checkbox::new("Option", true).label_style(
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::ITALIC),
    );
}

/// Example: Checkbox::checked_symbol() - Sets the symbol for checked state
#[allow(dead_code)]
fn example_checked_symbol() {
    use tui_checkbox::Checkbox;

    // ASCII X symbol
    let _checkbox = Checkbox::new("Option", true).checked_symbol("[X]");

    // Emoji symbol
    let _checkbox = Checkbox::new("Option", true).checked_symbol("✅ ");

    // Custom unicode symbol
    let _checkbox = Checkbox::new("Option", true).checked_symbol("● ");
}

/// Example: Checkbox::unchecked_symbol() - Sets the symbol for unchecked state
#[allow(dead_code)]
fn example_unchecked_symbol() {
    use tui_checkbox::Checkbox;

    // ASCII space symbol
    let _checkbox = Checkbox::new("Option", false).unchecked_symbol("[ ]");

    // Emoji symbol
    let _checkbox = Checkbox::new("Option", false).unchecked_symbol("⬜ ");

    // Custom unicode symbol
    let _checkbox = Checkbox::new("Option", false).unchecked_symbol("○ ");
}

/// Example: Using predefined symbols::CHECKED and symbols::UNCHECKED
#[allow(dead_code)]
fn example_symbols_checked_unchecked() {
    use tui_checkbox::{symbols, Checkbox};

    // Default unicode symbols (☑ and ☐)
    let _checkbox = Checkbox::new("Option", true)
        .checked_symbol(symbols::CHECKED)
        .unchecked_symbol(symbols::UNCHECKED);
}

/// Example: Using symbols::CHECKED_X and symbols::UNCHECKED_SPACE
#[allow(dead_code)]
fn example_symbols_x_space() {
    use tui_checkbox::{symbols, Checkbox};

    // ASCII X style: [X] and [ ]
    let _checkbox = Checkbox::new("ASCII style", false)
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
}

/// Example: Using symbols::CHECKED_ASTERISK
#[allow(dead_code)]
fn example_symbols_asterisk() {
    use tui_checkbox::{symbols, Checkbox};

    // Asterisk style: [*] and [ ]
    let _checkbox = Checkbox::new("Important", true)
        .checked_symbol(symbols::CHECKED_ASTERISK)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
}

/// Example: Using symbols::CHECKED_PLUS and symbols::UNCHECKED_MINUS
#[allow(dead_code)]
fn example_symbols_plus_minus() {
    use tui_checkbox::{symbols, Checkbox};

    // Plus/minus style: [+] and [-]
    let _checkbox = Checkbox::new("Toggle option", true)
        .checked_symbol(symbols::CHECKED_PLUS)
        .unchecked_symbol(symbols::UNCHECKED_MINUS);
}

/// Example: Using symbols::CHECKED_PARENTHESIS_X and symbols::UNCHECKED_PARENTHESIS_O
#[allow(dead_code)]
fn example_symbols_parenthesis() {
    use tui_checkbox::{symbols, Checkbox};

    // Parenthesis X/O style: (X) and (O)
    let _checkbox = Checkbox::new("X or O", false)
        .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
        .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O);
}

/// Example: Combining multiple styling methods
#[allow(dead_code)]
fn example_combined_styling() {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::widgets::Block;
    use tui_checkbox::{symbols, Checkbox};

    // Complex styled checkbox with all options
    let _checkbox = Checkbox::new("Complete example", true)
        .block(Block::bordered().title("Settings"))
        .style(Style::default().bg(Color::Black))
        .checkbox_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        )
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
}

/// Example: Using Styled trait (from ratatui)
#[allow(dead_code)]
fn example_styled_trait() {
    use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
    use tui_checkbox::Checkbox;

    // Using Stylize trait methods
    let _checkbox = Checkbox::new("Stylized", true).cyan().bold();

    // Using set_style from Styled trait
    let _checkbox = Checkbox::new("Styled", false).set_style(
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::UNDERLINED),
    );
}

/// Example: Rendering checkbox with Widget trait
#[allow(dead_code)]
fn example_render_widget() {
    use ratatui::{layout::Rect, Frame};
    use tui_checkbox::Checkbox;

    fn render_example(frame: &mut Frame, area: Rect) {
        let checkbox = Checkbox::new("Render me", true);

        // Render by value (consumes checkbox)
        frame.render_widget(checkbox, area);
    }

    fn render_example_ref(frame: &mut Frame, area: Rect) {
        let checkbox = Checkbox::new("Render me", true);

        // Render by reference (checkbox can be reused)
        frame.render_widget(&checkbox, area);
        // checkbox is still available here
    }
}

/// Example: Method chaining for fluent API
#[allow(dead_code)]
fn example_method_chaining() {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::Block;
    use tui_checkbox::{symbols, Checkbox};

    // All methods return Self for easy chaining
    let _checkbox = Checkbox::default()
        .label("Chained configuration")
        .checked(true)
        .style(Color::White)
        .checkbox_style(Style::default().fg(Color::Green))
        .label_style(Style::default().fg(Color::Gray))
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE)
        .block(Block::bordered().title("Options"));
}

/// Example: Dynamic checkbox state updates
#[allow(dead_code)]
fn example_dynamic_state() {
    use tui_checkbox::Checkbox;

    let mut is_checked = false;

    // Toggle state
    is_checked = !is_checked;

    // Create checkbox with updated state
    let _checkbox = Checkbox::new("Dynamic state", is_checked);

    // Or rebuild with new checked state
    let _checkbox = Checkbox::new("Toggle me", false).checked(is_checked);
}

/// Example: Using different symbol sets for different contexts
#[allow(dead_code)]
fn example_contextual_symbols() {
    use tui_checkbox::{symbols, Checkbox};

    // For terminal that supports unicode
    let _checkbox = Checkbox::new("Unicode", true)
        .checked_symbol(symbols::CHECKED)
        .unchecked_symbol(symbols::UNCHECKED);

    // For ASCII-only terminals
    let _checkbox = Checkbox::new("ASCII", true)
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);

    // For visual emphasis
    let _checkbox = Checkbox::new("Important", true)
        .checked_symbol(symbols::CHECKED_ASTERISK)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);

    // For mathematical/logical contexts
    let _checkbox = Checkbox::new("Logic", true)
        .checked_symbol(symbols::CHECKED_PLUS)
        .unchecked_symbol(symbols::UNCHECKED_MINUS);
}

/// Example: Emoji symbols for visual appeal
#[allow(dead_code)]
fn example_emoji_symbols() {
    use tui_checkbox::Checkbox;

    // Checkmark emoji
    let _checkbox = Checkbox::new("Task complete", true)
        .checked_symbol("✅ ")
        .unchecked_symbol("⬜ ");

    // Star emoji
    let _checkbox = Checkbox::new("Favorite", true)
        .checked_symbol("⭐ ")
        .unchecked_symbol("☆ ");

    // Heart emoji
    let _checkbox = Checkbox::new("Like", false)
        .checked_symbol("❤️ ")
        .unchecked_symbol("🤍 ");

    // Thumbs up/down
    let _checkbox = Checkbox::new("Approve", true)
        .checked_symbol("👍 ")
        .unchecked_symbol("👎 ");
}

/// Example: List of checkboxes in an application
#[allow(dead_code)]
fn example_checkbox_list() {
    use ratatui::style::{Color, Style};
    use tui_checkbox::Checkbox;

    struct TodoItem {
        label: String,
        completed: bool,
    }

    let todos = [
        TodoItem {
            label: "Buy groceries".to_string(),
            completed: true,
        },
        TodoItem {
            label: "Write code".to_string(),
            completed: false,
        },
        TodoItem {
            label: "Exercise".to_string(),
            completed: false,
        },
    ];

    // Create checkboxes from todo items
    let _checkboxes: Vec<Checkbox> = todos
        .iter()
        .map(|todo| {
            Checkbox::new(todo.label.as_str(), todo.completed).checkbox_style(if todo.completed {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Gray)
            })
        })
        .collect();
}
