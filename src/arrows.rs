use bevy::prelude::*;
use rand::Rng;

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

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
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
struct Arrow;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let num: f32 = -(rand::thread_rng().gen::<f32>() * 300.);
    let num2: f32 = -(rand::thread_rng().gen::<f32>() * 300.);

    commands
        .spawn(SpriteBundle {
            texture: materials.green_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(140., 140.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(num, num2, 10.)),
            ..Default::default()
        })
        .insert(Arrow);
}

fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize Resources
            .init_resource::<ArrowMaterialResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_system(spawn_arrows)
            .add_system(move_arrows);
    }
}
