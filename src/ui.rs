use bevy::{prelude::*, diagnostic::DiagnosticsStore};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;

use crate::{time::{PhysicsTime, PhysicsTimeExt, PhysicsTimeMode}, RestartEvent};

const ICON_RESTART: char = '\u{E800}';
const ICON_PAUSE:   char = '\u{E801}';
const ICON_PLAY:    char = '\u{E802}';
const ICON_FASTFWD: char = '\u{E803}';
const ICON_STEP:    char = '\u{E804}';

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiSettings>()
            .register_type::<UiSettings>()
            .add_systems(Startup, init_icon_font)
            .add_systems(Update, display_custom_window);
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct UiSettings {
    enabled: bool,
    margin_top: f32,
    icon_font_size: f32,
    info_font_size: f32,
    line_height: f32,
    spacing: f32,
    spacing_before: f32,
    spacing_after: f32,
    height: f32,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            margin_top: 2.,
            icon_font_size: 22.,
            info_font_size: 16.,
            line_height: 15.,
            spacing: 8.,
            spacing_before: 15.,
            spacing_after: 15.,
            height: 20.,
        }
    }
}

fn init_icon_font(mut contexts: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "sim_icons".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/fontello.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("sim_icons".into()))
        .or_default()
        .push("sim_icons".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}

fn display_custom_window(
    mut egui_contexts: EguiContexts,
    settings: ResMut<UiSettings>,
    mut time: ResMut<PhysicsTime>,
    mut restart_events: EventWriter<RestartEvent>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_fps: Local<f64>,
) {
    if !settings.enabled { return; }
    let ctx = egui_contexts.ctx_mut();

    let font = egui::FontId::new(
        settings.icon_font_size,
        egui::FontFamily::Name("sim_icons".into()),
    );

    egui::Window::new("widget")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., settings.margin_top))
        .title_bar(false)
        .auto_sized()
        .show(ctx, |ui| {
            ui.set_height(settings.height);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let elapsed = time.elapsed();
                ui.label(
                    egui::RichText::new(
                        format!(
                            "{:01}:{:02}:{:02}:{:03}",
                            elapsed.as_secs() / 3600,
                            (elapsed.as_secs() % 3600) / 60,
                            elapsed.as_secs() % 60,
                            elapsed.subsec_millis(),
                        )
                    ).size(settings.info_font_size),
                );

                let active_icon = match time.context().mode {
                    PhysicsTimeMode::Paused => ICON_PAUSE,
                    PhysicsTimeMode::OneTick => ICON_STEP,
                    PhysicsTimeMode::Running => {
                        if time.context().speed == 1. {
                            ICON_PLAY
                        } else {
                            ICON_FASTFWD
                        }
                    }
                };

                ui.add_space(settings.spacing_before);
                for (idx, icon) in [ICON_RESTART, ICON_PAUSE, ICON_STEP, ICON_PLAY, ICON_FASTFWD].into_iter().enumerate() {
                    if idx > 0 { ui.add_space(settings.spacing); }

                    let base_color = if active_icon == icon {
                        if icon == ICON_PAUSE {
                            egui::Color32::from_rgb(255, 128, 128)
                        } else {
                            egui::Color32::from_rgb(128, 255, 128)
                        }
                    } else {
                        egui::Color32::from_gray(150)
                    };

                    ui.style_mut().visuals.widgets.inactive.fg_stroke.color = base_color.gamma_multiply(0.7);
                    ui.style_mut().visuals.widgets.hovered.fg_stroke.color = base_color.gamma_multiply(0.9);
                    ui.style_mut().visuals.widgets.active.fg_stroke.color = base_color;

                    let text = egui::RichText::new(icon).font(font.clone()).line_height(Some(settings.line_height));
                    let label = egui::Label::new(text).sense(egui::Sense::click());

                    if ui.add(label).clicked() {
                        match icon {
                            ICON_RESTART => {
                                restart_events.send(RestartEvent {
                                    mode_after_restart: time.context().mode,
                                    speed_after_restart: time.context().speed,
                                });
                                time.pause();
                            },
                            ICON_PAUSE => time.pause(),
                            ICON_STEP => time.step(),
                            ICON_PLAY => {
                                time.set_speed(1.);
                                time.run();
                            }
                            ICON_FASTFWD => {
                                time.set_speed(std::f32::INFINITY);
                                time.run();
                            }
                            _ => (),
                        }
                    }
                }
                ui.add_space(settings.spacing_after);

                let speed = match time.context().mode {
                    PhysicsTimeMode::Paused => 0.,
                    PhysicsTimeMode::OneTick => 0.,
                    PhysicsTimeMode::Running => {
                        let expected_fps = time.context().timestep.as_secs_f64().recip();
                        let measured_fps = diagnostics.get(crate::time::PHYSICS_FPS).unwrap().average().unwrap_or_default();
                        let speed_factor = time.context().speed as f64;

                        let actual_fps = last_fps.max(measured_fps);
                        *last_fps = measured_fps;

                        let mut result = actual_fps / expected_fps;
                        if result > speed_factor * 0.95 {
                            result = speed_factor;
                        }
                        result
                    }
                };

                ui.label(egui::RichText::new(format!("{:.2}x", speed)).size(settings.info_font_size));
            });
        });
}
