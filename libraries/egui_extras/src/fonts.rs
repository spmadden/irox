// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use egui::epaint::text::FontData;
use egui::{Context, FontDefinitions, FontFamily};

pub const UBUNTU: &[u8] = include_bytes!("../fonts/Ubuntu-R.ttf");
pub const UBUNTU_BOLD: &[u8] = include_bytes!("../fonts/Ubuntu-B.ttf");
pub const UBUNTU_ITALIC: &[u8] = include_bytes!("../fonts/Ubuntu-RI.ttf");
pub const UBUNTU_BOLD_ITALIC: &[u8] = include_bytes!("../fonts/Ubuntu-BI.ttf");
pub const UBUNTU_CONDENSED: &[u8] = include_bytes!("../fonts/Ubuntu-C.ttf");
pub const UBUNTU_LIGHT: &[u8] = include_bytes!("../fonts/Ubuntu-L.ttf");
pub const UBUNTU_LIGHT_ITALIC: &[u8] = include_bytes!("../fonts/Ubuntu-LI.ttf");
pub const UBUNTU_MEDIUM: &[u8] = include_bytes!("../fonts/Ubuntu-M.ttf");
pub const UBUNTU_MEDIUM_ITALIC: &[u8] = include_bytes!("../fonts/Ubuntu-MI.ttf");
pub const UBUNTU_THIN: &[u8] = include_bytes!("../fonts/Ubuntu-Th.ttf");

pub const UBUNTU_MONOSPACE: &[u8] = include_bytes!("../fonts/UbuntuMono-R.ttf");
pub const UBUNTU_MONOSPACE_ITALIC: &[u8] = include_bytes!("../fonts/UbuntuMono-RI.ttf");
pub const UBUNTU_MONOSPACE_BOLD: &[u8] = include_bytes!("../fonts/UbuntuMono-B.ttf");
pub const UBUNTU_MONOSPACE_BOLD_ITALIC: &[u8] = include_bytes!("../fonts/UbuntuMono-BI.ttf");

pub const REGULAR: &str = "regular";
pub const BOLD: &str = "bold";
pub const ITALIC: &str = "italic";
pub const BOLD_ITALIC: &str = "bold_italic";
pub const CONDENSED: &str = "condensed";
pub const LIGHT: &str = "light";
pub const LIGHT_ITALIC: &str = "light_italic";
pub const MEDIUM: &str = "medium";
pub const MEDIUM_ITALIC: &str = "medium_italic";
pub const THIN: &str = "thin";
pub const MONOSPACE: &str = "monospace";
pub const MONOSPACE_ITALIC: &str = "monospace_italic";
pub const MONOSPACE_BOLD: &str = "monospace_bold";
pub const MONOSPACE_BOLD_ITALIC: &str = "monospace_bold_italic";

#[derive(Debug, Default, Copy, Clone)]
pub struct FontSet {
    pub ubuntu: bool,
    pub ubuntu_bold: bool,
    pub ubuntu_italic: bool,
    pub ubuntu_bold_italic: bool,
    pub ubuntu_condensed: bool,
    pub ubuntu_light: bool,
    pub ubuntu_light_italic: bool,
    pub ubuntu_medium: bool,
    pub ubuntu_medium_italic: bool,
    pub ubuntu_thin: bool,
    pub ubuntu_mono: bool,
    pub ubuntu_mono_bold: bool,
    pub ubuntu_mono_italic: bool,
    pub ubuntu_mono_bold_italic: bool,
}
impl FontSet {
    pub fn all() -> Self {
        Self {
            ubuntu: true,
            ubuntu_bold: true,
            ubuntu_italic: true,
            ubuntu_bold_italic: true,
            ubuntu_condensed: true,
            ubuntu_light: true,
            ubuntu_light_italic: true,
            ubuntu_medium: true,
            ubuntu_medium_italic: true,
            ubuntu_thin: true,
            ubuntu_mono: true,
            ubuntu_mono_bold: true,
            ubuntu_mono_italic: true,
            ubuntu_mono_bold_italic: true,
        }
    }
}
macro_rules! load_instance {
    ($fonts:ident, $str:ident, $font:ident) => {
        $fonts.font_data.insert(
            $str.to_string(),
            alloc::sync::Arc::new(FontData::from_static($font)),
        );
        $fonts
            .families
            .insert(FontFamily::Name($str.into()), vec![$str.into()]);
    };
}

pub fn load_fonts(set: FontSet, ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    if set.ubuntu {
        load_instance!(fonts, REGULAR, UBUNTU);
    }
    if set.ubuntu_bold {
        load_instance!(fonts, BOLD, UBUNTU_BOLD);
    }
    if set.ubuntu_italic {
        load_instance!(fonts, ITALIC, UBUNTU_ITALIC);
    }
    if set.ubuntu_bold_italic {
        load_instance!(fonts, BOLD_ITALIC, UBUNTU_BOLD_ITALIC);
    }
    if set.ubuntu_condensed {
        load_instance!(fonts, CONDENSED, UBUNTU_CONDENSED);
    }
    if set.ubuntu_light {
        load_instance!(fonts, LIGHT, UBUNTU_LIGHT);
    }
    if set.ubuntu_light_italic {
        load_instance!(fonts, LIGHT_ITALIC, UBUNTU_LIGHT_ITALIC);
    }
    if set.ubuntu_medium {
        load_instance!(fonts, MEDIUM, UBUNTU_MEDIUM);
    }
    if set.ubuntu_medium_italic {
        load_instance!(fonts, MEDIUM_ITALIC, UBUNTU_MEDIUM_ITALIC);
    }
    if set.ubuntu_thin {
        load_instance!(fonts, THIN, UBUNTU_THIN);
    }
    if set.ubuntu_mono {
        load_instance!(fonts, MONOSPACE, UBUNTU_MONOSPACE);
    }
    if set.ubuntu_mono_bold {
        load_instance!(fonts, MONOSPACE_BOLD, UBUNTU_MONOSPACE_BOLD);
    }
    if set.ubuntu_mono_bold_italic {
        load_instance!(fonts, MONOSPACE_BOLD_ITALIC, UBUNTU_MONOSPACE_BOLD_ITALIC);
    }
    if set.ubuntu_mono_italic {
        load_instance!(fonts, MONOSPACE_ITALIC, UBUNTU_MONOSPACE_ITALIC);
    }

    ctx.set_fonts(fonts);
}
