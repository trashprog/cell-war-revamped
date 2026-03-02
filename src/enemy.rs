use std::{ f32::consts::PI, time::{Duration, Instant}};
use bevy::{prelude::*};
use rand::prelude::*;

use crate::base::*;
use crate::player::*;
use crate::bullet::*;
use crate::repetitive_code::*;
use super::{AppState, SimulationState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app

        //Resources
        .init_resource::<EnemyAbilityTimer>()
        .init_resource::<EnemySpawnCooldownTimer>()

        
        //While in game appstate
        .add_systems(Update,(enemy_spawn_timer_ticker,enemy_movement,enemy_ability_timer,deacon_behaviour)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )

        //On exit Game Appstate
        .add_systems(OnExit(AppState::Game), despawn_enemies)
        .add_systems(OnExit(AppState::Game), despawn_deacons);
        
    }
}


pub const ENEMY_SPAWN_COOLDOWN : f32 = 3.0;
pub const ENEMY_ABILITY_CYCLE : f32 = 10.0;
pub const ENEMY_POSITIONS: [(f32, f32);4] = [(5.0, 0.0), (-5.0, 0.0), (0.0, 5.0), (0.0, -5.0)];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NeonateGestation{
    pub direction : Vec3,
    pub spawn_time : Instant
}

impl NeonateGestation{
    fn time_since_spawn(&self) -> Duration{
        self.spawn_time.elapsed()
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Pawn,
    Stinger,
    Splitter{split_count : usize, instant: Instant, direction: Vec3},
    Rogue,
    Bishop,
    Propagator,
    Neonate(NeonateGestation),
}

#[derive(Component)]
pub struct Deacon{
    pub speed : f32,
    pub size : Vec2,
    pub direction : Vec3,
    pub instant : Instant

}

#[derive(Component)]
pub struct Enemy{
    pub health : i64,
    pub variant : EnemyType,
    pub speed : f32,
    pub size : Vec2
}

#[derive(Bundle)]
pub struct EnemyBundle{
    pub sprite: Sprite,
    pub transform: Transform,
    pub enemy: Enemy
}

#[derive(Bundle)]
pub struct DeaconBundle{
    pub(crate) sprite: Sprite,
    pub(crate) transform: Transform,
    pub(crate) deacon: Deacon
}

#[derive(Resource)]
pub struct EnemySpawnCooldownTimer{
    pub timer: Timer,
}

impl Default for EnemySpawnCooldownTimer{
    fn default() -> EnemySpawnCooldownTimer {
        EnemySpawnCooldownTimer{timer: Timer::from_seconds(ENEMY_SPAWN_COOLDOWN, TimerMode::Repeating)}
    }
    
}

#[derive(Resource)]
pub struct EnemyAbilityTimer{
    pub timer: Timer,
}

impl Default for EnemyAbilityTimer{
    fn default() -> EnemyAbilityTimer {
        EnemyAbilityTimer{timer: Timer::from_seconds(ENEMY_ABILITY_CYCLE, TimerMode::Repeating)}
    }
    
}

pub fn enemy_spawn_timer_ticker(mut enemy_spawn_timer: ResMut<EnemySpawnCooldownTimer>, time : Res<Time>){
    enemy_spawn_timer.timer.tick(time.delta());

}

pub fn despawn_enemies(mut commands : Commands, enemy_query : Query<Entity, With<Enemy>>){
    for i in enemy_query.iter(){
        commands.entity(i).despawn();
    }

}

pub fn enemy_ability_timer(mut enemy_ability_timer : ResMut<EnemyAbilityTimer>, time : Res<Time>){
    enemy_ability_timer.timer.tick(time.delta());
}

pub fn enemy_movement( mut enemy_query: Query<(&mut Transform, &mut Enemy), (With<Enemy>, Without<Base>, Without<Player>)>, time : Res<Time>, base_query: Query<&Transform, (With<Base>, Without<Player>, Without<Enemy>)>, player_query: Query<&Transform, (With<Player>, Without<Enemy>, Without<Base>)>, enemy_ability_timer : Res<EnemyAbilityTimer>, mut commands: Commands, asset_server : Res<AssetServer>){
    
    for (mut t, enemy) in enemy_query.iter_mut(){
        let base_translation = base_query.single().unwrap().translation;
        match enemy.variant{
            EnemyType::Pawn => {
                let direction = (base_translation - t.translation).normalize() + Vec3::new(rand::rng().random_range(0.4..=0.4),rand::rng().random_range(0.4..=0.4),0.0, );
                t.translation += direction* enemy.speed *time.delta_secs();
                t.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x) - PI/2.0);
            },
            EnemyType::Stinger => {
                let direction = (base_translation - t.translation).normalize() + Vec3::new(rand::rng().random_range(-0.5..=0.5),rand::rng().random_range(-0.5..=0.5),0.0, );
                t.translation += direction* enemy.speed *time.delta_secs();
                t.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x) - PI/2.0);
            },
            EnemyType::Splitter{split_count : ct, instant : inst, direction : mut dir} => {
                match ct {
                    0 =>{
                        dir = Vec3::new(base_translation.x - t.translation.x, base_translation.y - t.translation.y, 0.0).normalize();
                    },
                    _ => {
                        if inst.elapsed().as_secs() > 1{
                            dir = Vec3::new(base_translation.x - t.translation.x, base_translation.y - t.translation.y, 0.0).normalize();
                        }
                    } 
                }
                t.translation += dir * enemy.speed *time.delta_secs();
                t.rotation *= Quat::from_rotation_z(PI/135.0);
            },
            EnemyType::Rogue => {
                if let Ok(player_transform) = player_query.single(){
                    let direction = (player_transform.translation - t.translation).normalize();
                    t.translation += direction* enemy.speed *time.delta_secs();
                    t.rotation *= Quat::from_rotation_z(-PI/90.0);
    
                }
                else{
                    let direction = (base_translation - t.translation).normalize();
                    t.translation += direction* enemy.speed *time.delta_secs();
                    t.rotation *= Quat::from_rotation_z(PI/90.0);
                }     
            },
            EnemyType::Bishop => {
                if enemy_ability_timer.timer.finished() {
                    for _ in 0..2{
                        commands.spawn(
                        DeaconBundle{
                            sprite: Sprite{image: asset_server.load("Sprites/deacon.png"), ..default()},
                            transform: Transform{
                                translation : Vec3::new(t.translation.x, t.translation.y, 0.0),
                                scale : Vec3::splat(0.2),
                                ..default()
                            },
                            deacon: Deacon{speed: 20.0, size : Vec2::new(10.0, 10.0), direction : Vec3::new(rand::rng().random_range(-1.0..1.0), rand::rng().random_range(-1.0..1.0), 0.0), instant : Instant::now()}
                        });
                    }
                }
                let direction = (base_translation - t.translation).normalize();
                t.translation += direction* enemy.speed *time.delta_secs();
                t.rotation *= Quat::from_rotation_z(-PI/90.0);
            },
            EnemyType::Propagator => {
                if enemy_ability_timer.timer.finished(){
                    for positon in ENEMY_POSITIONS.iter(){
                        commands.spawn(
                        EnemyBundle{
                                sprite: Sprite{image: asset_server.load("Sprites/neonate.png"), ..default()},
                                 transform: Transform{
                                     translation : Vec3::new(t.translation.x, t.translation.y, 0.0),
                                     scale : Vec3::splat(0.2),
                                     ..default()
                                 },
                                 enemy: Enemy{health: 50, variant : EnemyType::Neonate(NeonateGestation{direction : Vec3::new(positon.0, positon.1, 0.0).normalize(), spawn_time : Instant::now()}), speed : 20.0, size : Vec2::new(10.0, 10.0)}
                                });
                    } 
                }else{
                let direction = (base_translation - t.translation).normalize();
                t.translation += direction* enemy.speed *time.delta_secs();
                t.rotation *= Quat::from_rotation_z(-PI/180.0);
                }
            },
            EnemyType::Neonate(mut inner_struct) => {
                if inner_struct.time_since_spawn().as_secs() > 1{
                    let direction = (base_translation - t.translation).normalize() + Vec3::new(rand::rng().random_range(-0.5..=0.5),rand::rng().random_range(-0.5..=0.5),0.0);
                    inner_struct.direction = direction;
                }
                let direction = Vec3::new(inner_struct.direction.x, inner_struct.direction.y, 0.0);
                t.translation += direction* enemy.speed *time.delta_secs();
                t.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x) - PI/2.0);
                
            }
        }
    }

}

