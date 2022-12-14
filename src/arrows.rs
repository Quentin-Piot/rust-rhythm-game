use crate::consts::{AppState, SPAWN_POSITION, TARGET_POSITION, THRESHOLD};
use crate::score::ScoreResource;
use crate::time::ControlledTime;
use crate::types::{Directions, SongConfig, Speed};
use bevy::prelude::*;

#[derive(Resource)]
struct ArrowMaterialResource {
    red_texture: Handle<Image>,
    blue_texture: Handle<Image>,
    green_texture: Handle<Image>,
    border_texture: Handle<Image>,
}

impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture: red_handle.into(),
            blue_texture: blue_handle.into(),
            green_texture: green_handle.into(),
            border_texture: border_handle.into(),
        }
    }
}

#[derive(Component)]
struct Arrow {
    speed: Speed,
    direction: Directions,
}

fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<ControlledTime>,
) {
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;

    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to speed
            let texture = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            // Rotate the arrow acording to direction
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
            commands
                .spawn(SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(140., 140.)),
                        ..default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
        } else {
            break;
        }
    } // Remove the arrows we have spawned from the list
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

fn move_arrows(time: Res<ControlledTime>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);
        }
    }
}

#[derive(Component)]
struct TargetArrow;

fn setup_target_arrows(mut commands: Commands, materials: Res<ArrowMaterialResource>) {
    use Directions::*;
    let directions = [Up, Down, Left, Right];

    for direction in directions.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn(SpriteBundle {
                texture: materials.border_texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(140., 140.)),
                    ..default()
                },
                transform,
                ..Default::default()
            })
            .insert(TargetArrow);
    }
}

fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();
            score.increase_correct(TARGET_POSITION - pos);
        }

        if pos >= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();
            score.increase_fails();
        }
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowMaterialResource>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_target_arrows))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_arrows)
                    .with_system(move_arrows)
                    .with_system(despawn_arrows),
            );
    }
}
