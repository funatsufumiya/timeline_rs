use std::time::Duration;

use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use timeline::{easing::{self, EasingFunction, EasingType}, Keyframe, Timeline};
// use egui_dropdown::DropDownBox;
// use lazy_static::lazy_static;

fn s(dur: f32) -> Duration {
    Duration::from_secs_f32(dur)
}

fn create_timeline() -> Timeline<f32> {
    let mut tl = Timeline::<f32>::new();
    tl.new_track::<f32>("x");
    tl.new_track::<f32>("y");

    let tx = tl.get_track_mut("x").unwrap();
    tx
        .add_keyframe(Keyframe {
            time: s(0.0),
            value: 0.0,
            easing_function: EasingFunction::Quintic,
            easing_type: EasingType::In,
        })
        .add_keyframe(Keyframe {
            time: s(1.0),
            value: 0.5,
            easing_function: EasingFunction::Bounce,
            easing_type: EasingType::Out,
        })
        .add_keyframe(Keyframe {
            time: s(2.0),
            value: 1.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });

    let ty = tl.get_track_mut("y").unwrap();
    ty
        .add_keyframe(Keyframe {
            time: s(0.0),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::Out,
        })
        .add_keyframe(Keyframe {
            time: s(0.5),
            value: 0.0,
            easing_function: EasingFunction::Sine,
            easing_type: EasingType::In,
        })
        .add_keyframe(Keyframe {
            time: s(0.75),
            value: 0.3,
            easing_function: EasingFunction::Sine,
            easing_type: EasingType::Out,
        })
        .add_keyframe(Keyframe {
            time: s(1.0),
            value: 0.0,
            easing_function: EasingFunction::Sine,
            easing_type: EasingType::Out,
        })
        .add_keyframe(Keyframe {
            time: s(1.20),
            value: 1.0,
            easing_function: EasingFunction::Sine,
            easing_type: EasingType::In,
        })
        .add_keyframe(Keyframe {
            time: s(1.3),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });

    tl
}

#[derive(Resource)]
struct TimelineData {
    pub timeline: Timeline<f32>,
    pub t : f32,
    pub x : f32,
    pub y : f32,
    pub looped: bool,
}

impl Default for TimelineData {
    fn default() -> Self {
        TimelineData {
            timeline: create_timeline(),
            t: 0.0,
            x: 0.0,
            y: 0.0,
            looped: true,
        }
    }
}


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(TimelineData::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_egui)
        .add_systems(Update, update)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_egui (
    mut contexts: EguiContexts,
    mut data: ResMut<TimelineData>,
) {
    egui::Window::new("Easing").show(contexts.ctx_mut(), |ui| {
        // display looped as checkbox
        ui.checkbox(&mut data.looped, "Looped");

        let duration: f32 = data.timeline.get_max_duration().as_secs_f32();

        // display t as slider
        ui.add(egui::Slider::new(&mut data.t, 0.0..=duration).text("t"));         

        // display x as slider
        ui.add(egui::Slider::new(&mut data.x, 0.0..=1.0).text("x"));
        
        // display y as slider
        ui.add(egui::Slider::new(&mut data.y, 0.0..=1.0).text("y"));
    });
}

fn update(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut data: ResMut<TimelineData>,
) {
    let duration: f32 = data.timeline.get_max_duration().as_secs_f32();

    data.t += time.delta_seconds();

    if data.looped {
        if data.t > duration {
            data.t = 0.0;
        }
    } else {
        if data.t > duration {
            // data.t = duration;

            // do nothing

        }
    }

    let vx = data.timeline.get_value("x", Duration::from_secs_f32(data.t));
    let vy = data.timeline.get_value("y", Duration::from_secs_f32(data.t));
    data.x = vx;
    data.y = vy;

    let length = 400.0;
    let height = 100.0;
    let x = vx * length - length / 2.0;
    let y = vy * height;

    // draw circle_2d
    gizmos.circle_2d(Vec2::new(x, y), 30.0, Color::RED);

    // draw base line (green)
    let ny = -60.0;
    gizmos.line_2d(Vec2::new(-length / 2.0, ny), Vec2::new(length / 2.0, ny), Color::GREEN);

    // separator lines (start and end, green)
    gizmos.line_2d(Vec2::new(-length / 2.0, ny - 10.0), Vec2::new(-length / 2.0, ny + 10.0), Color::GREEN);
    gizmos.line_2d(Vec2::new(length / 2.0, ny - 10.0), Vec2::new(length / 2.0, ny + 10.0), Color::GREEN);

}