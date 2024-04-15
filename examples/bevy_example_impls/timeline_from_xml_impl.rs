use std::time::Duration;

use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use timeline::{easing::{self, EasingFunction, EasingType}, loader::TimelineXMLLoader, Keyframe, Timeline, TimelineTrack, Track, TrackGetter};
// use egui_dropdown::DropDownBox;
// use lazy_static::lazy_static;

fn s(dur: f32) -> Duration {
    Duration::from_secs_f32(dur)
}

fn create_timeline() -> Timeline {
    let mut tl = Timeline::new();

    let xml_x = r#"
    <keyframes>
    <key>
        <easefunc>0</easefunc>
        <easetype>0</easetype>
        <time>00:00:00:524</time>
        <value>0.375000000</value>
    </key>
    <key>
        <easefunc>0</easefunc>
        <easetype>0</easetype>
        <time>00:00:00:826</time>
        <value>0.408691406</value>
    </key>
    <key>
        <easefunc>0</easefunc>
        <easetype>0</easetype>
        <time>00:00:01:034</time>
        <value>0.324999988</value>
    </key>
    <key>
        <easefunc>0</easefunc>
        <easetype>0</easetype>
        <time>00:00:01:459</time>
        <value>0.777343750</value>
    </key>
    <key>
        <easefunc>4</easefunc>
        <easetype>0</easetype>
        <time>00:00:02:123</time>
        <value>0.330175757</value>
    </key>
</keyframes>
    "#;

    let xml_y = r#"
<keyframes>
    <key>
        <easefunc>0</easefunc>
        <easetype>0</easetype>
        <time>00:00:00:643</time>
        <value>0.585546851</value>
    </key>
    <key>
        <easefunc>4</easefunc>
        <easetype>2</easetype>
        <time>00:00:00:826</time>
        <value>0.141503930</value>
    </key>
    <key>
        <easefunc>1</easefunc>
        <easetype>1</easetype>
        <time>00:00:01:594</time>
        <value>0.443359375</value>
    </key>
    <key>
        <easefunc>0</easefunc>
        <easetype>2</easetype>
        <time>00:00:02:033</time>
        <value>0.400390625</value>
    </key>
    <key>
        <easefunc>4</easefunc>
        <easetype>0</easetype>
        <time>00:00:02:260</time>
        <value>0.586718738</value>
    </key>
</keyframes>"#;

    // NOTE: you can also use json

    tl.load_xml_str::<f32>("x", xml_x).unwrap();
    tl.load_xml_str::<f32>("y", xml_y).unwrap();

    tl
}

#[derive(Resource)]
struct TimelineData {
    pub timeline: Timeline,
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

    let vx = data.timeline.get_value("x", Duration::from_secs_f32(data.t)).into();
    let vy = data.timeline.get_value("y", Duration::from_secs_f32(data.t)).into();
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