//! Cyberpunk look for the [`Checkbox`] widget.
//!
//! The shape language follows the dodai-ui cyberpunk design system: a filled slab with
//! chamfered ("cut") corners, a hard offset shadow, uppercase labels, a status readout, and
//! glitch / decrypt effects driven by a frame counter the caller owns.
//!
//! ```text
//! ▐✓◤  ARM SYSTEM  //ONLINE          inline, Slab chamfer, readout
//! ◢✓◤  ARM SYSTEM◤                 focused (label inverted into a bar)
//! ▐ ◤  ARM SYSTEM  //OFFLINE        unchecked: a dark slab
//!
//! ◢   ◣                             block (5×3), Tab chamfer, shadow
//!   ✓  ▌  ARM SYSTEM
//!      ▌  //ONLINE
//!  ▀▀▀▀▘
//! ```
//!
//! Enable it with [`Checkbox::cyber`]:
//!
//! ```
//! use tui_checkbox::Checkbox;
//! use tui_checkbox::cyber::{Chamfer, Cyber, CyberSize};
//!
//! let cb = Checkbox::new("Arm system", true).cyber(
//!     Cyber::new()
//!         .chamfer(Chamfer::Diagonal)
//!         .size(CyberSize::Block)
//!         .shadow(true)
//!         .readout(true),
//! );
//! ```
//!
//! [`Checkbox`]: crate::Checkbox
//! [`Checkbox::cyber`]: crate::Checkbox::cyber

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// Which corners of the slab are cut.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Chamfer {
    /// Bottom-right corner cut (default, the classic cyberpunk slab).
    #[default]
    Slab,
    /// Top-left and bottom-right corners cut.
    Diagonal,
    /// Top-right and bottom-left corners cut.
    AntiDiagonal,
    /// Both top corners cut.
    Tab,
    /// Both left corners cut.
    Bracket,
    /// No corners cut.
    Sharp,
}

impl Chamfer {
    /// Every chamfer, in display order.
    pub const ALL: [Self; 6] = [
        Self::Slab,
        Self::Diagonal,
        Self::AntiDiagonal,
        Self::Tab,
        Self::Bracket,
        Self::Sharp,
    ];

    /// Human-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Slab => "Slab",
            Self::Diagonal => "Diagonal",
            Self::AntiDiagonal => "Anti-diagonal",
            Self::Tab => "Tab",
            Self::Bracket => "Bracket",
            Self::Sharp => "Sharp",
        }
    }

    /// Cut corners as `[top-left, top-right, bottom-left, bottom-right]`.
    const fn corners(self) -> [bool; 4] {
        match self {
            Self::Slab => [false, false, false, true],
            Self::Diagonal => [true, false, false, true],
            Self::AntiDiagonal => [false, true, true, false],
            Self::Tab => [true, true, false, false],
            Self::Bracket => [true, false, true, false],
            Self::Sharp => [false; 4],
        }
    }

    /// Left and right edge glyphs of the one-row slab (drawn in the fill colour).
    const fn inline_edges(self) -> (&'static str, &'static str) {
        let [tl, tr, bl, br] = self.corners();
        let left = match (tl, bl) {
            (true, true) => "◀",
            (true, false) => "◢",
            (false, true) => "◥",
            (false, false) => "▐",
        };
        let right = match (tr, br) {
            (true, true) => "▶",
            (true, false) => "◣",
            (false, true) => "◤",
            (false, false) => "▌",
        };
        (left, right)
    }
}

/// Size of the cyber checkbox.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CyberSize {
    /// One row, three cells: `▐✓◤` (default).
    #[default]
    Inline,
    /// Three rows, five cells: a chunky slab with the label on the middle row.
    Block,
}

/// Colours of the cyber look.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CyberPalette {
    /// Fill of a checked slab, focus colour and readout `ONLINE`.
    pub primary: Color,
    /// Fill of an indeterminate slab, glitch and decrypt colour.
    pub secondary: Color,
    /// Fill of an unchecked slab.
    pub surface: Color,
    /// Label text.
    pub text: Color,
    /// Unchecked outline, disabled parts and readout `OFFLINE`.
    pub muted: Color,
    /// The mark drawn on a filled slab (usually the background colour).
    pub ink: Color,
    /// Hard offset shadow.
    pub shadow: Color,
}

impl CyberPalette {
    /// Night City yellow with cyan glitches and a red shadow (default).
    pub const CYBERPUNK: Self = Self {
        primary: Color::Rgb(0xfc, 0xee, 0x0a),
        secondary: Color::Rgb(0x00, 0xd2, 0xeb),
        surface: Color::Rgb(0x2a, 0x2a, 0x36),
        text: Color::Rgb(0xe6, 0xe6, 0xdc),
        muted: Color::Rgb(0x6a, 0x6a, 0x74),
        ink: Color::Rgb(0x0a, 0x0a, 0x10),
        shadow: Color::Rgb(0x9a, 0x1c, 0x2a),
    };

