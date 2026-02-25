use bevy::{prelude::*, app::AppExit};
use crate::repetitive_code::*;
use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_systems(
            (
                interact_with_play_button,
                interact_with_quit_button
            ).in_set(OnUpdate(AppState::MainMenu))
        )
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

//try adding some random battle in the background
//Components

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct MusicJockey;

//Layout

pub fn spawn_main_menu(mut commands : Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands : Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single(){
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands : &mut Commands, asset_server: &Res<AssetServer>) -> Entity{
    let main_menu_entity = commands
        .spawn((
            NodeBundle{
                style : MAIN_MENU_STYLE,
                ..default()

            },
            MainMenu{},
        ))
        .with_children(|parent|{
            //title
            parent.spawn(NodeBundle{
                style : TITLE_STYLE,
                ..default()
            })
            .with_children(|parent|{
                //text

                parent.spawn(TextBundle{
                    text : Text{
                        sections: vec![
                            TextSection::new("Cell War",
                            TextStyle{
                                font : asset_server.load("Fonts/FiraMono-Medium.ttf"),
                                font_size :  64.0,
                                color : Color::WHITE   
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }); 
            });
            //playbutton
            parent.spawn((
                ButtonBundle{   
                    style: BUTTON_STYLE,
                    background_color : NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton{},
            ))
            .with_children(|parent|{
                parent.spawn(TextBundle{
                    text : Text{
                        sections : vec![
                            TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                    },
                    ..default()
                });
            });
            //quibutton
            parent.spawn((
                ButtonBundle{   
                    style: BUTTON_STYLE,
                    background_color : NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton{},
            ))
            .with_children(|parent|{
                parent.spawn(TextBundle{
                    text : Text{
                        sections : vec![
                            TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server)
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                    },
                    ..default()
                });
            });
        })
        .id();
    main_menu_entity
}


//Interactions

pub fn interact_with_play_button(mut button_query : Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>, mut app_state_next_state : ResMut<NextState<AppState>>){
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction{
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game)
            },
            Interaction::Hovered => {*background_color = HOVERED_BUTTON_COLOR.into()},
            Interaction::None =>  {*background_color = NORMAL_BUTTON_COLOR.into()},
        }
    }
}

pub fn interact_with_quit_button(mut button_query : Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>, mut app_exit_event_writer : EventWriter<AppExit>){
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction{
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit)
            },
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None =>  *background_color = NORMAL_BUTTON_COLOR.into()
        }
    }

}

//other systems
