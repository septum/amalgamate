use bevy::prelude::*;

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::Rgba {
        red: 63.0 / u8::MAX as f32,
        green: 91.0 / u8::MAX as f32,
        blue: 126.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const LIGHT: Color = Color::Rgba {
        red: 240.0 / u8::MAX as f32,
        green: 240.0 / u8::MAX as f32,
        blue: 240.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const LIGHTER: Color = Color::Rgba {
        red: 247.0 / u8::MAX as f32,
        green: 247.0 / u8::MAX as f32,
        blue: 247.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const DARK: Color = Color::Rgba {
        red: 14.0 / u8::MAX as f32,
        green: 14.0 / u8::MAX as f32,
        blue: 14.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const DARKER: Color = Color::Rgba {
        red: 7.0 / u8::MAX as f32,
        green: 7.0 / u8::MAX as f32,
        blue: 7.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const TRANSPARENT: Color = Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    };
}