    /// Electric cyan with magenta glitches.
    pub const NEON: Self = Self {
        primary: Color::Rgb(0x00, 0xf0, 0xff),
        secondary: Color::Rgb(0xff, 0x00, 0x96),
        surface: Color::Rgb(0x16, 0x2a, 0x33),
        text: Color::Rgb(0xd8, 0xf8, 0xff),
        muted: Color::Rgb(0x4a, 0x6a, 0x78),
        ink: Color::Rgb(0x05, 0x0a, 0x10),
        shadow: Color::Rgb(0x6a, 0x00, 0x40),
    };

    /// Corporate red with bone-white glitches.
    pub const ARASAKA: Self = Self {
        primary: Color::Rgb(0xff, 0x32, 0x46),
        secondary: Color::Rgb(0xe6, 0xe6, 0xdc),
        surface: Color::Rgb(0x2a, 0x14, 0x18),
        text: Color::Rgb(0xf0, 0xe6, 0xe6),
        muted: Color::Rgb(0x6a, 0x4a, 0x50),
        ink: Color::Rgb(0x0a, 0x05, 0x06),
        shadow: Color::Rgb(0x4a, 0x0a, 0x10),
    };

    /// Hot pink with cyan glitches and a violet shadow.
    pub const SYNTHWAVE: Self = Self {
        primary: Color::Rgb(0xf6, 0x18, 0x8f),
        secondary: Color::Rgb(0x12, 0xc3, 0xe2),
        surface: Color::Rgb(0x2a, 0x18, 0x38),
        text: Color::Rgb(0xf0, 0xe0, 0xff),
        muted: Color::Rgb(0x7f, 0x70, 0x94),
        ink: Color::Rgb(0x0a, 0x08, 0x10),
        shadow: Color::Rgb(0x3a, 0x0a, 0x5a),
    };

    /// Every built-in palette with its name.
    pub const ALL: [(&'static str, Self); 4] = [
        ("Cyberpunk", Self::CYBERPUNK),
        ("Neon", Self::NEON),
        ("Arasaka", Self::ARASAKA),
        ("Synthwave", Self::SYNTHWAVE),
    ];

    /// Derives a palette from a shared [`Theme`](crate::Theme).
    ///
    /// `primary`/`secondary`/`foreground`/`muted` map directly; the ink is the theme
    /// background, the unchecked surface a mix of ink and muted, the shadow a dark `error`.
    #[cfg(feature = "theme")]
    #[must_use]
    pub fn from_theme(theme: &crate::Theme) -> Self {
        let ink = theme.background.unwrap_or(Color::Black);
        Self {
            primary: theme.primary,
            secondary: theme.secondary,
            surface: mix(ink, theme.muted, 0.35),
            text: theme.foreground,
            muted: theme.muted,
            ink,
            shadow: mix(ink, theme.error, 0.55),
        }
    }
}

impl Default for CyberPalette {
    fn default() -> Self {
        Self::CYBERPUNK
    }
}

/// Configuration of the cyber look; pass it to [`Checkbox::cyber`](crate::Checkbox::cyber).
///
/// Defaults: [`Chamfer::Slab`], [`CyberSize::Inline`], uppercase label, no shadow, no readout,
/// no glow, not flat, no effects, palette from the theme (or [`CyberPalette::CYBERPUNK`]).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[expect(clippy::struct_excessive_bools)] // independent builder flags, each set by its own method
pub struct Cyber {
    chamfer: Chamfer,
    size: CyberSize,
    shadow: bool,
    flat: bool,
    uppercase: bool,
    readout: Option<[&'static str; 3]>,
    glow: bool,
    glitch: Option<u16>,
    decrypt: Option<u8>,
    palette: Option<CyberPalette>,
}

impl Default for Cyber {
    fn default() -> Self {
        Self::new()
    }
}

impl Cyber {
    /// Default readout texts: checked, unchecked, indeterminate.
    pub const READOUT: [&'static str; 3] = ["//ONLINE", "//OFFLINE", "//PARTIAL"];

    /// The default cyber look.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            chamfer: Chamfer::Slab,
            size: CyberSize::Inline,
            shadow: false,
            flat: false,
            uppercase: true,
            readout: None,
            glow: false,
            glitch: None,
            decrypt: None,
            palette: None,
        }
    }

