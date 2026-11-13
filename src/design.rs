//! Material, Dodai and Neo-brutalism symbols (the Cyberpunk one lives in [`crate::cyber`]).
//!
//! Like the cyber slabs these use block elements only. Inline symbols are one row by three
//! cells, blocks five by three; both lay out exactly like the cyber ones.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::cyber::{CyberPalette, Design, Mark, State, mix, slab};

/// Fill, mark character and mark style of a symbol.
pub(crate) type Face = (Color, char, Style);

/// The symbol rows of `design` (never [`Design::Cyberpunk`], which draws its own slab).
pub(crate) fn body(
    design: Design,
    block: bool,
    pal: &CyberPalette,
    state: State,
    face: Face,
) -> Vec<Line<'static>> {
    match design {
        Design::Cyberpunk | Design::Material => material(block, pal, state, face),
        Design::Dodai => dodai(block, pal, state, face),
        Design::Neobrutalism => neobrutalism(block, pal, state, face),
    }
}

/// Outline colour of a box that is not filled: dim when disabled, primary when focused.
fn outline(pal: &CyberPalette, state: State) -> Color {
    if state.disabled {
        pal.surface
    } else if state.focused {
        pal.primary
    } else {
        pal.muted
    }
}

/// Material 3: a rounded outline while off, a tonal-filled box with a tick while on.
fn material(
    block: bool,
    pal: &CyberPalette,
    state: State,
    (fill_c, mark, mark_style): Face,
) -> Vec<Line<'static>> {
    let solid = state.mark != Mark::Unchecked;
    if solid && block {
        return slab([true; 4], fill_c, mark, mark_style);
    }
    if solid {
        let cap = Style::new().fg(fill_c);
        return vec![Line::from(vec![
            Span::styled("▐", cap),
            Span::styled(mark.to_string(), mark_style),
            Span::styled("▌", cap),
        ])];
    }
    let edge = Style::new().fg(outline(pal, state));
    if block {
        return vec![
            Line::from(Span::styled("╭───╮", edge)),
            Line::from(vec![
                Span::styled("│", edge),
                Span::raw("   "),
                Span::styled("│", edge),
            ]),
            Line::from(Span::styled("╰───╯", edge)),
        ];
    }
    vec![Line::from(vec![
        Span::styled("▐", edge),
        Span::raw(" "),
        Span::styled("▌", edge),
    ])]
}

/// The gradient ends of a Dodai symbol: primary to secondary when on, flat glass when off.
fn gradient(pal: &CyberPalette, state: State, fill_c: Color) -> (Color, Color) {
    if state.disabled || state.mark == Mark::Unchecked {
        return (fill_c, fill_c);
    }
    match state.mark {
        Mark::Mixed => (pal.secondary, pal.primary),
        _ => (pal.primary, pal.secondary),
    }
}

/// Dodai: rounded glass; when on, a gradient wash from one side to the other.
fn dodai(
    block: bool,
    pal: &CyberPalette,
    state: State,
    (fill_c, mark, mark_style): Face,
) -> Vec<Line<'static>> {
    let (c0, c1) = gradient(pal, state, fill_c);
    let at = |x: u16, w: u16| mix(c0, c1, f64::from(x) / f64::from(w.saturating_sub(1).max(1)));
    if !block {
        return vec![Line::from(vec![
            Span::styled("▐", Style::new().fg(c0)),
            Span::styled(mark.to_string(), mark_style.bg(mix(c0, c1, 0.5))),
            Span::styled("▌", Style::new().fg(c1)),
        ])];
    }
    // Rounded generously: the top and bottom rows are half rows, the corners single quadrants.
    let cap = |glyph: &'static str, x: u16| Span::styled(glyph, Style::new().fg(at(x, 5)));
    let row = |glyph: &'static str| {
        let mut spans = vec![cap(if glyph == "▄" { "▗" } else { "▝" }, 0)];
        spans.extend((1..4).map(|x| cap(glyph, x)));
        spans.push(cap(if glyph == "▄" { "▖" } else { "▘" }, 4));
        Line::from(spans)
    };
    let wash = |x: u16| Style::new().bg(at(x, 5));
    let mid = Line::from(vec![
        Span::styled(" ", wash(0)),
        Span::styled(" ", wash(1)),
        Span::styled(mark.to_string(), mark_style.bg(at(2, 5))),
        Span::styled(" ", wash(3)),
        Span::styled(" ", wash(4)),
    ]);
    vec![row("▄"), mid, row("▀")]
}

/// Neo-brutalism: a heavy half-cell border around a flat fill.
fn neobrutalism(
    block: bool,
    pal: &CyberPalette,
    state: State,
    (fill_c, mark, mark_style): Face,
) -> Vec<Line<'static>> {
    let edge = Style::new()
        .fg(if state.disabled {
            pal.surface
        } else {
            pal.muted
        })
        .bg(fill_c);
    let flat = Style::new().bg(fill_c);
    let mark_span = Span::styled(mark.to_string(), mark_style);
    if !block {
        return vec![Line::from(vec![
            Span::styled("▌", edge),
            mark_span,
            Span::styled("▐", edge),
        ])];
    }
    vec![
        Line::from(vec![
            Span::styled("▛", edge),
            Span::styled("▀▀▀", edge),
            Span::styled("▜", edge),
        ]),
        Line::from(vec![
            Span::styled("▌", edge),
            Span::styled(" ", flat),
            mark_span,
            Span::styled(" ", flat),
            Span::styled("▐", edge),
        ]),
        Line::from(vec![
            Span::styled("▙", edge),
            Span::styled("▄▄▄", edge),
            Span::styled("▟", edge),
        ]),
    ]
}