pub fn despawn_deacons(mut commands : Commands, deacon_query : Query<Entity, With<Deacon>>){
    for i in deacon_query.iter(){
        commands.entity(i).despawn()
    }

}

pub fn deacon_behaviour(mut commands: Commands, mut deacon_query : Query<(Entity, &mut Transform, &mut Deacon), (With<Deacon>, Without<Enemy>, Without<Bullet>)>, mut enemy_query: Query<(&Transform, &mut Enemy), (With<Enemy>, Without<Deacon>, Without<Bullet>)>, time : Res<Time>, bullet_query: Query<(Entity, &Transform, &Bullet), (With<Bullet>, Without<Enemy>, Without<Deacon>)>, asset_server : Res<AssetServer>){
    for (deacon_entity, mut deacon_transform, mut deacon) in deacon_query.iter_mut(){
        if deacon.instant.elapsed().as_secs() > 1{
            deacon.speed = 0.0;
            for (enemy_transform, mut enemy) in enemy_query.iter_mut(){
                if enemy.variant != EnemyType::Bishop{
                    let distance = enemy_transform.translation.distance(deacon_transform.translation);
                    if distance <= 35.0{
                        deacon.speed = 10.0;
                        deacon.direction = Vec3::new(enemy_transform.translation.x - deacon_transform.translation.x, enemy_transform.translation.y - deacon_transform.translation.y, 0.0);
                    if collide(enemy_transform.translation, enemy.size, deacon_transform.translation, deacon.size).is_some(){
                        let sound_effect = asset_server.load("Audio/doorClose_000.ogg");
                        commands.spawn(AudioPlayer::new(sound_effect));
                        commands.entity(deacon_entity).despawn();
                        enemy.health += 10;
                        enemy.speed += 1.0;
                    }}
                }
                
            }
            
        }
        deacon_transform.translation += deacon.direction * deacon.speed * time.delta_secs();
        deacon_transform.rotation *= Quat::from_rotation_z(-PI/180.0);
        for (bullet_entity, bullet_transform, bullet) in bullet_query.iter(){
            if collide(bullet_transform.translation, bullet.size, deacon_transform.translation, deacon.size).is_some(){
                let sound_effect = asset_server.load("Audio/doorOpen_001.ogg");
                commands.spawn(AudioPlayer::new(sound_effect));
                commands.entity(bullet_entity).despawn();
                commands.entity(deacon_entity).despawn();
            }
        }
    }
}