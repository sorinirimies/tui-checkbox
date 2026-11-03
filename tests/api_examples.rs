//! Every public tui-checkbox API, one test each: the usage examples are
//! compiled and run, so they can't go stale.

use ratatui::style::{Color, Style, Stylize};

// ============================================================================
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
#[test]
fn example_new() {
    use tui_checkbox::Checkbox;

    // Basic checkbox
    let _checkbox = Checkbox::new("Enable feature", true);

    // With styled label using Stylize trait
    let _checkbox = Checkbox::new("Enable feature".blue(), false);
}

/// Example: Checkbox::default() - Creates a checkbox with default values
#[test]
fn example_default() {
    use tui_checkbox::Checkbox;

    // Default checkbox: unchecked, empty label, unicode symbols
    let _checkbox = Checkbox::default();
}

/// Example: Checkbox::label() - Sets the checkbox label
#[test]
fn example_label() {
    use tui_checkbox::Checkbox;

    // Simple string label
    let _checkbox = Checkbox::default().label("My checkbox");

    // Styled label
    let _checkbox = Checkbox::default().label("My checkbox".cyan());
}

/// Example: Checkbox::checked() - Sets the checked state
#[test]
fn example_checked() {
    use tui_checkbox::Checkbox;

    // Checked checkbox
    let _checkbox = Checkbox::default().checked(true);

    // Unchecked checkbox
    let _checkbox = Checkbox::default().checked(false);
}

/// Example: Checkbox::block() - Wraps checkbox in a block
#[test]
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
#[test]
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
#[test]
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
#[test]
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
#[test]
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
#[test]
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
#[test]
fn example_symbols_checked_unchecked() {
    use tui_checkbox::{Checkbox, symbols};

    // Default unicode symbols (☑ and ☐)
    let _checkbox = Checkbox::new("Option", true)
        .checked_symbol(symbols::CHECKED)
        .unchecked_symbol(symbols::UNCHECKED);
}

/// Example: Using symbols::CHECKED_X and symbols::UNCHECKED_SPACE
#[test]
fn example_symbols_x_space() {
    use tui_checkbox::{Checkbox, symbols};

    // ASCII X style: [X] and [ ]
    let _checkbox = Checkbox::new("ASCII style", false)
        .checked_symbol(symbols::CHECKED_X)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
}

/// Example: Using symbols::CHECKED_ASTERISK
#[test]
fn example_symbols_asterisk() {
    use tui_checkbox::{Checkbox, symbols};

    // Asterisk style: [*] and [ ]
    let _checkbox = Checkbox::new("Important", true)
        .checked_symbol(symbols::CHECKED_ASTERISK)
        .unchecked_symbol(symbols::UNCHECKED_SPACE);
}

/// Example: Using symbols::CHECKED_PLUS and symbols::UNCHECKED_MINUS
#[test]
fn example_symbols_plus_minus() {
    use tui_checkbox::{Checkbox, symbols};

    // Plus/minus style: [+] and [-]
    let _checkbox = Checkbox::new("Toggle option", true)
        .checked_symbol(symbols::CHECKED_PLUS)
        .unchecked_symbol(symbols::UNCHECKED_MINUS);
}

/// Example: Using symbols::CHECKED_PARENTHESIS_X and symbols::UNCHECKED_PARENTHESIS_O
#[test]
fn example_symbols_parenthesis() {
    use tui_checkbox::{Checkbox, symbols};

    // Parenthesis X/O style: (X) and (O)
    let _checkbox = Checkbox::new("X or O", false)
        .checked_symbol(symbols::CHECKED_PARENTHESIS_X)
        .unchecked_symbol(symbols::UNCHECKED_PARENTHESIS_O);
}

/// Example: Combining multiple styling methods
#[test]
fn example_combined_styling() {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::widgets::Block;
    use tui_checkbox::{Checkbox, symbols};

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
#[test]
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
#[test]
fn example_render_widget() {
    use ratatui::{Frame, layout::Rect};
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

    let backend = ratatui::backend::TestBackend::new(20, 1);
    let mut terminal = ratatui::Terminal::new(backend).expect("a test terminal");
    for render in [render_example, render_example_ref] {
        let frame = terminal
            .draw(|frame| render(frame, frame.area()))
            .expect("draw");
        let text: String = frame.buffer.content().iter().map(|c| c.symbol()).collect();
        assert!(text.contains("Render me"), "rendered: {text}");
    }
}

/// Example: Method chaining for fluent API
#[test]
fn example_method_chaining() {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::Block;
    use tui_checkbox::{Checkbox, symbols};

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
#[test]
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
#[test]
fn example_contextual_symbols() {
    use tui_checkbox::{Checkbox, symbols};

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
#[test]
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
#[test]
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
