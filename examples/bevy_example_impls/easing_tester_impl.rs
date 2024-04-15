use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
    mut gizmos: Gizmos,
) {
    // draw circle_2d
    gizmos.circle_2d(Vec2::new(0.0, 0.0), 100.0, Color::RED);

}