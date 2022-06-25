use bevy::prelude::*;

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::Rgba {
        red: 103.0 / u8::MAX as f32,
        green: 214.0 / u8::MAX as f32,
        blue: 217.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const LIGHT: Color = Color::Rgba {
        red: 247.0 / u8::MAX as f32,
        green: 247.0 / u8::MAX as f32,
        blue: 247.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const DARK: Color = Color::Rgba {
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
