use crate::consts::*;
use crate::types::load_config;
use bevy::prelude::*;

#[derive(Resource)]
struct ButtonMaterials {
    none: Color,
    normal: Color,
    font: Handle<Font>,
    normal_material: Handle<ColorMaterial>,
    pressed_material: Handle<ColorMaterial>,
    hovered_material: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        ButtonMaterials {
            none: Color::NONE,
            normal: Color::rgb(0.15, 0.15, 0.15),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            pressed_material: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            hovered_material: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            normal_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        }
    }
}

#[derive(Component)]
enum MenuButton {
    PlaySong(String),
}
impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::PlaySong(song) => format!("{}", song),
        }
    }
}

#[derive(Component)]
struct MenuUI;
fn setup_menu(mut commands: Commands, button_materials: Res<ButtonMaterials>) {
    // Make list of buttons
    let mut buttons: Vec<MenuButton> = get_songs()
        .iter()
        .map(|name| MenuButton::PlaySong(name.clone()))
        .collect();

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            background_color: BackgroundColor(button_materials.none),
            ..Default::default()
        })
        .insert(MenuUI)
        .with_children(|parent| {
            // Add all of the buttons as children
            for button in buttons {
                // Spawn a new button
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                top: Val::Auto,
                                bottom: Val::Auto,
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(button_materials.normal),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                button.name(),
                                TextStyle {
                                    font: button_materials.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..Default::default()
                        });
                    })
                    .insert(button);
            }
        });
}

fn despawn_menu(mut commands: Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed_material.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered_material.clone();
            }
            Interaction::None => {
                *material = button_materials.normal_material.clone();
            }
        }
    }
}

fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::PlaySong(song) => {
                    let config = load_config(&*format!("{}.toml", song), &asset_server);
                    commands.insert_resource(config);
                    state
                        .set(AppState::Game)
                        .expect("Couldn't switch state to Game")
                }
            };
        }
    }
}

use bevy::ui::Val::Auto;

pub fn get_songs() -> Vec<String> {
    vec![String::from("Territory")]
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(button_color_system)
                    .with_system(button_press_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_menu));
    }
}
