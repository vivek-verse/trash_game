#![allow(unused)]

use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    player: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Trash Game!".to_string(),
                width: 600.0,
                height: 600.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();

    let (win_w, win_h) = (window.width(), window.height());

    window.set_position(MonitorSelection::Current, IVec2::new(680, 0));

    let win_size = WinSize { w: win_w, h: win_h };

    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };

    commands.insert_resource(game_textures);
}
