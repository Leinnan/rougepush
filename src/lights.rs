use bevy::prelude::*;

const MIN_VALUE: f32 = 'a' as i32 as f32;
const MAX_VALUE: f32 = 'm' as i32 as f32;
const VAL_DIFFERENCE: f32 = MAX_VALUE - MIN_VALUE;

/// Component that implements Valve flickering lights into bevy. For more: [link](https://www.alanzucconi.com/2021/06/15/valve-flickering-lights/)
#[derive(Component, Reflect)]
#[require(PointLight = torch_point_light(), Visibility = Visibility::Hidden, LightPattern)]
pub struct Torch {
    /// How much can the light power change. Max value is `min_intensity + intensity_variation`
    pub intensity_variation: f32,
    /// Min value for luminous power in lumens, representing the amount of light emitted by this source in all directions.
    pub min_intensity: f32,
    pub interval_counter: usize,
    pub cur_index: usize,
    pub target_intensity: f32,
}

impl Torch {
    pub fn update_intensity(&mut self, pattern: &LightPattern) {
        let variation = pattern.value(self.cur_index);
        self.target_intensity = self.min_intensity + variation * self.intensity_variation;
    }
}

#[derive(Reflect, Component, Default)]
// #[component(immutable)]
pub enum LightPattern {
    #[default]
    StableFull,
    FromChars(Vec<f32>),
}

impl LightPattern {
    pub fn from_chars(chars: &[char]) -> Self {
        let array = chars
            .iter()
            .map(|e| {
                let value = (*e) as i32 as f32;
                (value - MIN_VALUE) % VAL_DIFFERENCE
            })
            .collect();
        LightPattern::FromChars(array)
    }
    pub fn value(&self, index: usize) -> f32 {
        match self {
            LightPattern::StableFull => 1.0,
            LightPattern::FromChars(items) => {
                let i = index % items.len();
                items[i]
            }
        }
    }
}

impl Default for Torch {
    fn default() -> Self {
        Self {
            intensity_variation: 20_000.0,
            min_intensity: 30_000.0,
            interval_counter: 15,
            cur_index: 0,
            target_intensity: 30_000.0,
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

pub fn update_torches(mut q: Query<(&mut Torch, &LightPattern)>, mut counter: Local<usize>) {
    *counter += 1;
    for (mut torch, pattern) in q.iter_mut() {
        if *counter % torch.interval_counter != 0 {
            continue;
        }
        torch.cur_index = torch.cur_index.overflowing_add(1).0;
        torch.update_intensity(pattern);
    }
}

fn update_torch_lights(mut q: Query<(&mut PointLight, &Torch)>, time: Res<Time>) {
    let delta = time.delta_secs();
    q.par_iter_mut().for_each(|(mut light, torch)| {
        light.intensity = light.intensity.lerp(torch.target_intensity, 3.0 * delta);
    });
}