    /// Which corners are cut.
    #[must_use]
    pub const fn chamfer(mut self, chamfer: Chamfer) -> Self {
        self.chamfer = chamfer;
        self
    }

    /// Inline (one row) or block (three rows).
    #[must_use]
    pub const fn size(mut self, size: CyberSize) -> Self {
        self.size = size;
        self
    }

    /// Draws a hard offset shadow below (and, for blocks, right of) the slab.
    ///
    /// Adds one row and one column to the symbol.
    #[must_use]
    pub const fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    /// Flat look: keeps the cut-corner slab but drops everything painted behind it.
    ///
    /// No shadow, no glow and no filled focus bar; focus colours the label in the primary
    /// colour and keeps the `◤` tip. Wins over [`Cyber::shadow`] and [`Cyber::glow`] in
    /// any call order, and the layout is the same width as the non-flat look.
    #[must_use]
    pub const fn flat(mut self, flat: bool) -> Self {
        self.flat = flat;
        self
    }

    /// Uppercases the label (default `true`).
    #[must_use]
    pub const fn uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }

    /// Shows a status readout (`//ONLINE`, `//OFFLINE`, `//PARTIAL`) after the label.
    ///
    /// Inline it follows the label on the same row; a block puts it on the row below.
    #[must_use]
    pub const fn readout(mut self, readout: bool) -> Self {
        self.readout = if readout { Some(Self::READOUT) } else { None };
        self
    }

    /// Shows a readout with custom texts for checked, unchecked and indeterminate.
    #[must_use]
    pub const fn readout_texts(
        mut self,
        checked: &'static str,
        unchecked: &'static str,
        indeterminate: &'static str,
    ) -> Self {
        self.readout = Some([checked, unchecked, indeterminate]);
        self
    }

    /// Tints the label background with the fill colour while checked (a neon bar).
    ///
    /// Needs RGB colours in the palette; with named colours it does nothing.
    #[must_use]
    pub const fn glow(mut self, glow: bool) -> Self {
        self.glow = glow;
        self
    }

    /// Glitches the mark and label for this animation frame (`None` = off).
    ///
    /// Deterministic: the same frame always looks the same. Advance the frame for a few
    /// ticks after a toggle and then set it back to `None`.
    #[must_use]
    pub const fn glitch(mut self, frame: Option<u16>) -> Self {
        self.glitch = frame;
        self
    }

    /// Scrambles the label, revealing it left to right as `percent` goes `0..=100`
    /// (`None` = off). Values above 100 count as 100.
    #[must_use]
    pub const fn decrypt(mut self, percent: Option<u8>) -> Self {
        self.decrypt = percent;
        self
    }

    /// Sets the colours; wins over a theme.
    #[must_use]
    pub const fn palette(mut self, palette: CyberPalette) -> Self {
        self.palette = Some(palette);
        self
    }

    /// The explicit palette, if any.
    pub(crate) const fn explicit_palette(&self) -> Option<CyberPalette> {
        self.palette
    }

    /// Whether this is the three-row block size.
    pub(crate) const fn is_block(&self) -> bool {
        matches!(self.size, CyberSize::Block)
    }
}

/// What a checkbox shows.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Mark {
    Unchecked,
    Checked,
    Mixed,
}

/// Interaction state of a checkbox.
#[derive(Debug, Clone, Copy)]
pub(crate) struct State {
    pub mark: Mark,
    pub focused: bool,
    pub disabled: bool,
}

/// A drawn symbol: its rows, its width in cells and the row the label starts on.
#[derive(Debug, Clone)]
pub(crate) struct Glyph {
    pub lines: Vec<Line<'static>>,
    pub width: u16,
    pub label_row: u16,
}

/// Characters a glitched mark flickers through.
const GLITCH_MARKS: [char; 6] = ['▚', '▞', '▓', '▒', '░', '╳'];
/// Characters glitched label cells and scrambled decrypt cells use.
const SCRAMBLE: [char; 20] = [
    '!', '<', '>', '-', '_', '\\', '/', '[', ']', '{', '}', '=', '+', '*', '^', '?', '#', '%', '&',
    '@',
];

/// Small integer hash (deterministic noise for effects).
const fn noise(seed: u32, n: u32) -> u32 {
    let mut x = seed.wrapping_mul(0x9E37_79B9) ^ n.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    x
}

/// Picks one of `items` by noise.
fn pick<T: Copy>(items: &[T], seed: u32, n: u32) -> T {
    items[noise(seed, n) as usize % items.len()]
}

/// Whether a glitch frame shows a burst (two of three frames do).
fn glitch_burst(frame: u16) -> bool {
    !noise(u32::from(frame), 0).is_multiple_of(3)
}

