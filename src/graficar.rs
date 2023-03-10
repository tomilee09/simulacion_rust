
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn((
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(100.0, 50.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    },
    Direction::Up,
));

}



fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

pub fn graficar() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}