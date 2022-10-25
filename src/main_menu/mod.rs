use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

struct MenuEntity {
    text_entity: Entity,
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.7, 0.35);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(run_menu)
                    .with_system(button_cosmetic_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_menu));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_entity = commands
        .spawn_bundle(TextBundle::from_section(
            "MainMenu",
            TextStyle {
                font: asset_server.load("fonts/Silkscreen-Regular.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ))
        .id();

    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/Silkscreen-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();

    commands.insert_resource(MenuEntity {
        text_entity,
        button_entity,
    });
}

fn run_menu(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        let _ = app_state.set(AppState::InGame);
        keys.clear();
    }
}

fn cleanup_menu(mut commands: Commands, menu_entity: Res<MenuEntity>) {
    commands.entity(menu_entity.text_entity).despawn_recursive();
    commands
        .entity(menu_entity.button_entity)
        .despawn_recursive();
}

fn button_cosmetic_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<State<AppState>>,
) {
    //
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "WOOOOOOO!".to_string();
                *color = PRESSED_BUTTON.into();
                let _ = app_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Play!".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
