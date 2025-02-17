use bevy::prelude::*;

/// Component that implements Valve flickering lights into bevy. For more: [link](https://www.alanzucconi.com/2021/06/15/valve-flickering-lights/)
#[derive(Component, Reflect)]
#[require(PointLight(torch_point_light), Visibility(||Visibility::Hidden))]
pub struct Torch {
    /// Max value for luminous power in lumens, representing the amount of light emitted by this source in all directions.
    pub max_intensity: f32,
    /// Min value for luminous power in lumens, representing the amount of light emitted by this source in all directions.
    pub min_intensity: f32,
    pub pattern: Vec<char>,
    pub interval_counter: usize,
    pub cur_index: usize,
    pub current_intensity: f32,
}

impl Default for Torch {
    fn default() -> Self {
        Self {
            max_intensity: 50_000.0,
            min_intensity: 30_000.0,
            pattern: vec![
                'm', 'm', 'a', 'm', 'a', 'm', 'm', 'm', 'm', 'a', 'm', 'm', 'a', 'm', 'a', 'm',
                'a', 'a', 'a', 'm', 'a', 'm', 'm', 'm', 'a',
            ],
            interval_counter: 15,
            cur_index: 0,
            current_intensity: 30_000.0,
        }
    }
}

fn torch_point_light() -> PointLight {
    PointLight {
        shadows_enabled: true,
        color: Color::srgb_u8(190, 150, 0),
        range: 15.0,
        intensity: 550.0,
        ..default()
    }
}

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Torch>()
            .add_systems(FixedUpdate, update_torches)
            .add_systems(Update, update_torch_lights);
    }
}

pub fn update_torches(mut q: Query<&mut Torch>, mut counter: Local<usize>) {
    let min_value = 'a' as i32 as f32;
    let max_value = 'm' as i32 as f32;
    *counter += 1;
    for mut torch in q.iter_mut() {
        if *counter % torch.interval_counter != 0 {
            continue;
        }
        torch.cur_index = torch.cur_index.overflowing_add(1).0;
        let cur_value =
            torch.pattern[torch.cur_index % torch.pattern.len()] as i32 as f32 - min_value;
        torch.current_intensity = torch.min_intensity
            + (cur_value / (max_value - min_value) * (torch.max_intensity - torch.min_intensity));
    }
}

fn update_torch_lights(mut q: Query<(&mut PointLight, &Torch)>, time: Res<Time>) {
    for (mut light, torch) in q.iter_mut() {
        light.intensity = light
            .intensity
            .lerp(torch.current_intensity, 3.0 * time.delta_secs());
    }
}
