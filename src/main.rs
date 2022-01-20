use ui::*;
use input::*;
use simulation::*;
use bevy::prelude::*;

mod ui;
mod simulation;
mod input;

const GRID_SIZE: i32 = 100;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024f32,
            height: 720f32,
            title: "Game Of Life".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SimulationPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(MainMenuPlugin)
        .run();
}