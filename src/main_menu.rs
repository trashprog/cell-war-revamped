use bevy::{prelude::*, app::AppExit};
use crate::{SimulationState, repetitive_code::*};
use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(Update, (interact_with_play_button, interact_with_quit_button).run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
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


//Layout

pub fn spawn_main_menu(mut commands : Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands : Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.single(){
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands : &mut Commands, asset_server: &Res<AssetServer>) -> Entity{
    let main_menu_entity = commands
        .spawn((
            get_main_menu_style(),
            MainMenu{},
            ZIndex(1) // See Ref. 1
        ))
        .with_children(|parent|{
            //title
            parent.spawn(get_title_style())
            .with_children(|parent|{
                //text
                let text_components = get_title_text_components(asset_server, "Cell War");
                parent.spawn(
                    (
                    text_components.0,
                    text_components.1,
                    text_components.2,
                )
                );
            });
            //playbutton
            parent.spawn(
                (
                    build_button(),
                    PlayButton{}
                )
        )
            .with_children(|parent|{

                let text_components = get_button_text_components(asset_server, "Play", 32.0);
                parent.spawn(
                    (
                    text_components.0,
                    text_components.1,
                    text_components.2,
                )
            );
            });
            //quibutton
            parent.spawn((
                QuitButton{},
                build_button()
            )
        )
            .with_children(|parent|{
                let text_components = get_button_text_components(asset_server, "Quit", 32.0);
                parent.spawn(
                    (
                    text_components.0,
                    text_components.1,
                    text_components.2,
                )
            );
            });
        })
        .id();
    main_menu_entity
}


//Interactions

pub fn interact_with_play_button(mut button_query : Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>, mut app_state_next_state : ResMut<NextState<AppState>>){
    if let Ok((interaction, mut background_color)) = button_query.single_mut(){
        match *interaction{
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
                println!("Transitioning to Game State");
            },
            Interaction::Hovered => {*background_color = HOVERED_BUTTON_COLOR.into()},
            Interaction::None =>  {*background_color = NORMAL_BUTTON_COLOR.into()},
        }
    }
}

pub fn interact_with_quit_button(mut button_query : Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>, mut app_exit_event_writer : EventWriter<AppExit>){
    if let Ok((interaction, mut background_color)) = button_query.single_mut(){
        match *interaction{
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.write(AppExit::Success);
            },
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None =>  *background_color = NORMAL_BUTTON_COLOR.into()
        }
    }

}

//other systems
