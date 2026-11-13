//! Material, Dodai and Neo-brutalism symbols (the Cyberpunk one lives in [`crate::cyber`]).
//!
//! Like the cyber slabs these use block elements only. Inline symbols are one row by three
//! cells, blocks five by three; both lay out exactly like the cyber ones.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::cyber::{CUT, CyberPalette, Design, Mark, State, mix, slab};

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
    // The inside is the surface tinted with the outline colour, so the bars read as a box.
    let inside = mix(pal.surface, outline(pal, state), 0.3);
    vec![Line::from(vec![
        Span::styled("▐", edge),
        Span::styled(" ", Style::new().bg(inside)),
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

/// Dodai: rounded glass; when on, a gradient wash from one side to the other. A block is the
/// same 5×3 body as Material's, with the wash across it.
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
    // The wash is sampled twice per cell: a left half (foreground) and a right half (background)
    // of a `▌`, so the ramp has ten steps over five cells. A flat fill stays plain spaces.
    let flat = c0 == c1;
    let half = |x: u16| {
        let t = |i: u16| mix(c0, c1, f64::from(i) / 9.0);
        if flat {
            Span::styled(" ", Style::new().bg(c0))
        } else {
            Span::styled("▌", Style::new().fg(t(2 * x)).bg(t(2 * x + 1)))
        }
    };
    // Rounded corners: a quarter cell taken out, in the wash colour at that end.
    let corner = |i: usize, x: u16| Span::styled(CUT[i], Style::new().fg(at(x, 5)));
    let big = big_mark(mark, 3);
    let inner = |row: usize, x: u16| match &big {
        Some(b) => Span::styled(
            b[row]
                .chars()
                .nth(usize::from(x) - 1)
                .unwrap_or(' ')
                .to_string(),
            mark_style.bg(at(x, 5)),
        ),
        None if row == 1 && x == 2 => Span::styled(mark.to_string(), mark_style.bg(at(2, 5))),
        None => half(x),
    };
    let line = |row: usize, (l, r): (Span<'static>, Span<'static>)| {
        let mut spans = vec![l];
        spans.extend((1..4).map(|x| inner(row, x)));
        spans.push(r);
        Line::from(spans)
    };
    vec![
        line(0, (corner(0, 0), corner(1, 4))),
        line(1, (half(0), half(4))),
        line(2, (corner(2, 0), corner(3, 4))),
    ]
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
    let mark_span = Span::styled(mark.to_string(), mark_style);
    if !block {
        return vec![Line::from(vec![
            Span::styled("▌", edge),
            mark_span,
            Span::styled("▐", edge),
        ])];
    }
    let tick = tick(mark);
    let ink = mark_style.fg.unwrap_or(Color::Reset);
    let colour = |bottom: bool, x: usize| match &tick {
        Some(t) if if bottom { t.bottom } else { t.top }[x] => ink,
        _ => fill_c,
    };
    let mut top = vec![Span::styled("▛", edge)];
    top.extend((0..3).map(|x| Span::styled("▀", edge.bg(colour(false, x)))));
    top.push(Span::styled("▜", edge));
    let mut bottom = vec![Span::styled("▙", edge)];
    bottom.extend((0..3).map(|x| Span::styled("▄", edge.bg(colour(true, x)))));
    bottom.push(Span::styled("▟", edge));
    let mut mid = vec![Span::styled("▌", edge)];
    match (&tick, big_mark(mark, 1)) {
        (Some(t), _) => mid.extend(
            t.mid
                .iter()
                .map(|g| Span::styled(g.to_string(), mark_style)),
        ),
        (None, Some(b)) => mid.push(Span::styled(b[0].clone(), mark_style)),
        (None, None) => mid.push(Span::styled(format!(" {mark} "), mark_style)),
    }
    mid.push(Span::styled("▐", edge));
    vec![Line::from(top), Line::from(mid), Line::from(bottom)]
}

/// The check for the symbols whose body is a single row between two half-row edges: the lower
/// half of the top row and the upper half of the bottom row are cell-wide pixels, the middle
/// row three quadrant glyphs. `None` for any other mark.
struct Tick {
    top: [bool; 3],
    mid: [char; 3],
    bottom: [bool; 3],
}

fn tick(mark: char) -> Option<Tick> {
    const BITMAP: [&str; 4] = ["....##", "...##.", "##.#..", "..##.."];
    if mark != '✓' {
        return None;
    }
    let px = |r: usize, x: usize| BITMAP[r].as_bytes()[x] == b'#';
    // The outer rows are whole cells (both pixels of a pair are set together).
    let cell = |r: usize, c: usize| px(r, 2 * c);
    Some(Tick {
        top: [0, 1, 2].map(|c| cell(0, c)),
        mid: [0, 1, 2].map(|c| {
            quadrant(
                [px(1, 2 * c), px(1, 2 * c + 1)],
                [px(2, 2 * c), px(2, 2 * c + 1)],
            )
        }),
        bottom: [0, 1, 2].map(|c| cell(3, c)),
    })
}

/// Quadrant block glyph for the pixel pair rows `top` and `bottom` of a cell (`[left, right]`).
fn quadrant(top: [bool; 2], bottom: [bool; 2]) -> char {
    const Q: [char; 16] = [
        ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
    ];
    let bit = |on: bool, v: usize| if on { v } else { 0 };
    Q[bit(top[0], 1) + bit(top[1], 2) + bit(bottom[0], 4) + bit(bottom[1], 8)]
}

/// A check or dash drawn across three cells in quadrant pixels, for the 5×3 blocks, where a lone
/// `✓` would be a speck: `rows` is 3 for a symbol whose three centre cells are full cells on all
/// three rows, and 1 for one whose body is a single row. `None` for any other mark (a glitched
/// one flickers through single characters).
pub(crate) fn big_mark(mark: char, rows: usize) -> Option<Vec<String>> {
    // Pixels are half a cell wide, and (for `rows == 3`) half a row tall.
    let bitmap: &[&str] = match (mark, rows) {
        ('✓', 3) => &["......", "....##", "...##.", "#.##..", ".##...", "......"],
        ('−', 3) => return Some(vec!["   ".into(), "━━━".into(), "   ".into()]),
        ('−', _) => return Some(vec!["━━━".into()]),
        _ => return None,
    };
    let px = |r: usize, x: usize| bitmap[r].as_bytes()[x] == b'#';
    Some(
        bitmap
            .chunks(2)
            .enumerate()
            .map(|(i, _)| {
                (0..3)
                    .map(|c| {
                        quadrant(
                            [px(2 * i, 2 * c), px(2 * i, 2 * c + 1)],
                            [px(2 * i + 1, 2 * c), px(2 * i + 1, 2 * c + 1)],
                        )
                    })
                    .collect()
            })
            .collect(),
    )
}