/// Linear mix of two RGB colours; non-RGB colours snap to the nearer end.
pub(crate) fn mix(a: Color, b: Color, t: f64) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let t = t.clamp(0.0, 1.0);
            #[expect(clippy::cast_sign_loss)] // both ends are 0..=255, so the mix is too
            let ch =
                |x: u8, y: u8| (f64::from(x) + (f64::from(y) - f64::from(x)) * t).round() as u8;
            Color::Rgb(ch(r1, r2), ch(g1, g2), ch(b1, b2))
        }
        _ if t < 0.5 => a,
        _ => b,
    }
}

/// Fill colour of the slab and the mark drawn on it.
const fn fill(pal: &CyberPalette, state: State) -> (Color, char) {
    if state.disabled {
        return (
            pal.surface,
            match state.mark {
                Mark::Checked => '✓',
                Mark::Mixed => '−',
                Mark::Unchecked => ' ',
            },
        );
    }
    match state.mark {
        Mark::Checked => (pal.primary, '✓'),
        Mark::Mixed => (pal.secondary, '−'),
        Mark::Unchecked => (pal.surface, ' '),
    }
}

/// Builds the symbol for `state`.
pub(crate) fn glyph(cfg: &Cyber, pal: &CyberPalette, state: State) -> Glyph {
    let (fill_c, mut mark) = fill(pal, state);
    let mut mark_style = Style::new().fg(if state.disabled { pal.muted } else { pal.ink });
    mark_style = mark_style.bg(fill_c).add_modifier(Modifier::BOLD);
    let mut edge_c = fill_c;
    if state.focused && !state.disabled && state.mark == Mark::Unchecked {
        edge_c = pal.primary;
    }
    let burst = cfg.glitch.filter(|&f| glitch_burst(f));
    if let Some(f) = burst {
        mark = pick(&GLITCH_MARKS, u32::from(f), 1);
        mark_style = mark_style.fg(pal.secondary);
    }

    let mut lines = match cfg.size {
        CyberSize::Inline => {
            let (l, r) = cfg.chamfer.inline_edges();
            let r = match burst {
                Some(f) if noise(u32::from(f), 2).is_multiple_of(2) => "▞",
                _ => r,
            };
            vec![Line::from(vec![
                Span::styled(l, Style::new().fg(edge_c)),
                Span::styled(mark.to_string(), mark_style),
                Span::styled(r, Style::new().fg(edge_c)),
            ])]
        }
        CyberSize::Block if state.mark == Mark::Unchecked || state.disabled => {
            let outline = match (state.disabled, edge_c == pal.primary) {
                (true, _) => pal.surface,
                (false, true) => pal.primary,
                (false, false) => pal.muted,
            };
            outline_block(cfg.chamfer, outline, mark, mark_style, state)
        }
        CyberSize::Block => solid_block(cfg.chamfer, fill_c, mark, mark_style),
    };
    let mut width: u16 = match cfg.size {
        CyberSize::Inline => 3,
        CyberSize::Block => 5,
    };
    if cfg.shadow && !cfg.flat {
        let shadow = Style::new().fg(if state.disabled {
            pal.surface
        } else {
            pal.shadow
        });
        let rows = lines.len();
        for (i, line) in lines.iter_mut().enumerate() {
            let cell = if i == 0 || rows == 1 { " " } else { "▌" };
            line.spans.push(Span::styled(cell, shadow));
        }
        let bottom = format!(" {}▘", "▀".repeat(usize::from(width - 1)));
        lines.push(Line::from(Span::styled(bottom, shadow)));
        width += 1;
    }
    Glyph {
        lines,
        width,
        label_row: match cfg.size {
            CyberSize::Inline => 0,
            CyberSize::Block => 1,
        },
    }
}

/// A filled 5×3 slab; cut corners are triangles in the fill colour.
fn solid_block(
    chamfer: Chamfer,
    fill_c: Color,
    mark: char,
    mark_style: Style,
) -> Vec<Line<'static>> {
    let [tl, tr, bl, br] = chamfer.corners();
    let body = Style::new().bg(fill_c);
    let cut = Style::new().fg(fill_c);
    let corner = |is_cut: bool, glyph: &'static str| {
        if is_cut {
            Span::styled(glyph, cut)
        } else {
            Span::styled(" ", body)
        }
    };
    vec![
        Line::from(vec![
            corner(tl, "◢"),
            Span::styled("   ", body),
            corner(tr, "◣"),
        ]),
        Line::from(vec![
            Span::styled("  ", body),
            Span::styled(mark.to_string(), mark_style),
            Span::styled("  ", body),
        ]),
        Line::from(vec![
            corner(bl, "◥"),
            Span::styled("   ", body),
            corner(br, "◤"),
        ]),
    ]
}

