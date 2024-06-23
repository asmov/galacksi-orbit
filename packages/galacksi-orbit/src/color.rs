use bevy::color::Color;

/// The color palette used in the game.
pub mod palette {
    use super::Color;

    pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
    pub const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
    pub const CYAN: Color = Color::srgb(0.0, 1.0, 1.0);
    pub const DARK_GRAY: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const LESS_DARK_GRAY: Color = Color::srgb(0.25, 0.25, 0.25);

    pub const BLOOM_WHITE: Color = Color::srgb(5.0, 5.0, 5.0);
    pub const BLOOM_RED: Color = Color::srgb(5.0, 0.0, 0.0);
    pub const BLOOM_GREEN: Color = Color::srgb(0.0, 5.0, 0.0);
    pub const BLOOM_BLUE: Color = Color::srgb(0.0, 0.0, 5.0);
    pub const BLOOM_PURPLE: Color = Color::srgb(5.0, 0.0, 5.0);
    pub const BLOOM_YELLOW: Color = Color::srgb(5.0, 5.0, 0.0);
    pub const BLOOM_CYAN: Color = Color::srgb(0.0, 5.0, 5.0);
    pub const BLOOM_ORANGE: Color = Color::srgb(5.0, 2.5, 0.0);
    pub const BLOOM_VIOLET: Color = Color::srgb(5.0, 0.0, 2.5);
}

/// A subset of colorful colors from [Palette] for use in player selection.
pub enum Palette {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    BloomWhite,
    BloomRed,
    BloomGreen,
    BloomBlue,
    BloomPurple,
    BloomYellow,
    BloomCyan,
    BloomOrange,
    BloomViolet
}

impl Palette {
    pub const fn color(&self) -> Color {
        match self {
            Self::Red => palette::RED,
            Self::Green => palette::GREEN,
            Self::Blue => palette::BLUE,
            Self::Yellow => palette::YELLOW,
            Self::Cyan => palette::CYAN,
            Self::BloomWhite => palette::BLOOM_WHITE,
            Self::BloomRed => palette::BLOOM_RED,
            Self::BloomGreen => palette::BLOOM_GREEN,
            Self::BloomBlue => palette::BLOOM_BLUE,
            Self::BloomPurple => palette::BLOOM_PURPLE,
            Self::BloomYellow => palette::BLOOM_YELLOW,
            Self::BloomCyan => palette::BLOOM_CYAN,
            Self::BloomOrange => palette::BLOOM_ORANGE,
            Self::BloomViolet => palette::BLOOM_VIOLET
        }
    }

    pub fn rand_bloom() -> Color {
        match rand::random::<u8>() % 9 {
            0 => palette::BLOOM_WHITE,
            1 => palette::BLOOM_RED,
            2 => palette::BLOOM_GREEN,
            3 => palette::BLOOM_BLUE,
            4 => palette::BLOOM_PURPLE,
            5 => palette::BLOOM_YELLOW,
            6 => palette::BLOOM_CYAN,
            7 => palette::BLOOM_ORANGE,
            8 => palette::BLOOM_VIOLET,
            _ => unreachable!()
        }
    }
    pub fn rand_button_text() -> Color {
        match rand::random::<u8>() % 8 {
            0 => palette::BLOOM_WHITE,
            1 => palette::BLOOM_RED,
            2 => palette::BLOOM_GREEN,
            3 => palette::BLOOM_PURPLE,
            4 => palette::BLOOM_YELLOW,
            5 => palette::BLOOM_CYAN,
            6 => palette::BLOOM_ORANGE,
            7 => palette::BLOOM_VIOLET,
            _ => unreachable!()
        }
    }

}

pub mod swatch {
    use super::{Color, palette};

    pub const MENU_TEXT: Color = Color::WHITE;
    pub const MENU_BUTTON_BG_NORMAL: Color = palette::DARK_GRAY;
    pub const MENU_BUTTON_BG_HOVER: Color = palette::LESS_DARK_GRAY;
}

