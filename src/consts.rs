use bevy::{color::LinearRgba, prelude::Color};

// pub const WINDOW_WIDTH: f32 = 960.;
// pub const WINDOW_HEIGHT: f32 = 600.;

pub const BASE_FONT: &str = "fonts/NotJamOldStyle11.ttf";

pub const MY_ACCENT_COLOR: Color = Color::LinearRgba(LinearRgba {
    red: 0.901,
    green: 0.4,
    blue: 0.01,
    alpha: 1.0,
});

pub const BG_COLOR: Color = Color::srgb(0.08, 0.08, 0.115);

// pub const ENEMY_SKULL : usize = 622;
// pub const POTION : usize = 671;
pub const GRAVES: [usize; 5] = [686, 687, 686, 687, 688];