/// An outlined 5×3 box; cut corners are thin diagonals.
fn outline_block(
    chamfer: Chamfer,
    edge_c: Color,
    mark: char,
    mark_style: Style,
    state: State,
) -> Vec<Line<'static>> {
    let [tl, tr, bl, br] = chamfer.corners();
    let edge = Style::new().fg(edge_c);
    let c = |is_cut: bool, cut: &'static str, square: &'static str| {
        Span::styled(if is_cut { cut } else { square }, edge)
    };
    // A disabled checked box keeps its mark, drawn without a fill.
    let mark_span = if state.mark == Mark::Unchecked {
        Span::raw(mark.to_string())
    } else {
        Span::styled(mark.to_string(), mark_style.bg(Color::Reset))
    };
    vec![
        Line::from(vec![
            c(tl, "╱", "▛"),
            Span::styled("▀▀▀", edge),
            c(tr, "╲", "▜"),
        ]),
        Line::from(vec![
            Span::styled("▌ ", edge),
            mark_span,
            Span::styled(" ▐", edge),
        ]),
        Line::from(vec![
            c(bl, "╲", "▙"),
            Span::styled("▄▄▄", edge),
            c(br, "╱", "▟"),
        ]),
    ]
}

/// Style of the label before the user's own styles.
pub(crate) fn label_style(pal: &CyberPalette, state: State) -> Style {
    if state.disabled {
        return Style::new().fg(pal.muted);
    }
    let mut s = Style::new().fg(pal.text);
    if state.mark != Mark::Unchecked {
        s = s.add_modifier(Modifier::BOLD);
    }
    if state.focused {
        s = s.fg(pal.primary).add_modifier(Modifier::BOLD);
    }
    s
}

/// Rebuilds `line` char by char through `f(index, char, style)`, merging equal styles.
fn map_chars(
    line: &Line<'static>,
    mut f: impl FnMut(usize, char, Style) -> (char, Style),
) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut i = 0;
    for span in &line.spans {
        for ch in span.content.chars() {
            let (c, s) = f(i, ch, span.style);
            i += 1;
            match spans.last_mut() {
                Some(last) if last.style == s => last.content.to_mut().push(c),
                _ => spans.push(Span::styled(c.to_string(), s)),
            }
        }
    }
    Line::from(spans)
}

/// Applies uppercase, decrypt and glitch to an already styled label.
pub(crate) fn transform_label(
    cfg: &Cyber,
    pal: &CyberPalette,
    line: Line<'static>,
) -> Line<'static> {
    let mut line = if cfg.uppercase {
        Line::from(
            line.spans
                .into_iter()
                .map(|s| Span::styled(s.content.to_uppercase(), s.style))
                .collect::<Vec<_>>(),
        )
    } else {
        line
    };
    let len = line
        .spans
        .iter()
        .map(|s| s.content.chars().count())
        .sum::<usize>();
    if let Some(p) = cfg.decrypt {
        let shown = len * usize::from(p.min(100)) / 100;
        let seed = u32::from(p);
        line = map_chars(&line, |i, c, s| {
            if i < shown || c.is_whitespace() {
                (c, s)
            } else {
                (pick(&SCRAMBLE, seed, i as u32), s.fg(pal.secondary))
            }
        });
    }
    if let Some(f) = cfg.glitch.filter(|&f| glitch_burst(f))
        && len > 0
    {
        let f = u32::from(f);
        let hits = [noise(f, 3) as usize % len, noise(f, 4) as usize % len];
        line = map_chars(&line, |i, c, s| {
            if hits.contains(&i) && !c.is_whitespace() {
                (pick(&SCRAMBLE, f, i as u32 + 7), s.fg(pal.secondary))
            } else {
                (c, s)
            }
        });
    }
    line
}

/// The readout span for `state`, if enabled.
pub(crate) fn readout(cfg: &Cyber, pal: &CyberPalette, state: State) -> Option<Span<'static>> {
    let [on, off, mixed] = cfg.readout?;
    let (text, style) = match state.mark {
        _ if state.disabled => (
            match state.mark {
                Mark::Checked => on,
                Mark::Unchecked => off,
                Mark::Mixed => mixed,
            },
            Style::new().fg(pal.muted),
        ),
        Mark::Checked => (
            on,
            Style::new().fg(pal.primary).add_modifier(Modifier::BOLD),
        ),
        Mark::Unchecked => (off, Style::new().fg(pal.muted)),
        Mark::Mixed => (
            mixed,
            Style::new().fg(pal.secondary).add_modifier(Modifier::BOLD),
        ),
    };
    Some(Span::styled(text, style))
}

