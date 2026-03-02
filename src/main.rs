use bevy::{app::AppExit, prelude::*}; // 

mod repetitive_code;
mod player;
// mod wave;
mod bullet;
mod enemy;
mod base;
mod main_menu;
mod turret;
mod part;
mod pause_menu;
// mod game_over;
// mod hud;

use turret::TurretPlugin;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use base::BasePlugin;
use main_menu::MainMenuPlugin;
// use wave::WavePlugin;
use player::PlayerPlugin;
use pause_menu::PauseMenuPlugin;
// use game_over::GameOverMenuPlugin;
// use hud::HudPlugin;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)

    // Startup
    .add_systems(Startup,spawn_camera)

    //Resources
    .init_resource::<FinalScore>()
    //States
    .init_state::<AppState>()
    .init_state::<SimulationState>()

    //Events
    .add_event::<GameOver>()

     // OnEnter Systems
     .add_systems(OnEnter(AppState::Game), resume_simulation)
     

    //Plugins
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(TurretPlugin)
    // .add_plugin(WavePlugin)
    .add_plugins(BasePlugin)
    .add_plugins(BulletPlugin)
    .add_plugins(MainMenuPlugin)
    .add_plugins(PauseMenuPlugin)
    // .add_plugin(GameOverMenuPlugin)
    // .add_plugins(HudPlugin)

    //Systems
    .add_systems(Update, (
        exit_game, 
        transition_to_game_state, 
        transition_to_main_menu_state,
        update_final_score,
        toggle_simulation.run_if(in_state(AppState::Game)),
        handle_game_over.run_if(in_state(AppState::GameOver))
    ))

    //On Exit Systems
    .add_systems(OnExit(AppState::Game), pause_simulation)
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
    Running,
    #[default] // settig default here makes sure that when the app starts, the simulation is paused, which is what we want since we start in the main menu
    Paused,
}


#[derive(Event)]
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
    for event in game_over_event_reader.read(){
        final_scores.scores.push((event.base_level, event.time_alive));
    }
}

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>){
    
    if keyboard_input.just_pressed(KeyCode::Escape){
        app_exit_event_writer.write_default();
        println!("escaped");
    }

}

pub fn spawn_camera(mut commands: Commands){ // window_query : Query<&Window, With<PrimaryWindow>>
    // let window = window_query.unwrap();
    commands.spawn(Camera2d);
}

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("game paused");
        }
        if *simulation_state.get() == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("game resumed");
        }
    }
}

pub fn handle_game_over(mut game_over_event_reader : EventMutator<GameOver>, mut next_app_state : ResMut<NextState<AppState>>,){
    for i in game_over_event_reader.read(){
        println!("Time alive: {} seconds | base level : {}", i.time_alive, i.base_level);
        next_app_state.set(AppState::GameOver);
        println!("Transitioning to Game Over State");
    }
}


pub fn transition_to_game_state( keyboard_input: Res<ButtonInput<KeyCode>>, mut next_app_state : ResMut<NextState<AppState>>, app_state : Res<State<AppState>>) {
    if keyboard_input.pressed(KeyCode::KeyG){
        if *app_state.get() != AppState::Game{
            next_app_state.set(AppState::Game);
            println!("Transitioning to Game State");
        }
    }

}

pub fn transition_to_main_menu_state(keyboard_input: Res<ButtonInput<KeyCode>>, app_state : Res<State<AppState>>, mut next_app_state : ResMut<NextState<AppState>>, mut next_simulation_state : ResMut<NextState<SimulationState>>){
    if keyboard_input.pressed(KeyCode::KeyM){
        if *app_state.get() != AppState::MainMenu{
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("Transitioning to Main Menu State");
        };
    }
}











