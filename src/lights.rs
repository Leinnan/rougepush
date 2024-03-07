use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Torch {
    pub max_intensity: f32,
    pub min_intensity: f32,
    pub pattern: Vec<char>,
    pub interval_counter: usize,
    pub cur_index: usize,
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
        }
    }
}

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Torch>()
            .add_systems(Update, spawn_torches)
            .add_systems(FixedUpdate, update_torches);
    }
}

pub fn spawn_torches(q: Query<(Entity, &Transform, &Torch), Added<Torch>>, mut commands: Commands) {
    for (e, t, torch) in q.iter() {
        commands.entity(e).insert(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: false,
                color: Color::rgb_u8(190, 150, 0),
                range: 15.0,
                intensity: torch.max_intensity,
                ..default()
            },
            transform: *t,
            ..default()
        });
    }
}

pub fn update_torches(mut q: Query<(&mut PointLight, &mut Torch)>, mut counter: Local<usize>) {
    let min_value = 'a' as i32 as f32;
    let max_value = 'm' as i32 as f32;
    *counter += 1;
    for (mut light, mut torch) in q.iter_mut() {
        if *counter % torch.interval_counter != 0 {
            continue;
        }
        torch.cur_index += 1;
        if torch.cur_index >= torch.pattern.len() {
            torch.cur_index = 0;
        }
        let cur_value = torch.pattern[torch.cur_index] as i32 as f32 - min_value;
        light.intensity = torch.min_intensity
            + (cur_value / (max_value - min_value) * (torch.max_intensity - torch.min_intensity));
    }
}
