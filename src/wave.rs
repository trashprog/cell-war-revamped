use std::time::Instant;
// use rand::prelude::*;
use bevy::{ prelude::*, window::PrimaryWindow,};

use crate::{enemy::*, AppState, SimulationState, repetitive_code::*};

pub struct WavePlugin;

impl Plugin for WavePlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<WaveTimer>()
        .add_systems(Update,(wave_timer_ticker, wave_spawner)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
    
        .add_systems(OnExit(AppState::Game), reset_waves);
    }
}



pub const WAVE_COOLDOWN : f32 = 15.0;


#[derive(Resource)]
pub struct WaveTimer{
    pub timer: Timer,
    pub wave: usize,
    pub enemy_count : i32,
    pub variants : Vec<EnemyType>
}

impl Default for WaveTimer{
    fn default() -> WaveTimer {
        WaveTimer{timer: Timer::from_seconds(WAVE_COOLDOWN, TimerMode::Repeating), wave : 0, enemy_count : 5, variants : vec![EnemyType::Pawn]}
    }
    
}


pub fn calculate_probabilities(elements: usize) -> Vec<f64>{
    let num_elements = elements.min(4);
    let mut probs: Vec<f64> = Vec::new();
    for i in 0..num_elements{
        let prob = (num_elements - i) as f64 / (num_elements * (num_elements + 1) / 2) as f64;
        probs.push(prob);
    }
    probs
}


pub fn wave_spawner(mut commands: Commands, asset_server : Res<AssetServer>, mut wave_timer : ResMut<WaveTimer>, window_query : Query<&Window, With<PrimaryWindow>>){
    let window = window_query.single().unwrap();
    if wave_timer.timer.finished(){
        wave_timer.wave += 1;
        match wave_timer.wave {
            6 => {
                wave_timer.enemy_count = 6;
                wave_timer.variants.push(EnemyType::Stinger)
            },
            15 => {
                wave_timer.enemy_count = 7;
                wave_timer.variants.push(EnemyType::Rogue)
            },
            20 => {
                wave_timer.enemy_count = 8;
                wave_timer.variants.push(EnemyType::Splitter{split_count : 0, instant : Instant::now(), direction : Vec3::ZERO})
            },
            25 => {
                wave_timer.enemy_count = 9;
                wave_timer.variants.remove(0);
                wave_timer.variants.push(EnemyType::Bishop)
            },
            30 => {
                wave_timer.enemy_count = 10;
                wave_timer.variants.remove(0);
                wave_timer.variants.push(EnemyType::Propagator)
            },
            _ => {
                wave_timer.enemy_count = 11
            },
        
        }
        for _ in 0..= wave_timer.enemy_count{
            let rand_side = generate_random_usize() % 4; // Randomly select one of the four sides
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;
            let (rand_x, rand_y) = match rand_side {
                0 => ((generate_random_number() * window.width()) - half_width, -half_height - 20.0), // Bottom edge
                1 => ((generate_random_number() * window.width()) - half_width,  half_height + 20.0), // Top edge
                2 => (-half_width - 20.0, (generate_random_number() * window.height()) - half_height), // Left edge
                _ => ( half_width + 20.0, (generate_random_number() * window.height()) - half_height), // Right edge
            };
            let rand_num : f64 = generate_random_number() as f64;
            let mut cum_prob : f64 = 0.0;
            let probs = calculate_probabilities(wave_timer.variants.len());
            let mut selected_variant = None;

            for (i, &prob) in probs.iter().enumerate(){
                cum_prob += prob;
                if rand_num < cum_prob{
                    selected_variant = Some(&wave_timer.variants[i]);
                    break;
                }
            }
            match selected_variant.unwrap(){
              EnemyType::Pawn => {
                commands.spawn(
                EnemyBundle{
                    enemy: Enemy{health : 50, variant : EnemyType::Pawn, speed : 25.0, size : Vec2::new(10.0, 10.0)},
                    sprite: Sprite{image: asset_server.load("Sprites/pawn.png"), ..default()},
                    transform: Transform{
                        translation : Vec3::new(rand_x, rand_y, 0.0),
                        scale : Vec3::splat(0.15),
                        ..default()
                    }
                }
            );

              },
              EnemyType::Stinger => {
                commands.spawn(
                EnemyBundle{
                    enemy: Enemy{health : 50, variant : EnemyType::Stinger, speed : 40.0, size : Vec2::new(10.0, 10.0)},
                    sprite: Sprite{image: asset_server.load("Sprites/stinger.png"), ..default()},
                    transform: Transform{
                        translation : Vec3::new(rand_x, rand_y, 0.0),
                        scale : Vec3::splat(0.2),
                        ..default()
                    }
                }
            );

              },
              EnemyType::Splitter{split_count : _, instant : _, direction : _} => {
                commands.spawn(
                EnemyBundle{
                    enemy: Enemy{health : 100, variant : EnemyType::Splitter{split_count : 0, instant : Instant::now(), direction : Vec3::ZERO}, speed : 20.0, size : Vec2::new(15.0, 15.0)},
                    sprite: Sprite{image: asset_server.load("Sprites/splitter.png"), ..default()},
                    transform: Transform{
                        translation : Vec3::new(rand_x, rand_y, 0.0),
                        scale : Vec3::splat(0.3),
                        ..default()
                    }
                }
            );

              },
              EnemyType::Rogue => {
                commands.spawn(

                EnemyBundle{
                    enemy: Enemy{health : 200, variant : EnemyType::Rogue, speed : 25.0, size : Vec2::new(15.0, 15.0)},
                    sprite: Sprite{image: asset_server.load("Sprites/rogue.png"), ..default()},
                    transform: Transform{
                        translation : Vec3::new(rand_x, rand_y, 0.0),
                        scale : Vec3::splat(0.3),
                        ..default()
                    }
                }
            );

              },
              EnemyType::Bishop => {
                commands.spawn(
                EnemyBundle{
                    enemy: Enemy{health : 300, variant : EnemyType::Bishop, speed : 15.0, size : Vec2::new(20.0, 20.0)},
                    sprite: Sprite{image: asset_server.load("Sprites/bishop.png"), ..default()},
                    transform: Transform{
                        translation : Vec3::new(rand_x, rand_y, 0.0),
                        scale : Vec3::splat(0.4),
                        ..default()
                    }
                }
            );

              },
              EnemyType::Propagator => {
                commands.spawn(
                EnemyBundle{
                    enemy: Enemy{health : 500, variant : EnemyType::Propagator, speed : 5.0, size : Vec2::new(25.0, 25.0)},
                    sprite: Sprite{image: asset_server.load("Sprites/propogator.png"), ..default()},
                    transform: Transform{
                        translation : Vec3::new(rand_x, rand_y, 0.0),
                        scale : Vec3::splat(0.5),
                        ..default()
                    }
                }
            );

              },
              _ => return

            }
        }
    }
    

}

pub fn wave_timer_ticker(mut wave_timer: ResMut<WaveTimer>, time : Res<Time>){
    wave_timer.timer.tick(time.delta());
    
}

pub fn reset_waves(mut wave_timer : ResMut<WaveTimer>){
    *wave_timer = WaveTimer::default();
}