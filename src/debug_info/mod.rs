use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Duration,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

pub struct DebugInfoPlugin;

impl Plugin for DebugInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(display_debug_info)
            .add_system(update)
            .init_resource::<DebugInfoState>();
    }
}

#[derive(Resource)]
pub struct DebugInfoState {
    pub timer: Timer,
    pub update_now: bool,
    pub fps: f64,
}

impl Default for DebugInfoState {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, TimerMode::Once),
            update_now: true,
            fps: 0.0,
        }
    }
}

fn display_debug_info(mut contexts: EguiContexts, state_resource: Option<ResMut<DebugInfoState>>) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        if let Some(state) = state_resource {
            let fps = format!("{:.0}", state.fps);
            ui.horizontal(|ui| {
                ui.label("FPS: ");
                ui.label(fps);
            });
        }
    });
}

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    state_resource: Option<ResMut<DebugInfoState>>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.paused() {
                state.fps = 0.0;
            } else {
                let fps_diags = extract_fps(&diagnostics);
                if let Some(fps) = fps_diags {
                    state.fps = fps;
                }
            }
        }
    }
}

fn extract_fps(diagnostics: &Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}
