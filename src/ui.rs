use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
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
                            parent // QUIT BUTTON
                                .spawn_bundle(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text("PLAY", &asset_server));
                                });

                            parent // QUIT BUTTON
                                .spawn_bundle(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text("STOP", &asset_server));
                                });

                            parent // QUIT BUTTON
                                .spawn_bundle(build_classic_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text("QUIT", &asset_server));
                                });
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
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
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