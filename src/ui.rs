use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

pub struct GameExitEvent;

pub struct SimulationStartEvent;

pub struct SimulationStopEvent;

#[derive(Component)]
struct ClassicButton(ButtonType);

#[derive(PartialEq, Copy, Clone)]
enum ButtonType {
    Start,
    Stop,
    Exit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameExitEvent>()
            .add_event::<SimulationStartEvent>()
            .add_event::<SimulationStopEvent>()
            .add_startup_system(setup)
            .add_system(button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle { // root
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent // bottom button BG border
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                        border: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent // bottom button BG fill 
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::rgb(0.2, 0.2, 0.2).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent // PLAY BUTTON
                                .spawn_bundle(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text("PLAY", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Start));

                            parent // STOP BUTTON
                                .spawn_bundle(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text("STOP", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Stop));

                            parent // QUIT BUTTON
                                .spawn_bundle(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text("QUIT", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Exit));
                        });
                });
        });
}

fn build_classic_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: NORMAL_BUTTON.into(),
        image: UiImage(asset_server.load("sprites/button.png")),
        ..Default::default()
    }
}

fn build_classic_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/Symtext.ttf"),
                font_size: 30.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default()
        ),
        ..Default::default()
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ClassicButton),
        (Changed<Interaction>, With<Button>)>,
    mut start_writer: EventWriter<SimulationStartEvent>,
    mut stop_writer: EventWriter<SimulationStopEvent>,
    mut exit_writer: EventWriter<GameExitEvent>,
) {
    for (interaction, mut color, classic_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match classic_button.0 {
                    ButtonType::Start => {
                        start_writer.send(SimulationStartEvent);
                    },
                    ButtonType::Stop => {
                        stop_writer.send(SimulationStopEvent);
                    },
                    ButtonType::Exit => {
                        exit_writer.send(GameExitEvent);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}