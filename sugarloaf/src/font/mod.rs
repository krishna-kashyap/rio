pub mod constants;
pub mod fonts;
#[cfg(not(target_arch = "wasm32"))]
pub mod loader;
// pub mod ligatures;

pub const FONT_ID_REGULAR: usize = 0;
pub const FONT_ID_ITALIC: usize = 1;
pub const FONT_ID_BOLD: usize = 2;
pub const FONT_ID_BOLD_ITALIC: usize = 3;
pub const FONT_ID_SYMBOL: usize = 4;
pub const FONT_ID_EMOJIS: usize = 5;
pub const FONT_ID_UNICODE: usize = 6;
pub const FONT_ID_ICONS: usize = 7;
pub const FONT_ID_BUILTIN: usize = 8;

use crate::font::constants::*;

pub type SugarloafFont = fonts::SugarloafFont;
pub type SugarloafFonts = fonts::SugarloafFonts;

