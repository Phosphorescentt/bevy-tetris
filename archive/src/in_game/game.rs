use bevy::prelude::*;

// #[derive(Component)]
// struct Coordinates {
//     x: i32,
//     y: i32,
// }

#[derive(Component)]
pub struct Square;

pub struct SquareEntity {
    square_entity: Entity,
}

#[derive(Component)]
pub struct Board {
    tiles: Vec<i32>,
}

pub struct GameHolder {
    board_entity: Entity,
    sidebar_entity: Entity,
}

fn setup_board(mut commands: Commands) {
    let tiles = vec![0_i32, 10 * 20];
    let board_entity = commands.spawn().with_children(move |parent| {});
}

pub fn make_square(mut commands: Commands) {
    let square_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 100.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Square)
        .id();

    commands.insert_resource(SquareEntity { square_entity });
}

pub fn move_square_on_tick(mut square_transforms: Query<&mut Transform, With<Square>>) {
    for mut transform in square_transforms.iter_mut() {
        transform.translation.x += 10.;
    }
}

pub fn remove_square(mut commands: Commands, square: Res<SquareEntity>) {
    commands.entity(square.square_entity).despawn_recursive();
}

pub fn cleanup_game(mut commands: Commands, game_holder: Res<GameHolder>) {
    commands
        .entity(game_holder.board_entity)
        .despawn_recursive();
    commands
        .entity(game_holder.sidebar_entity)
        .despawn_recursive();
}
