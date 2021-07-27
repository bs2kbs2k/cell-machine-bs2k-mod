use bevy::prelude::*;
use bevy::window::WindowMode;

mod plugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Cell Machine".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(plugin::CellMachinePlugin)
        .add_plugin(bevy_easings::EasingsPlugin)
        .run();
}
