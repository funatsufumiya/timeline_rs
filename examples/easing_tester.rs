use bevy::{prelude::*, tasks::ParallelSlice};

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use timeline_rs::easing;
use lazy_static::lazy_static;

lazy_static! {
    static ref EASING_FUNCTIONS: Vec<(&'static str, easing::EasingFunction)> = easing_functions_list();
    static ref EASING_TYPES: Vec<(&'static str, easing::EasingType)> = easing_type_list();
}

#[derive(Resource)]
struct EasingInfo {
    pub easing_function: easing::EasingFunction,
    pub easing_function_selected_name: &'static str,
    pub easing_type: easing::EasingType,
    pub easing_type_selected_name: &'static str,
    pub t : f32,
    pub v : f32,
    pub is_go_and_back: bool,
}

impl Default for EasingInfo {
    fn default() -> Self {
        EasingInfo {
            easing_function: easing::EasingFunction::Linear,
            easing_type: easing::EasingType::In,
            easing_function_selected_name: "Linear",
            easing_type_selected_name: "In",
            t: 0.0,
            v: 0.0,
            is_go_and_back: false,
        }
    }
}

fn easing_functions_list() -> Vec<(&'static str, easing::EasingFunction)> {
    vec![
        ("Linear", easing::EasingFunction::Linear),
        ("Sine", easing::EasingFunction::Sine),
        ("Circular", easing::EasingFunction::Circular),
        ("Quadratic", easing::EasingFunction::Quadratic),
        ("Cubic", easing::EasingFunction::Cubic),
        ("Quartic", easing::EasingFunction::Quartic),
        ("Quintic", easing::EasingFunction::Quintic),
        ("Exponential", easing::EasingFunction::Exponential),
        ("Back", easing::EasingFunction::Back),
        ("Bounce", easing::EasingFunction::Bounce),
        ("Elastic", easing::EasingFunction::Elastic),
    ]
}

fn easing_type_list() -> Vec<(&'static str, easing::EasingType)> {
    vec![
        ("In", easing::EasingType::In),
        ("Out", easing::EasingType::Out),
        ("InOut", easing::EasingType::InOut),
    ]
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(EasingInfo::default())
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
    mut info: ResMut<EasingInfo>,
) {
    egui::Window::new("Easing").show(contexts.ctx_mut(), |ui| {
        // display go_and_back as checkbox
        ui.checkbox(&mut info.is_go_and_back, "Go and back");

        // display t as slider
        ui.add(egui::Slider::new(&mut info.t, 0.0..=1.0).text("t"));         

        // display v as slider
        ui.add(egui::Slider::new(&mut info.v, 0.0..=1.0).text("v"));

        // select easing function (dropdown)
        egui::ComboBox::from_label("function")
            .selected_text(info.easing_function_selected_name)
            .show_ui(ui, |ui| {
                EASING_FUNCTIONS.iter().for_each(|(name, f)| { // Use the cloned vector
                    if (ui.selectable_value(&mut info.easing_function, *f, *name)
                        .clicked()){
                        info.easing_function_selected_name = *name;
                    }
                });
            });

        // select easing type (dropdown)
        egui::ComboBox::from_label("type")
            .selected_text(info.easing_type_selected_name)
            .show_ui(ui, |ui| {
                EASING_TYPES.iter().for_each(|(name, t)| { // Use the cloned vector
                    // ui.selectable_value(&mut info.easing_type, *t, *name);
                    if (ui.selectable_value(&mut info.easing_type, *t, *name)
                        .clicked()){
                        info.easing_type_selected_name = *name;
                    }
                });
            });
    });
}

fn update(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut info: ResMut<EasingInfo>,
) {
    let duration: f32 = 1.0;
    if info.is_go_and_back {
        let s = ((time.elapsed_seconds_f64()) % (duration as f64 * 2.0)) as f32;
        info.t = s;
        if s > duration {
            info.t = 2.0 * duration - s;
        }
    } else {
        let s = ((time.elapsed_seconds_f64()) % (duration as f64)) as f32;
        info.t = s;
    }

    info.v = easing::easing(
        info.t,
        0.0,
        1.0,
        duration,
        info.easing_function,
        info.easing_type,
    );

    let v = info.v;
    let length = 200.0;
    let x = v * length - length / 2.0;
    let y = 0.0;

    // draw circle_2d
    gizmos.circle_2d(Vec2::new(x, y), 30.0, Color::RED);

    // draw base line (green)
    let ny = -60.0;
    gizmos.line_2d(Vec2::new(-length / 2.0, ny), Vec2::new(length / 2.0, ny), Color::GREEN);

    // separator lines (start and end, green)
    gizmos.line_2d(Vec2::new(-length / 2.0, ny - 10.0), Vec2::new(-length / 2.0, ny + 10.0), Color::GREEN);
    gizmos.line_2d(Vec2::new(length / 2.0, ny - 10.0), Vec2::new(length / 2.0, ny + 10.0), Color::GREEN);

}