//! Turbo Vision's cell buffer as plank glyphs.

use plank_guest_support::CellGlyph;
use turbo_vision::core::draw::Cell;
use turbo_vision::core::palette::Style;

/// Every cell, background included: a Turbo Vision screen is defined by its
/// backgrounds, so none is left for the host to fill.
///
/// plank's frame protocol has no cursor field, so a session's text cursor
/// (see `Session::cursor`) is drawn here instead: the one cell at `cursor`,
/// when `Some`, gets its foreground and background swapped. Every other
/// cell is painted as-is.
#[must_use]
pub fn cells(buffer: &[Vec<Cell>], cursor: Option<(u16, u16)>) -> Vec<CellGlyph> {
    let mut out = Vec::with_capacity(buffer.len() * buffer.first().map_or(0, Vec::len));
    for (y, row) in buffer.iter().enumerate() {
        let Ok(y) = u16::try_from(y) else { break };
        for (x, cell) in row.iter().enumerate() {
            let Ok(x) = u16::try_from(x) else { break };
            let (fg, bg) = (cell.attr.fg.to_rgb(), cell.attr.bg.to_rgb());
            let (fg, bg) = if cursor == Some((x, y)) {
                (bg, fg)
            } else {
                (fg, bg)
            };
            out.push(CellGlyph {
                x,
                y,
                ch: cell.ch,
                fg,
                bg: Some(bg),
                bold: cell.attr.style.contains(Style::BOLD),
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use turbo_vision::core::palette::{Attr, TvColor};

    #[test]
    fn every_cell_becomes_a_glyph_with_its_background() {
        let plain = Attr::new(TvColor::White, TvColor::Blue);
        let buffer = vec![vec![Cell::new('a', plain), Cell::new('b', plain.bold())]];
        let glyphs = cells(&buffer, None);
        assert_eq!(glyphs.len(), 2);
        assert!(glyphs.iter().all(|g| g.bg == Some(TvColor::Blue.to_rgb())));
        assert_eq!((glyphs[1].x, glyphs[1].y, glyphs[1].ch), (1, 0, 'b'));
        assert!(!glyphs[0].bold);
        assert!(glyphs[1].bold);
    }

    #[test]
    fn the_cursor_cell_is_drawn_with_fg_and_bg_swapped() {
        let plain = Attr::new(TvColor::White, TvColor::Blue);
        let buffer = vec![vec![Cell::new('a', plain), Cell::new('b', plain)]];
        let glyphs = cells(&buffer, Some((1, 0)));
        assert_eq!(glyphs.len(), 2);
        // Cell (0, 0) is untouched.
        assert_eq!(glyphs[0].fg, TvColor::White.to_rgb());
        assert_eq!(glyphs[0].bg, Some(TvColor::Blue.to_rgb()));
        // Cell (1, 0), the cursor, is swapped.
        assert_eq!(glyphs[1].fg, TvColor::Blue.to_rgb());
        assert_eq!(glyphs[1].bg, Some(TvColor::White.to_rgb()));
    }

    #[test]
    fn no_cursor_swaps_nothing() {
        let plain = Attr::new(TvColor::White, TvColor::Blue);
        let buffer = vec![vec![Cell::new('a', plain)]];
        let glyphs = cells(&buffer, None);
        assert_eq!(glyphs[0].fg, TvColor::White.to_rgb());
        assert_eq!(glyphs[0].bg, Some(TvColor::Blue.to_rgb()));
    }
}
