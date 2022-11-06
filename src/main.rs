use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

// CONSTANTS
const TILE_SIZE: f32 = 25.0;

// ENTITIES
// Uncomment this when eventually adding states
// struct BoardEntityResource {
//     board_entity: Entity,
// }

// COMPONENTS
#[derive(Component)]
struct BoardCoordinates {
    x: i32,
    y: i32,
}

struct BoardTileEntities {
    // tiles: [Entity; 200],
    tile_entities: Vec<Entity>,
}

#[derive(Debug, Copy, Clone, Component, Inspectable)]
struct Tile {
    tile_state: TileState,
    // coordinates: BoardCoordinates,
}

#[derive(Debug, Copy, Clone, Component, Inspectable)]
enum TileState {
    Empty,
    Filled(i32),
}

// SYSTEMS
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // Engine setup
        .add_startup_system_set(
            SystemSet::new()
                .with_system(set_title)
                .with_system(setup_camera),
        )
        // Game setup
        .add_startup_system_set(SystemSet::new().with_system(spawn_tiles))
        .add_startup_system_set(
            SystemSet::new()
                .with_system(arrange_tiles)
                .after(spawn_tiles),
        )
        // .add_system(spawn_block)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::steps_per_second(5.0))
        //         .with_system(fall_blocks),
        // )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(5.0))
                .with_system(update_tile_numbers),
        )
        .add_system_set(
            SystemSet::new()
                .with_system(update_board_colours)
                .after(update_tile_numbers),
        )
        // Debug
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn set_title(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title("Bevytris".to_string());
}

fn spawn_tiles(mut commands: Commands) {
    let board_entity = commands
        .spawn()
        .insert(Name::new("Board"))
        .insert_bundle(SpatialBundle {
            visibility: Visibility::visible(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            // Create tiles
            for x in 0..10 {
                for y in 0..20 {
                    let s = SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            (TILE_SIZE + 5.0) * (x as f32),
                            (TILE_SIZE + 5.0) * (y as f32),
                            1.0,
                        ),
                        ..Default::default()
                    };
                    parent
                        .spawn_bundle(s)
                        .insert(Name::new(format!("Tile ({}, {})", x, y)))
                        .insert(Tile {
                            tile_state: TileState::Empty,
                        })
                        .insert(BoardCoordinates { x, y });
                }
            }
        })
        .id();

    // Uncomment this when eventually adding states
    // commands.insert_resource(BoardEntityResource { board_entity });
}

fn arrange_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut BoardCoordinates), With<Tile>>,
) {
    let mut i: usize = 0;
    let mut tile_entities: Vec<Entity> = Vec::new();
    for (entity, mut transform, mut board_coordinates) in query.iter_mut() {
        let x = i / 20;
        let y = i % 20;

        transform.translation = Vec3::new(
            (TILE_SIZE + 5.0) * (x as f32),
            (TILE_SIZE + 5.0) * (y as f32),
            1.0,
        );

        board_coordinates.x = x as i32;
        board_coordinates.y = y as i32;

        tile_entities.push(entity);

        i += 1;
    }

    commands.insert_resource(BoardTileEntities { tile_entities });
}

// fn create_board_entity_resource(mut commands: Commands, query: Query<&Tile>) {
//     let mut tile_entities = Vec::new();
//     for tile_entity in query.iter() {
//         info!("Here 2");
//         tile_entities.push(tile_entity)
//     }

//     info!("{:?}", tile_entities);
//     commands.insert_resource(BoardTileEntities { tile_entities });
// }

fn update_board_colours(mut query: Query<(&Tile, &mut Sprite)>) {
    for (tile, mut sprite) in query.iter_mut() {
        match tile.tile_state {
            TileState::Empty => sprite.color = Color::BLACK,
            TileState::Filled(c) => match c {
                1 => sprite.color = Color::RED,
                2 => sprite.color = Color::GREEN,
                3 => sprite.color = Color::BLUE,
                _ => sprite.color = Color::PURPLE,
            },
        }
    }
}

fn update_tile_numbers(mut query: Query<&mut Tile>, tile_entities: Res<BoardTileEntities>) {
    info!("Here");
    info!("{:?}", tile_entities.tile_entities);
    for tile_entity in tile_entities.tile_entities.iter() {
        info!("Here 2");
        if let Ok(mut current_tile) = query.get_mut(*tile_entity) {
            info!("Herer 3");
            current_tile.tile_state = match current_tile.tile_state {
                TileState::Empty => TileState::Filled(1),
                TileState::Filled(c) => TileState::Filled((c + 1) % 3 + 1),
            }
        }
    }
}
