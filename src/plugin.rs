use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::egui::math::Rot2;

pub struct CellMachinePlugin;

impl Plugin for CellMachinePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::MainMenu)
            .add_startup_system(setup.system());
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
    CreateSandbox,
    LoadSave,
    TextureSelect,
    LevelSelect,
}

struct Materials {
    bg: Handle<ColorMaterial>,
    cell_background: Handle<ColorMaterial>,
    cell_background_editable: Handle<ColorMaterial>,
    cell_enemy: Handle<ColorMaterial>,
    cell_generator: Handle<ColorMaterial>,
    cell_mover: Handle<ColorMaterial>,
    cell_push: Handle<ColorMaterial>,
    cell_slide: Handle<ColorMaterial>,
    cell_spinner_ccw: Handle<ColorMaterial>,
    cell_spinner_cw: Handle<ColorMaterial>,
    cell_trash: Handle<ColorMaterial>,
    cell_wall: Handle<ColorMaterial>,
}

#[derive(Bundle)]
struct CellBundle<T: 'static>
where
    T: Send + Sync,
{
    transform: Transform,
    _type: T,
    material: Handle<ColorMaterial>,
    #[bundle]
    sprite: SpriteBundle,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        bg: materials.add(Color::rgb_u8(41, 41, 41).into()),
        cell_background: materials.add(asset_server.get_handle("background.png").into()),
        cell_background_editable: materials.add(asset_server.get_handle("editable.png").into()),
        cell_enemy: materials.add(asset_server.get_handle("enemy.png").into()),
        cell_generator: materials.add(asset_server.get_handle("generator.png").into()),
        cell_mover: materials.add(asset_server.get_handle("mover.png").into()),
        cell_push: materials.add(asset_server.get_handle("push.png").into()),
        cell_slide: materials.add(asset_server.get_handle("slide.png").into()),
        cell_spinner_ccw: materials.add(asset_server.get_handle("spinner_ccw.png").into()),
        cell_spinner_cw: materials.add(asset_server.get_handle("spinner_cw.png").into()),
        cell_trash: materials.add(asset_server.get_handle("trash.png").into()),
        cell_wall: materials.add(asset_server.get_handle("wall.png").into()),
    })
}
