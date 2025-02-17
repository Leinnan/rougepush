use bevy::input::common_conditions::input_just_released;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::prelude::{
    PerfUiEntryEntityCount, PerfUiEntryFrameTime, PerfUiEntryFrameTimeWorst, PerfUiRoot,
};
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)));

    app.add_systems(Startup, configure_egui);
    app.add_systems(Startup, perf_info);
    app.add_systems(
        Update,
        toggle_perf_info.run_if(input_just_released(KeyCode::F2)),
    );

    app.add_plugins((
        iyes_perf_ui::PerfUiPlugin,
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
    ));
}

fn toggle_perf_info(mut query: Query<&mut Visibility, With<PerfUiRoot>>) {
    let Ok(mut vis) = query.get_single_mut() else {
        return;
    };
    *vis = if vis.eq(&Visibility::Hidden) {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
}

fn perf_info(mut commands: Commands) {
    commands
        .spawn(PerfUiRoot {
            position: iyes_perf_ui::prelude::PerfUiPosition::BottomRight,
            ..default()
        })
        .insert((
            PerfUiEntryFrameTime::default(),
            PerfUiEntryFrameTimeWorst::default(),
            PerfUiEntryEntityCount::default(),
            iyes_perf_ui::prelude::PerfUiEntryCpuUsage::default(),
            iyes_perf_ui::prelude::PerfUiEntryMemUsage::default(),
        ))
        .insert(Name::new("PerfUiRoot"));
}

fn configure_egui(mut contexts: EguiContexts) {
    #[cfg(windows)]
    {
        if let Some((regular, semibold)) = get_fonts() {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "regular".to_owned(),
                egui::FontData::from_owned(regular).into(),
            );
            fonts.font_data.insert(
                "semibold".to_owned(),
                egui::FontData::from_owned(semibold).into(),
            );

            // Put my font first (highest priority) for proportional text:
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "regular".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Name("semibold".into()))
                .or_default()
                .insert(0, "semibold".to_owned());

            // Put my font as last fallback for monospace:
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("regular".to_owned());

            // Tell egui to use these fonts:
            contexts.ctx_mut().set_fonts(fonts);
        }
    }
    contexts.ctx_mut().style_mut(|style| {
        for font_id in style.text_styles.values_mut() {
            font_id.size *= 1.2;
        }
    });
}

#[cfg(windows)]
fn get_fonts() -> Option<(Vec<u8>, Vec<u8>)> {
    use std::fs;

    let app_data = std::env::var("APPDATA").ok()?;
    let font_path = std::path::Path::new(&app_data);

    let regular = fs::read(font_path.join("../Local/Microsoft/Windows/Fonts/aptos.ttf")).ok()?;
    let semibold =
        fs::read(font_path.join("../Local/Microsoft/Windows/Fonts/aptos-bold.ttf")).ok()?;

    Some((regular, semibold))
}
