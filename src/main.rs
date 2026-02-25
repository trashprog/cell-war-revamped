use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

mod repetitive_code;
mod player;
mod wave;
mod bullet;
mod enemy;
mod base;
mod main_menu;
mod turret;
mod part;
mod pause_menu;
mod game_over;
mod hud;

use turret::TurretPlugin;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use base::BasePlugin;
use main_menu::MainMenuPlugin;
use wave::WavePlugin;
use player::PlayerPlugin;
use pause_menu::PauseMenuPlugin;
use game_over::GameOverMenuPlugin;
use hud::HudPlugin;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)

    //Resources
    .init_resource::<FinalScore>()
    //States
    .add_state::<AppState>()
    .add_state::<SimulationState>()

    //Events
    .add_event::<GameOver>()

     // OnEnter Systems
     .add_system(resume_simulation.in_schedule(OnEnter(AppState::Game)))
     

    //Plugins
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(TurretPlugin)
    .add_plugin(WavePlugin)
    .add_plugin(BasePlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(PauseMenuPlugin)
    .add_plugin(GameOverMenuPlugin)
    .add_plugin(HudPlugin)

    //Systems
    .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
    .add_system(transition_to_game_state)
    .add_system(transition_to_main_menu_state)
    .add_system(update_final_score)
    .add_system(exit_game)
    .add_system(handle_game_over)

    //On Exit Systems
    //.add_system(pause_simulation.in_schedule(OnExit(AppState::Game)))
    .run();
}


#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}


pub struct GameOver{
    pub time_alive : u64,
    pub base_level : i64
}

#[derive(Resource)]

pub struct FinalScore{
    pub scores: Vec<(i64, u64)>,
}

impl Default for FinalScore {
    fn default() -> FinalScore {
        FinalScore {scores: Vec::new()}
        
    }
}


pub fn update_final_score(mut game_over_event_reader : EventReader<GameOver>, mut final_scores : ResMut<FinalScore>){
    for event in game_over_event_reader.iter(){
        final_scores.scores.push((event.base_level, event.time_alive));
    }
}

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>){
    
    if keyboard_input.just_pressed(KeyCode::Escape){
        app_exit_event_writer.send(AppExit);
    }

}

pub fn spawn_camera(mut commands: Commands, window_query : Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle{
        transform : Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        ..default()
    });
}

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
        }
        if simulation_state.0 == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
        }
    }
}

pub fn handle_game_over(mut game_over_event_reader : EventReader<GameOver>, mut next_app_state : ResMut<NextState<AppState>>){
    for i in game_over_event_reader.iter(){
        println!("Time alive: {} seconds | base level : {}", i.time_alive, i.base_level);
        next_app_state.set(AppState::GameOver)
    }
}

pub fn transition_to_game_state( keyboard_input: Res<Input<KeyCode>>, mut next_app_state : ResMut<NextState<AppState>>, app_state : Res<State<AppState>>) {
    if keyboard_input.pressed(KeyCode::G){
        if app_state.0 != AppState::Game{
            next_app_state.set(AppState::Game);
        }
    }

}

pub fn transition_to_main_menu_state(keyboard_input: Res<Input<KeyCode>>, app_state : Res<State<AppState>>, mut next_app_state : ResMut<NextState<AppState>>, mut next_simulation_state : ResMut<NextState<SimulationState>>){
    if keyboard_input.pressed(KeyCode::M){
        if app_state.0 != AppState::MainMenu{
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
        };
    }
}











