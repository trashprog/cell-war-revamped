use bevy::{prelude::*, app::AppExit};
use crate::{repetitive_code::*, FinalScore};
use super::AppState;


pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(Update,
                ( interact_with_restart_button, interact_with_main_menu_button, interact_with_quit_button, update_final_score_text, final_score_checker
                )
                .run_if(in_state(AppState::GameOver)),
            )
            // // OnExit State Systems
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}

//Components


#[derive(Component)]
pub struct GameOverMenu {}

#[derive(Component)]
pub struct FinalScoreText {}

#[derive(Component)]
pub struct RestartButton {}

#[derive(Component)]
pub struct MainMenuButton {}

#[derive(Component)]
pub struct QuitButton {}

#[derive(Component)]
pub struct CommentText {}

//Layout

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(mut commands: Commands, game_over_menu_query: Query<Entity, With<GameOverMenu>>,) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.single() {
        commands.entity(game_over_menu_entity).despawn();
    }
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            get_game_over_menu_style(),
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    get_base_button_style()
            )
                .with_children(|parent| {
                    // Title
                    let text_components = get_title_text_components(asset_server, "Game Over");
                    parent.spawn(
                        (
                        text_components.0,
                        text_components.1,
                        text_components.2,
                    ));
                    // Final Score Text
                    let text_components = get_title_text_components(asset_server, "Your Final Score:");
                    parent.spawn((
                        (
                        text_components.0,
                        text_components.1,
                        text_components.2,
                        ),
                        FinalScoreText {}
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            build_button(),
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                           let text_components = get_button_text_components(asset_server, "Restart", 32.0);
                            parent.spawn(
                                (
                                text_components.0,
                                text_components.1,
                                text_components.2,
                                )
                            );
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            build_button(),
                            MainMenuButton {},
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
                        });
                    // Quit Button
                    parent
                        .spawn((
                            build_button(),
                            QuitButton {},
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
                        });
                });
        })
        .id();

    game_over_menu_entity
}


//Interactions

pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
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

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
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

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
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

//Updates

pub fn update_final_score_text(final_score : Res<FinalScore>, mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    let time_alive = final_score.scores.last().unwrap().1;
    let base_level = final_score.scores.last().unwrap().0;
    for mut text in text_query.iter_mut(){
        text.0 = format!("Time alive: {}\nBase level: {}", time_alive, base_level);
    }
}

pub fn final_score_checker(final_score : Res<FinalScore>, mut text_query: Query<&mut Text, With<CommentText>>){
    let time_survived = final_score.scores.last().unwrap().1;
    for mut text in text_query.iter_mut(){
        let result = match time_survived {
            time_survived if time_survived < 120 => "Better luck next time",
            time_survived if time_survived < 240 => "Nice",
            time_survived if time_survived < 600 => "Fantastic score",
            _ => "You are awesome" 
        };
        text.0 = format!("{}", result)

    }
    
    
}