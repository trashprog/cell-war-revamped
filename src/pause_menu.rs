use bevy::{prelude::*, app::AppExit};
use crate::repetitive_code::*;
use super::{AppState, SimulationState};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // Systems
            .add_systems(Update, (interact_with_resume_button, interact_with_main_menu_button, interact_with_quit_button,).run_if(in_state(SimulationState::Paused)))
            // OnExit Systems
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu)

            .add_systems(OnEnter(AppState::MainMenu), despawn_pause_menu);  // on entering  mainmenu, despawn the pause menu
    }
}


//Components

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct QuitButton;


//Layout

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>,) {
    if let Ok(pause_menu_entity) = pause_menu_query.single() {
        commands.entity(pause_menu_entity).despawn();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            // NodeBundle {
            //     style: PAUSE_MENU_STYLE,
            //     z_index: ZIndex::Local(1), // See Ref. 1
            //     ..default()
            // },
            get_pause_menu_style(),
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(build_container())
                .with_children(|parent| {
                    // Title
                    let text_components = get_title_text_components(asset_server, "Pause Menu");
                    parent.spawn(
                        (
                        text_components.0,
                        text_components.1,
                        text_components.2,
                    )
                    );
                    // parent.spawn(TextBundle {
                    //     text: Text {
                    //         sections: vec![TextSection::new(
                    //             "Pause Menu",
                    //             get_title_text_style(&asset_server),
                    //         )],
                    //         alignment: TextAlignment::Center,
                    //         ..default()
                    //     },
                    //     ..default()
                    // });
                    // Resume Button
                    parent
                        .spawn((
                            build_button(),
                            ResumeButton{}
                            // ButtonBundle {
                            //     style: BUTTON_STYLE,
                            //     background_color: NORMAL_BUTTON_COLOR.into(),
                            //     ..default()
                            // },
                            // ResumeButton {},
                        ))
                        .with_children(|parent| {
                            let text_components = get_button_text_components(asset_server, "Resume", 32.0);
                            parent.spawn(
                                (
                                text_components.0,
                                text_components.1,
                                text_components.2,
                                )
                            );
                            // parent.spawn(TextBundle {
                            //     style: Style { ..default() },
                            //     text: Text {
                            //         sections: vec![TextSection::new(
                            //             "Resume",
                            //             get_button_text_style(&asset_server),
                            //         )],
                            //         alignment: TextAlignment::Center,
                            //         ..default()
                            //     },
                            //     ..default()
                            // });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            build_button(),
                            MainMenuButton{}
                            // ButtonBundle {
                            //     style: BUTTON_STYLE,
                            //     background_color: NORMAL_BUTTON_COLOR.into(),
                            //     ..default()
                            // },
                            // MainMenuButton {},
                        ))
                        .with_children(|parent| {

                            let text_components = get_button_text_components(asset_server, "Main Menu", 32.0);
                            parent.spawn(
                                (
                                text_components.0,
                                text_components.1,
                                text_components.2,
                                )
                            );
                            // parent.spawn(TextBundle {
                            //     style: Style { ..default() },
                            //     text: Text {
                            //         sections: vec![TextSection::new(
                            //             "Main Menu",
                            //             get_button_text_style(&asset_server),
                            //         )],
                            //         alignment: TextAlignment::Center,
                            //         ..default()
                            //     },
                            //     ..default()
                            // });
                        });
                    // Quit Button
                    parent
                        .spawn((
                            build_button(),
                            QuitButton{}
                            // ButtonBundle {
                            //     style: BUTTON_STYLE,
                            //     background_color: NORMAL_BUTTON_COLOR.into(),
                            //     ..default()
                            // },
                            // QuitButton {},
                        ))
                        .with_children(|parent| {
                            let text_components = get_button_text_components(asset_server, "Quit", 32.0);
                            parent.spawn(
                                (
                                text_components.0,
                                text_components.1,
                                text_components.2,
                                )
                            );
                            // parent.spawn(TextBundle {
                            //     style: Style { ..default() },
                            //     text: Text {
                            //         sections: vec![TextSection::new(
                            //             "Quit",
                            //             get_button_text_style(&asset_server),
                            //         )],
                            //         alignment: TextAlignment::Center,
                            //         ..default()
                            //     },
                            //     ..default()
                            // });
                        });
                });
        })
        .id();
    pause_menu_entity
}


//Interactions


pub fn interact_with_resume_button(mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<ResumeButton>),>, mut simulation_state_next_state: ResMut<NextState<SimulationState>>,) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                simulation_state_next_state.set(SimulationState::Running);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_main_menu_button(mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<MainMenuButton>),>,mut app_state_next_state: ResMut<NextState<AppState>>,) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(mut app_exit_event_writer: EventWriter<AppExit>, mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<QuitButton>),>,) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}