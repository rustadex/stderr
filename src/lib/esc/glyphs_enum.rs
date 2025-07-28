// esc/glyphs.rs

//! A collection of Unicode glyphs for use in terminal output.

pub struct GlyphEnum;

impl GlyphEnum {
    // --- Your Original Glyphs ---
    pub const USAGE: &'static str = "\u{2756}";  // ❖
    pub const CMDR: &'static str = "\u{2318}";   // ⌘
    pub const BOTO: &'static str = "\u{232C}";   // ⌬
    pub const GEAR: &'static str = "\u{26ED}";   // ⛭
    pub const ROOK: &'static str = "\u{265C}";   // ♜
    pub const PAWN: &'static str = "\u{265F}";   // ♟
    pub const KING: &'static str = "\u{265A}";   // ♚
    pub const QUEEN: &'static str = "\u{265B}";  // ♛
    pub const VTRI: &'static str = "\u{25BD}";   // ▽
    pub const UTRI: &'static str = "\u{25B3}";   // △
    pub const XMARK: &'static str = "\u{292C}";  // ⤬
    pub const INFO: &'static str = "\u{2139}";   // ℹ
    pub const FAIL: &'static str = "\u{2715}";   // ✕
    pub const PASS: &'static str = "\u{2713}";   // ✓ (Note: Same as CHECK)
    pub const RECV: &'static str = "\u{27F2}";   // ⟲ ANTICLOCKWISE GAPPED CIRCLE ARROW

    // --- Translated from your Bash byte sequences ---
    pub const STAR: &'static str = "\u{2605}";      // ★ (from $'\xE2\x98\x85')
    pub const LAMBDA: &'static str = "\u{03BB}";    // λ (from $'\xCE\xBB')
    pub const IDOTS: &'static str = "\u{2026}";     // … (from $'\xE2\x80\xA6')
    pub const BOLT: &'static str = "\u{21AF}";      // ↯ (from $'\xE2\x86\ AF')
    pub const REDO: &'static str = "\u{21BB}";      // ↻ (from $'\xE2\x86\xBB')
    pub const SNAKE: &'static str = "\u{264B}";     // ♋ (from $'\xe2\x99\x8b', Cancer sign)
    pub const FLAG_OFF: &'static str = "\u{2690}";  // ⚐ (from $'\xe2\x9a\x90')
    pub const FLAG_ON: &'static str = "\u{2691}";   // ⚑ (from $'\xe2\x9a\x91')
    pub const DIAMOND: &'static str = "\u{16DC}";   // ᛜ (Runic letter)
    pub const ARROW_UP: &'static str = "\u{2191}";  // ↑ (from $'\xE2\x86\x91')
    pub const ARROW_DN: &'static str = "\u{2193}";  // ↓ (from $'\xE2\x86\x93')
    pub const DARR: &'static str = "\u{21B3}";      // ↳
    pub const UARR: &'static str = "\u{21B0}";      // ↰

                                                   // Be careful using it in terminal output.
    pub const DELTA: &'static str = "\u{25B3}";     // △ (from $'\xE2\x96\xB3', Same as UTRI)
    pub const USPARK: &'static str = "\u{27E1}";    //  weißen Rautenfleck (White Concave-Sided Diamond)

    // --- Your Other Glyphs ---
    pub const SWORD: &'static str = "\u{2694}";    // ⚔
    pub const MOON: &'static str = "\u{263E}";     // ☾
    pub const SUN: &'static str = "\u{2600}";      // ☀
    pub const SPARK: &'static str = "\u{273B}";    // ✻
    pub const COLON2: &'static str = "\u{2237}";   // ∷
    pub const THEREFORE: &'static str = "\u{2234}"; // ∴
    pub const BULLSEYE: &'static str = "\u{29BF}"; // ⦿
    pub const SECT: &'static str = "\u{00A7}";     // §
    pub const BOWTIE: &'static str = "\u{22C8}";   // ⋈
    pub const SUM: &'static str = "\u{2211}";      // ∑
    pub const PROD: &'static str = "\u{220F}";     // ∏
    pub const DHARMA: &'static str = "\u{2638}";   // ☸
    pub const SCROLL: &'static str = "\u{07F7}";   // ߷
    pub const NOTE: &'static str = "\u{266A}";     // ♪
    pub const ANCHOR: &'static str = "\u{2693}";   // ⚓
    pub const UNLOCK: &'static str = "\u{26BF}";   // ⚿
    pub const SPINDLE: &'static str = "\u{27D0}";  // ⟐
    pub const ANOTE: &'static str = "\u{260D}";    // ☍ (Conjunction)
    pub const CLOCK: &'static str = "\u{23F1}";    // ⏱
    pub const TIMER: &'static str = "\u{23F2}";    // ⏲
    pub const HOURGLASS: &'static str = "\u{29D6}"; // ⧖
    pub const CALENDAR: &'static str = "\u{1F5D3}";// 🗓

