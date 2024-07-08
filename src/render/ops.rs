use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};


pub struct TextOptions {
    pub font: FontKind,
    pub v_align: VAlign,
    pub h_align: HAlign,
}

pub enum FontKind {
    // #[cfg_attr(feature = "strum", strum(serialize = "6x6"))]
    Font6x6,
    // #[cfg_attr(feature = "strum", strum(serialize = "6x8"))]
    Font6x8,
    // #[cfg_attr(feature = "strum", strum(serialize = "6x12"))]
    Font6x12,
    // #[cfg_attr(feature = "strum", strum(serialize = "8x16"))]
    Font8x16,
    // #[cfg_attr(feature = "strum", strum(serialize = "12x16"))]
    Font12x16,
    // #[cfg_attr(feature = "strum", strum(serialize = "24x32"))]
    Font24x32,
}

pub enum VAlign {
    Top,
    Centre,
    Bottom,
}

pub enum HAlign {
    Left,
    Centre,
    Right,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font: FontKind::Font12x16,
            h_align: HAlign::Centre,
            v_align: VAlign::Centre,
        }
    }
}
