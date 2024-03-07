use bevy::prelude::Color;
use bevy_egui::egui::Color32;

pub const GIT_HASH: &str = env!("GIT_HASH");
pub const GIT_DATE: &str = env!("GIT_DATE");

// pub const WINDOW_WIDTH: f32 = 960.;
// pub const WINDOW_HEIGHT: f32 = 600.;

pub const BASE_FONT: &str = "fonts/NotJamOldStyle11.ttf";

pub const MY_ACCENT_COLOR: Color = Color::Rgba {
    red: 0.901,
    green: 0.4,
    blue: 0.01,
    alpha: 1.0,
};
pub const MY_ACCENT_COLOR32: Color32 = Color32::from_rgb(230, 102, 1);
pub const BG_COLOR: Color = Color::rgb(0.08, 0.08, 0.115);

// pub const ENEMY_SKULL : usize = 622;
// pub const POTION : usize = 671;
pub const GRAVES: [usize; 5] = [686, 687, 686, 687, 688];
