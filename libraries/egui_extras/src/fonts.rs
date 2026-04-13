// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use egui::epaint::text::FontPriority;
use egui::epaint::text::{FontData, FontInsert, InsertFontFamily};
use egui::{Context, FontFamily};

pub const UBUNTU: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-R.ttf");
pub const UBUNTU_BOLD: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-B.ttf");
pub const UBUNTU_ITALIC: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-RI.ttf");
pub const UBUNTU_BOLD_ITALIC: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-BI.ttf");
pub const UBUNTU_CONDENSED: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-C.ttf");
pub const UBUNTU_LIGHT: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-L.ttf");
pub const UBUNTU_LIGHT_ITALIC: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-LI.ttf");
pub const UBUNTU_MEDIUM: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-M.ttf");
pub const UBUNTU_MEDIUM_ITALIC: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-MI.ttf");
pub const UBUNTU_THIN: &[u8] = include_bytes!("../fonts/ubuntu/Ubuntu-Th.ttf");

pub const UBUNTU_MONOSPACE: &[u8] = include_bytes!("../fonts/ubuntu/UbuntuMono-R.ttf");
pub const UBUNTU_MONOSPACE_ITALIC: &[u8] = include_bytes!("../fonts/ubuntu/UbuntuMono-RI.ttf");
pub const UBUNTU_MONOSPACE_BOLD: &[u8] = include_bytes!("../fonts/ubuntu/UbuntuMono-B.ttf");
pub const UBUNTU_MONOSPACE_BOLD_ITALIC: &[u8] = include_bytes!("../fonts/ubuntu/UbuntuMono-BI.ttf");

pub const NOTO_EMOJI: &[u8] = include_bytes!("../fonts/noto/NotoEmoji-Regular.ttf");
pub const EMOJI_ICON: &[u8] = include_bytes!("../fonts/emoji-icon/emoji.ttf");

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
pub const NOTO_EMOJI_REGULAR: &str = "noto_emoji_regular";
pub const EMOJI_ICON_REGULAR: &str = "emoji_icon";

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
    pub emoji_icon: bool,
    pub noto_emoji: bool,
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
            emoji_icon: true,
            noto_emoji: true,
        }
    }
    pub fn basics() -> Self {
        Self {
            ubuntu: false,
            ubuntu_bold: false,
            ubuntu_italic: false,
            ubuntu_bold_italic: false,
            ubuntu_condensed: false,
            ubuntu_light: true,
            ubuntu_light_italic: false,
            ubuntu_medium: false,
            ubuntu_medium_italic: false,
            ubuntu_thin: false,
            ubuntu_mono: true,
            ubuntu_mono_bold: false,
            ubuntu_mono_italic: false,
            ubuntu_mono_bold_italic: false,
            emoji_icon: true,
            noto_emoji: false,
        }
    }
}
macro_rules! load_instance {
    ($ctx:ident, $str:ident, $font:ident, $($fams:expr)*) => {
       $ctx.add_font(FontInsert {
            name: $str.to_string(),
            data: FontData::from_static($font),
            families: vec![
                InsertFontFamily {
                    family: FontFamily::Name($str.into()),
                    priority: FontPriority::Lowest,
                },
                $(
                InsertFontFamily {
                    family: $fams,
                    priority: FontPriority::Lowest,
                }),*
            ],
        })
    };
}

pub fn load_fonts(set: FontSet, ctx: &Context) {
    if set.ubuntu {
        load_instance!(ctx, REGULAR, UBUNTU, FontFamily::Proportional);
    }
    if set.ubuntu_bold {
        load_instance!(ctx, BOLD, UBUNTU_BOLD,);
    }
    if set.ubuntu_italic {
        load_instance!(ctx, ITALIC, UBUNTU_ITALIC,);
    }
    if set.ubuntu_bold_italic {
        load_instance!(ctx, BOLD_ITALIC, UBUNTU_BOLD_ITALIC,);
    }
    if set.ubuntu_condensed {
        load_instance!(ctx, CONDENSED, UBUNTU_CONDENSED, FontFamily::Proportional);
    }
    if set.ubuntu_light {
        load_instance!(ctx, LIGHT, UBUNTU_LIGHT, FontFamily::Proportional);
    }
    if set.ubuntu_light_italic {
        load_instance!(ctx, LIGHT_ITALIC, UBUNTU_LIGHT_ITALIC,);
    }
    if set.ubuntu_medium {
        load_instance!(ctx, MEDIUM, UBUNTU_MEDIUM, FontFamily::Proportional);
    }
    if set.ubuntu_medium_italic {
        load_instance!(ctx, MEDIUM_ITALIC, UBUNTU_MEDIUM_ITALIC,);
    }
    if set.ubuntu_thin {
        load_instance!(ctx, THIN, UBUNTU_THIN, FontFamily::Proportional);
    }
    if set.ubuntu_mono {
        load_instance!(ctx, MONOSPACE, UBUNTU_MONOSPACE, FontFamily::Monospace);
    }
    if set.ubuntu_mono_bold {
        load_instance!(ctx, MONOSPACE_BOLD, UBUNTU_MONOSPACE_BOLD,);
    }
    if set.ubuntu_mono_bold_italic {
        load_instance!(ctx, MONOSPACE_BOLD_ITALIC, UBUNTU_MONOSPACE_BOLD_ITALIC,);
    }
    if set.ubuntu_mono_italic {
        load_instance!(ctx, MONOSPACE_ITALIC, UBUNTU_MONOSPACE_ITALIC,);
    }
    if set.emoji_icon {
        load_instance!(ctx, EMOJI_ICON_REGULAR, EMOJI_ICON, FontFamily::Proportional FontFamily::Monospace);
    }
    if set.noto_emoji {
        load_instance!(ctx, NOTO_EMOJI_REGULAR, NOTO_EMOJI, FontFamily::Proportional FontFamily::Monospace);
    }
}
