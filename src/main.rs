use bevy::{prelude::*, core::FixedTimestep};
use ui::MainMenuPlugin;

mod ui;

const SPRITE_SIZE: f32 = 32.0;
const GRID_SIZE: i32 = 100;
const CAMERA_MOVE_SPEED: f32 = 15.0;
const CAMERA_ZOOM_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.8, 0.1)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.033))
                .with_system(camera_move)
                .with_system(camera_zoom)
        )
        .add_system(cell_interaction)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Movement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

#[derive(Component)]
struct Cell {
    position: (i32, i32),
    alive: bool,
}

#[derive(Default)]
struct GenerationNumber(u32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Movement {
            plane_speed: Vec3::new(0.0, 0.0, 0.0),
            zoom_speed: 0.0,
        });

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        ..Default::default()
                    },
                    texture: asset_server.load("sprites/empty_cell.png"),
                    ..Default::default()
                })
                .insert(Transform {
                    translation: Vec3::new((x as f32) * SPRITE_SIZE, (y as f32) * SPRITE_SIZE, 0.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                })
                .insert(Cell {
                    position: (x, y),
                    alive: false,
                });
        }
    }
}

fn camera_move(
    mut camera: Query<(&mut Transform, &mut Movement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut move_direction = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }
    let move_direction = move_direction.normalize_or_zero();
    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform on main camera?!");
    movement.plane_speed = (movement.plane_speed  + move_direction)
        .clamp(
            Vec3::new(-CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED),
            Vec3::new(CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED)
        );

    transform.translation += movement.plane_speed;
}

fn camera_zoom(
    mut camera: Query<(&mut Movement, &mut OrthographicProjection), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut zoom_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = 0.01;
    }
    if keyboard_input.pressed(KeyCode::E) {
        zoom_direction = -0.01;
    }

    let (mut movement, mut orto_proj) = camera
        .iter_mut()
        .next()
        .unwrap();
    movement.zoom_speed = (movement.zoom_speed + zoom_direction).clamp(-CAMERA_ZOOM_SPEED, CAMERA_ZOOM_SPEED);
    orto_proj.scale = (orto_proj.scale + movement.zoom_speed).clamp(2.0, 6.0);

    if (orto_proj.scale - 2.0).abs() < 0.00001 || (orto_proj.scale - 8.0).abs() < 0.00001 {
        movement.zoom_speed = 0.0;
    }
}

fn cell_interaction(
    
) {

}

fn simulation_step() {

}