    pub const ALPHA: &'static str = "\u{03B1}";   // α
    pub const BETA: &'static str = "\u{03B2}";    // β
    pub const GAMMA: &'static str = "\u{03B3}";   // γ
    pub const DELTA: &'static str = "\u{03B4}";   // δ
    pub const EPSILON: &'static str = "\u{03B5}"; // ε
    pub const ZETA: &'static str = "\u{03B6}";    // ζ
    pub const ETA: &'static str = "\u{03B7}";     // η
    pub const THETA: &'static str = "\u{03B8}";   // θ
    pub const IOTA: &'static str = "\u{03B9}";    // ι
    pub const KAPPA: &'static str = "\u{03BA}";   // κ
    pub const LAMBDA: &'static str = "\u{03BB}";  // λ (You already had this one!)
    pub const MU: &'static str = "\u{03BC}";      // μ
    pub const NU: &'static str = "\u{03BD}";      // ν
    pub const XI: &'static str = "\u{03BE}";      // ξ
    pub const OMICRON: &'static str = "\u{03BF}"; // ο
    pub const PI: &'static str = "\u{03C0}";      // π
    pub const RHO: &'static str = "\u{03C1}";     // ρ
    pub const SIGMA: &'static str = "\u{03C3}";   // σ
    pub const TAU: &'static str = "\u{03C4}";     // τ
    pub const UPSILON: &'static str = "\u{03C5}"; // υ
    pub const PHI: &'static str = "\u{03C6}";     // φ
    pub const CHI: &'static str = "\u{03C7}";     // χ
    pub const PSI: &'static str = "\u{03C8}";     // ψ
    pub const OMEGA: &'static str = "\u{03C9}";   // ω

    pub const CORNER_UR: &'static str = "\u{2514}";    // └ (Up and Right - The final branch in a list)

    // --- Line Drawing ---
    pub const H_LINE: &'static str = "\u{2500}";      // ─ (Box Drawings Light Horizontal)
    pub const V_LINE: &'static str = "\u{2502}";      // │ (Box Drawings Light Vertical)
    pub const T_DOWN: &'static str = "\u{252C}";      // ┬ (Box Drawings Light Down and Horizontal)
    pub const T_UP: &'static str = "\u{2534}";        // ┴ (Box Drawings Light Up and Horizontal)
    pub const T_RIGHT: &'static str = "\u{251C}";     // ├ (Box Drawings Light Vertical and Right)
    pub const T_LEFT: &'static str = "\u{2524}";      // ┤ (Box Drawings Light Vertical and Left)
    pub const CORNER_DR: &'static str = "\u{250C}";   // ╭ (Presentation Form for Vertical Right Parenthesis)
                                                     // Often used as a top-left corner
    pub const CORNER_DL: &'static str = "\u{2510}";   // ╮ (Presentation Form for Vertical Left Parenthesis)
                                                     // Often used as a top-right corner
    pub const CORNER_UR: &'static str = "\u{2514}";   // ╰ (Presentation Form for Vertical Right Parenthesis)
                                                     // Often used as a bottom-left corner
    pub const CORNER_UL: &'static str = "\u{2518}";   // ╯ (Presentation Form for Vertical Left Parenthesis)
    
    // --- Heavy/Bold Lines (for emphasis) ---
    pub const H_LINE_HEAVY: &'static str = "\u{2501}"; // ━
    pub const V_LINE_HEAVY: &'static str = "\u{2503}"; // ┃
    pub const T_RIGHT_HEAVY: &'static str = "\u{2523}";// ┣
    pub const CORNER_UR_HEAVY: &'static str = "\u{2517}";// ┗

    // Often used as a bottom-right corner
    // --- Arrows & Pointers ---
    pub const ARROW_RIGHT: &'static str = "\u{2192}"; // →
    pub const ARROW_LEFT: &'static str = "\u{2190}";  // ←
    pub const HEAVY_ARROW_RIGHT: &'static str = "\u{279C}"; // ➜
    pub const POINTER: &'static str = "\u{25B6}";     // ▶ (Black Right-Pointing Triangle)

    pub const ARROW_SW: &'static str = "\u{2B0E}";        // ⬎ (South West Black Arrow)
    pub const ARROW_DN_RIGHT: &'static str = "\u{21B3}";  // ↳ (Downwards Arrow with Corner Leftwards)
    pub const ARROW_UP_RIGHT: &'static str = "\u{21B1}";  // ↱ (Upwards Arrow with Corner Leftwards)
    pub const ARROW_CURVE_SE: &'static str = "\u{2BA7}";  // ⮧ (South East Curved Arrow)
    pub const UNDO: &'static str = "\u{238C}";           // ⎌

    pub const REDO_CLOSED: &'static str = "\u{27F3}";    // ⟳ (Clockwise Gapped Circle Arrow)
    pub const RETURN_SYMBOL: &'static str = "\u{21A9}";  // ↩ (Leftwards Arrow with Hook)
    pub const NEWLINE_SYMBOL: &'static str = "\u{21B2}"; // ↲ (Downwards Arrow with Tip Leftwards)
    pub const BOLT: &'static str = "\u{21AF}";           // ↯ (You already had this!)
    pub const CURVE_ARROW_LEFT: &'static str = "\u{21B6}";// ↶ (Anticlockwise Top Semicircle Arrow)


    //
    // --- Status & Bullets ---
    pub const BULLET: &'static str = "\u{2022}";      // •
    pub const DOT: &'static str = "\u{2219}";         // ∙ (Bullet Operator)
    pub const TARGET: &'static str = "\u{25CE}";      // ◎ (Bullseye)
    pub const RADIO_ON: &'static str = "\u{25C9}";    // ◉ (Fisheye)
    pub const RADIO_OFF: &'static str = "\u{25CB}";   // ○ (White Circle)
    pub const SQUARE_SMALL: &'static str = "\u{25AB}";// ▫ (White Small Square)
    //
    // --- Miscellaneous ---
    pub const ELLIPSIS: &'static str = "\u{2026}";    // … (Same as IDOTS)
    pub const NOT_EQUAL: &'static str = "\u{2260}";   // ≠
    pub const APPROX: &'static str = "\u{2248}";      // ≈
    pub const DEGREE: &'static str = "\u{00B0}";      // °
}