/// Pads every label line by one cell on each side and paints the focus or glow bar in it.
///
/// Focused: the label is inverted (ink on primary) and ends in a `◤` tip. Glowing (checked or
/// indeterminate with [`Cyber::glow`]): the label sits on a dark tint of the fill, also with a
/// tip. Flat ([`Cyber::flat`]): no fill; focus only colours the label and tip. The padding is
/// always there, so neither bar shifts the layout when it appears.
pub(crate) fn bar(cfg: &Cyber, pal: &CyberPalette, state: State, lines: &mut [Line<'static>]) {
    let focused = state.focused && !state.disabled;
    let lit = cfg.glow && !cfg.flat && !state.disabled && state.mark != Mark::Unchecked;
    let tint = mix(pal.ink, fill(pal, state).0, 0.22);
    if focused && cfg.flat {
        // Flat focus: no fill, the label and its tip take the primary colour.
        for line in lines {
            for span in &mut line.spans {
                span.style = span.style.fg(pal.primary);
            }
            line.spans.insert(0, Span::raw(" "));
            line.spans
                .push(Span::styled("◤", Style::new().fg(pal.primary)));
        }
        return;
    }
    let paint = if focused {
        Some((pal.primary, Some(pal.ink)))
    } else if lit && tint != pal.ink {
        Some((tint, None))
    } else {
        None
    };
    for line in lines {
        let (left, right) = match paint {
            Some((bg, fg)) => {
                for span in &mut line.spans {
                    span.style = span.style.bg(bg);
                    if let Some(fg) = fg {
                        span.style = span.style.fg(fg);
                    }
                }
                (
                    Span::styled(" ", Style::new().bg(bg)),
                    Span::styled("◤", Style::new().fg(bg)),
                )
            }
            None => (Span::raw(" "), Span::raw(" ")),
        };
        line.spans.insert(0, left);
        line.spans.push(right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st(mark: Mark) -> State {
        State {
            mark,
            focused: false,
            disabled: false,
        }
    }

    fn text(line: &Line<'_>) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn inline_edges_follow_the_chamfer() {
        let cases = [
            (Chamfer::Slab, "▐✓◤"),
            (Chamfer::Diagonal, "◢✓◤"),
            (Chamfer::AntiDiagonal, "◥✓◣"),
            (Chamfer::Tab, "◢✓◣"),
            (Chamfer::Bracket, "◀✓▌"),
            (Chamfer::Sharp, "▐✓▌"),
        ];
        for (chamfer, want) in cases {
            let g = glyph(
                &Cyber::new().chamfer(chamfer),
                &CyberPalette::CYBERPUNK,
                st(Mark::Checked),
            );
            assert_eq!(text(&g.lines[0]), want, "{chamfer:?}");
            assert_eq!(g.width, 3);
        }
    }

    #[test]
    fn anti_diagonal_left_edge_cuts_bottom() {
        let (l, _) = Chamfer::AntiDiagonal.inline_edges();
        assert_eq!(l, "◥");
    }

    #[test]
    fn marks_per_state() {
        let pal = CyberPalette::CYBERPUNK;
        let cfg = Cyber::new();
        assert_eq!(
            text(&glyph(&cfg, &pal, st(Mark::Unchecked)).lines[0]),
            "▐ ◤"
        );
        assert_eq!(text(&glyph(&cfg, &pal, st(Mark::Mixed)).lines[0]), "▐−◤");
        let mixed = glyph(&cfg, &pal, st(Mark::Mixed));
        assert_eq!(mixed.lines[0].spans[0].style.fg, Some(pal.secondary));
    }

    #[test]
    fn focused_unchecked_lights_the_edges() {
        let pal = CyberPalette::CYBERPUNK;
        let g = glyph(
            &Cyber::new(),
            &pal,
            State {
                focused: true,
                ..st(Mark::Unchecked)
            },
        );
        assert_eq!(g.lines[0].spans[0].style.fg, Some(pal.primary));
    }

    #[test]
    fn block_is_five_by_three_with_label_on_middle_row() {
        let g = glyph(
            &Cyber::new().size(CyberSize::Block),
            &CyberPalette::CYBERPUNK,
            st(Mark::Checked),
        );
        assert_eq!((g.width, g.lines.len(), g.label_row), (5, 3, 1));
        assert_eq!(text(&g.lines[1]), "  ✓  ");
        assert_eq!(text(&g.lines[2]), "    ◤");
        for l in &g.lines {
            assert_eq!(l.width(), 5);
        }
    }

    #[test]
    fn unchecked_block_is_an_outline() {
        let g = glyph(
            &Cyber::new()
                .size(CyberSize::Block)
                .chamfer(Chamfer::Diagonal),
            &CyberPalette::CYBERPUNK,
            st(Mark::Unchecked),
        );
        assert_eq!(text(&g.lines[0]), "╱▀▀▀▜");
        assert_eq!(text(&g.lines[1]), "▌   ▐");
        assert_eq!(text(&g.lines[2]), "▙▄▄▄╱");
    }

    #[test]
    fn shadow_adds_a_row_and_a_column() {
        for size in [CyberSize::Inline, CyberSize::Block] {
            let plain = glyph(
                &Cyber::new().size(size),
                &CyberPalette::NEON,
                st(Mark::Checked),
            );
            let g = glyph(
                &Cyber::new().size(size).shadow(true),
                &CyberPalette::NEON,
                st(Mark::Checked),
            );
            assert_eq!(g.width, plain.width + 1);
            assert_eq!(g.lines.len(), plain.lines.len() + 1);
            for l in &g.lines {
                assert_eq!(l.width(), usize::from(g.width), "{size:?}");
            }
            assert!(text(g.lines.last().unwrap()).ends_with('▘'));
        }
    }

    #[test]
    fn glitch_is_deterministic_and_keeps_widths() {
        let cfg = Cyber::new().glitch(Some(3));
        let a = glyph(&cfg, &CyberPalette::CYBERPUNK, st(Mark::Checked));
        let b = glyph(&cfg, &CyberPalette::CYBERPUNK, st(Mark::Checked));
        assert_eq!(a.lines, b.lines);
        let bursts = (0..30u16).filter(|&f| glitch_burst(f)).count();
        assert!((10..30).contains(&bursts), "bursty, not constant: {bursts}");
        for f in 0..30 {
            let g = glyph(
                &Cyber::new().glitch(Some(f)),
                &CyberPalette::CYBERPUNK,
                st(Mark::Checked),
            );
            assert_eq!(g.lines[0].width(), 3);
            let l = transform_label(
                &Cyber::new().glitch(Some(f)),
                &CyberPalette::CYBERPUNK,
                Line::from("abc def"),
            );
            assert_eq!(l.width(), 7);
        }
    }

    #[test]
    fn decrypt_reveals_left_to_right() {
        let pal = CyberPalette::CYBERPUNK;
        let label = || Line::from("ab cd");
        let none = transform_label(&Cyber::new().decrypt(Some(0)), &pal, label());
        let t = text(&none);
        assert_eq!(t.chars().nth(2), Some(' '), "spaces stay");
        assert!(!t.starts_with('A'));
        let full = transform_label(&Cyber::new().decrypt(Some(100)), &pal, label());
        assert_eq!(text(&full), "AB CD");
        let over = transform_label(&Cyber::new().decrypt(Some(250)), &pal, label());
        assert_eq!(text(&over), "AB CD");
        let half = transform_label(&Cyber::new().decrypt(Some(40)), &pal, label());
        assert!(text(&half).starts_with("AB"));
        assert_ne!(text(&half), "AB CD");
    }

    #[test]
    fn uppercase_can_be_turned_off_and_handles_multichar() {
        let pal = CyberPalette::CYBERPUNK;
        assert_eq!(
            text(&transform_label(&Cyber::new(), &pal, Line::from("straße"))),
            "STRASSE"
        );
        assert_eq!(
            text(&transform_label(
                &Cyber::new().uppercase(false),
                &pal,
                Line::from("abc")
            )),
            "abc"
        );
    }

    #[test]
    fn readout_texts_and_colours() {
        let pal = CyberPalette::CYBERPUNK;
        assert!(readout(&Cyber::new(), &pal, st(Mark::Checked)).is_none());
        let cfg = Cyber::new().readout(true);
        assert_eq!(
            readout(&cfg, &pal, st(Mark::Checked)).unwrap().content,
            "//ONLINE"
        );
        assert_eq!(
            readout(&cfg, &pal, st(Mark::Unchecked)).unwrap().content,
            "//OFFLINE"
        );
        assert_eq!(
            readout(&cfg, &pal, st(Mark::Mixed)).unwrap().content,
            "//PARTIAL"
        );
        let custom = Cyber::new().readout_texts("ON", "OFF", "MIX");
        let disabled = State {
            disabled: true,
            ..st(Mark::Checked)
        };
        let s = readout(&custom, &pal, disabled).unwrap();
        assert_eq!((s.content.as_ref(), s.style.fg), ("ON", Some(pal.muted)));
    }

    #[test]
    fn flat_drops_shadow_glow_and_bar_fill() {
        let pal = CyberPalette::CYBERPUNK;
        let cfg = Cyber::new().shadow(true).glow(true).flat(true);
        // No shadow: same size as without it, in either call order.
        let g = glyph(&cfg, &pal, st(Mark::Checked));
        assert_eq!((g.width, g.lines.len()), (3, 1));
        let g = glyph(
            &Cyber::new().flat(true).shadow(true),
            &pal,
            st(Mark::Checked),
        );
        assert_eq!(g.width, 3);
        // The slab keeps its cut corner.
        assert_eq!(text(&g.lines[0]), "▐✓◤");
        // No glow.
        let mut lines = vec![Line::from("X")];
        bar(&cfg, &pal, st(Mark::Checked), &mut lines);
        assert_eq!(text(&lines[0]), " X ");
        assert!(lines[0].spans.iter().all(|s| s.style.bg.is_none()));
        // Focus: primary label and tip, no fill, same width.
        let focused = State {
            focused: true,
            ..st(Mark::Checked)
        };
        let mut lines = vec![Line::from("X")];
        bar(&cfg, &pal, focused, &mut lines);
        assert_eq!(text(&lines[0]), " X◤");
        assert!(lines[0].spans.iter().all(|s| s.style.bg.is_none()));
        assert_eq!(lines[0].spans[1].style.fg, Some(pal.primary));
        assert_eq!(lines[0].spans[2].style.fg, Some(pal.primary));
        // Flat block: no shadow row or column.
        let block = Cyber::new().size(CyberSize::Block).shadow(true).flat(true);
        let g = glyph(&block, &pal, st(Mark::Checked));
        assert_eq!((g.width, g.lines.len()), (5, 3));
    }

    #[test]
    fn bar_pads_always_and_paints_focus_or_glow() {
        let pal = CyberPalette::CYBERPUNK;
        let cfg = Cyber::new().glow(true);
        // Unchecked, unfocused: padding only, no colour.
        let mut lines = vec![Line::from("X")];
        bar(&cfg, &pal, st(Mark::Unchecked), &mut lines);
        assert_eq!(text(&lines[0]), " X ");
        assert!(lines[0].spans.iter().all(|s| s.style.bg.is_none()));
        // Checked with glow: tinted bar ending in a tip.
        let mut lines = vec![Line::from("X")];
        bar(&cfg, &pal, st(Mark::Checked), &mut lines);
        assert_eq!(text(&lines[0]), " X◤");
        assert!(lines[0].spans[1].style.bg.is_some());
        // Focused wins: ink on primary.
        let mut lines = vec![Line::from("X")];
        let focused = State {
            focused: true,
            ..st(Mark::Unchecked)
        };
        bar(&Cyber::new(), &pal, focused, &mut lines);
        assert_eq!(lines[0].spans[1].style.bg, Some(pal.primary));
        assert_eq!(lines[0].spans[1].style.fg, Some(pal.ink));
        assert_eq!(lines[0].spans[2].style.fg, Some(pal.primary));
        // Disabled never paints.
        let mut lines = vec![Line::from("X")];
        let disabled = State {
            disabled: true,
            ..focused
        };
        bar(&cfg, &pal, disabled, &mut lines);
        assert!(lines[0].spans.iter().all(|s| s.style.bg.is_none()));
        // Named colours cannot be mixed, so glow paints nothing.
        let named = CyberPalette {
            ink: Color::Black,
            primary: Color::Yellow,
            ..pal
        };
        let mut lines = vec![Line::from("X")];
        bar(&cfg, &named, st(Mark::Checked), &mut lines);
        assert_eq!(text(&lines[0]), " X ");
    }

    #[test]
    fn mix_blends_rgb_and_snaps_named() {
        assert_eq!(
            mix(Color::Rgb(0, 0, 0), Color::Rgb(200, 100, 50), 0.5),
            Color::Rgb(100, 50, 25)
        );
        assert_eq!(mix(Color::Black, Color::Red, 0.2), Color::Black);
        assert_eq!(mix(Color::Black, Color::Red, 0.8), Color::Red);
    }

    #[test]
    fn disabled_uses_surface_and_muted() {
        let pal = CyberPalette::CYBERPUNK;
        let s = State {
            disabled: true,
            ..st(Mark::Checked)
        };
        assert_eq!(label_style(&pal, s).fg, Some(pal.muted));
        let g = glyph(&Cyber::new(), &pal, s);
        assert_eq!(g.lines[0].spans[0].style.fg, Some(pal.surface));
    }

    #[test]
    fn chamfer_names_are_unique() {
        let mut names: Vec<_> = Chamfer::ALL.iter().map(|c| c.name()).collect();
        names.dedup();
        assert_eq!(names.len(), Chamfer::ALL.len());
    }
}
