use std::{ f32::consts::PI, time::Instant};
use rand::prelude::*;
use bevy::{ prelude::*, window::PrimaryWindow, sprite::collide_aabb::collide};

use crate::part::*;
use crate::repetitive_code::*;
use crate::base::*;
use crate::enemy::*;
use crate::bullet::*;
use super::SimulationState;
use super::AppState;

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
pub struct PlayerMovementSet;

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
pub struct ConfinementSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app

        //Ordering 
        .configure_set(PlayerMovementSet.before(ConfinementSet))

        //Resource
        .init_resource::<BlasterCooldownTimer>()

        //When entering Game Appstate
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))

        //player movements
        .add_systems(
            (
                player_movement.in_set(PlayerMovementSet),
                confine_player_movement.in_set(ConfinementSet)
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )


        //while in game Appstate
        .add_systems(
            (
                player_shoot,
                blaster_timer_ticker,
                player_shoot_enemy,
                enemy_hit_player,
                base_part_collecting
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )


        //on exit Game Appstate
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    
    }
}


pub const PLAYER_SIZE :f32 = 32.0;
pub const BLASTER_COOLDOWN : f32 =  0.5;

#[derive(Component)]
pub struct Player{
    pub health : i64,
    pub speed : f32,
    pub size : Vec2,
    pub max_health : i64
}


#[derive(Resource)]
pub struct BlasterCooldownTimer{
    pub timer: Timer,
}

impl Default for BlasterCooldownTimer{
    fn default() -> BlasterCooldownTimer {
        BlasterCooldownTimer{timer: Timer::from_seconds(BLASTER_COOLDOWN, TimerMode::Repeating)}
    }
}

impl BlasterCooldownTimer{
    pub fn set_cooldown(&mut self, new_cooldown : f32){
        self.timer = Timer::from_seconds(new_cooldown, TimerMode::Repeating);
    }
}

pub fn despawn_player(mut commands : Commands, player_query: Query<Entity, With<Player>>, mut blaster_timer: ResMut<BlasterCooldownTimer>){
    if let Ok(player_entity) = player_query.get_single(){
        commands.entity(player_entity).despawn();
        *blaster_timer = BlasterCooldownTimer::default();
    }
}

pub fn pause_simulation(mut next_simulation_state : ResMut<NextState<SimulationState>>){
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_simulation_state : ResMut<NextState<SimulationState>>){
    next_simulation_state.set(SimulationState::Running);

}

pub fn blaster_timer_ticker(mut blaster_timer: ResMut<BlasterCooldownTimer>, time : Res<Time>){
    blaster_timer.timer.tick(time.delta());

}

pub fn spawn_player(mut commands: Commands, asset_server : Res<AssetServer>, window_query : Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();
    commands.spawn((
            SpriteBundle{
            transform : Transform{
                translation: Vec3::new(window.width()/2.0, window.height()/2.5, 0.0),
                scale: Vec3::splat(0.2), // Decrease the size by half along all axes
                ..default()
                },
            texture : asset_server.load("Sprites/spaceShips_008.png"),
            ..default()
        },
        Player{health: 100, speed : 250.0, size : Vec2::new(15.0, 15.0), max_health : 100}
    ));
}

pub fn player_movement(keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<(&mut Transform, &Player), With<Player>>, time: Res<Time>){
    if let Ok((mut transform, player)) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {direction += Vec3::new(0.0, 1.0, 0.0); transform.rotation = Quat::from_rotation_z(0.0);}
        if keyboard_input.pressed(KeyCode::A) {direction += Vec3::new(-1.0, 0.0, 0.0); transform.rotation = Quat::from_rotation_z(1.5708);}
        if keyboard_input.pressed(KeyCode::S) {direction += Vec3::new(0.0, -1.0, 0.0); transform.rotation = Quat::from_rotation_z(3.14159);}
        if keyboard_input.pressed(KeyCode::D) {direction += Vec3::new(1.0, 0.0, 0.0); transform.rotation = Quat::from_rotation_z(-1.5708);}
        if direction.length() > 0.0 {
            direction = direction.normalize();}

        transform.translation += direction * player.speed * time.delta_seconds();
    }
}

pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>, window_query : Query<&Window, With<PrimaryWindow>>){

    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE/2.0;
        let x_lim = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_lim = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;
        if translation.x < x_lim{translation.x = x_lim;}
        else if translation.x > x_max {translation.x = x_max;}
        else if translation.y < y_lim {translation.y = y_lim;}
        else if translation.y > y_max {translation.y = y_max;}

        player_transform.translation = translation;
    }

}

pub fn player_shoot(mut commands: Commands, mouse_input : Res<Input<MouseButton>>, player_query: Query<(&Transform, Entity), With<Player>>, asset_server : Res<AssetServer>,  blaster_timer: ResMut<BlasterCooldownTimer>, window_query : Query<&Window, With<PrimaryWindow>>, audio : Res<Audio>){
    if let Ok((player_transform, player_entity)) = player_query.get_single(){
        let translation = player_transform.translation;
         if let Some(cursor_position) = window_query.single().cursor_position(){
            if mouse_input.pressed(MouseButton::Left) && blaster_timer.timer.just_finished(){
                let angle = Quat::from_rotation_z((cursor_position.y - translation.y).atan2(cursor_position.x - translation.x) - PI/2.0);
                commands.spawn((
                        SpriteBundle{
                            transform: Transform{
                                translation: Vec3::new(translation.x, translation.y, 0.0),
                                scale: Vec3::splat(0.2),
                                ..default()
                            },
                            texture : asset_server.load("Sprites/spaceMissiles_027.png"),
                            ..default()
                        },
                        
                        Bullet{speed: BULLET_SPEED, direction : Vec2::new(cursor_position.x - translation.x, cursor_position.y-translation.y).normalize(), size : Vec2::new(10.0, 10.0), damage : 50, instant : Instant::now()}
                    ));
                commands.entity(player_entity).insert(
                    Transform{
                        translation : translation,
                        rotation : angle,
                        scale : Vec3::splat(0.2),
                        ..default()
                    });
                    let sound_effect = asset_server.load("Audio/laserSmall_000.ogg");
                    audio.play(sound_effect); 
            }
        }
         
    }

}

pub fn player_shoot_enemy(mut commands: Commands, mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>, mut bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>, asset_server : Res<AssetServer>, audio : Res<Audio>){
    
    for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut(){
        for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut(){
            if collide(enemy_transform.translation, enemy.size, bullet_transform.translation, bullet.size).is_some(){
                commands.entity(bullet_entity).despawn();
                enemy.health -= bullet.damage;
                if enemy.health <= 0{
                    let sound_effect = asset_server.load("Audio/explosionCrunch_004.ogg");
                    let sound_effect_two = asset_server.load("Audio/lowFrequency_explosion_001.ogg");
                    match enemy.variant {
                        EnemyType::Pawn => {
                            let reward_chance = random::<f32>();
                            if reward_chance < 0.1{
                                commands.spawn((SpriteBundle{
                                    transform : get_enemy_transform_0_2(enemy_transform.translation),
                                    texture : asset_server.load("Sprites/spaceParts_008.png"),
                                    ..default()
                                },
                                Part{part_tier : PartTier::Blue, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                            ));
                            }
                            commands.entity(enemy_entity).despawn();
                            audio.play(sound_effect);
                        },
                        EnemyType::Stinger =>{
                            let reward_chance = random::<f32>();
                            if reward_chance < 0.1{
                                commands.spawn((SpriteBundle{
                                    transform : get_enemy_transform_0_2(enemy_transform.translation),
                                    texture : asset_server.load("Sprites/spaceParts_013.png"),
                                    ..default()
                                },
                                Part{part_tier : PartTier::Red, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                            ));
                            }
                            commands.entity(enemy_entity).despawn();
                            audio.play(sound_effect);
                        },
                        EnemyType::Rogue => {
                            let reward_chance = random::<f32>();
                            if reward_chance < 0.2{
                                let reward_tier_chance = random::<f32>();
                                if reward_tier_chance < 0.4{
                                    commands.spawn((SpriteBundle{
                                        transform : get_enemy_transform_0_2(enemy_transform.translation),
                                        texture : asset_server.load("Sprites/spaceParts_013.png"),
                                        ..default()
                                    },
                                    Part{part_tier : PartTier::Red, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                ));
                                }
                                else{
                                    commands.spawn((SpriteBundle{
                                        transform : get_enemy_transform_0_2(enemy_transform.translation),
                                        texture : asset_server.load("Sprites/spaceParts_008.png"),
                                        ..default()
                                    },
                                    Part{part_tier : PartTier::Blue, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                ));
                                }
                            }
                            commands.entity(enemy_entity).despawn();
                            audio.play(sound_effect);
                        },
                        EnemyType::Splitter{split_count : count, instant : _, direction: _} => {
                            if count == 0{
                                for _ in 0..2{
                                    commands.spawn((SpriteBundle{
                                        transform : Transform{
                                            translation : enemy_transform.translation + Vec3::new(10.0, 10.0, 0.0),
                                            scale : Vec3::splat(0.25),
                                            ..default()
                                        },
                                        texture : asset_server.load("Sprites/splitter.png"),
                                        ..default()
    
                                    },
                                    Enemy{health : 150, variant: EnemyType::Splitter { split_count: 1, instant: Instant::now(), direction : Vec3::new(rand::thread_rng().gen_range(-1.0..=1.0), rand::thread_rng().gen_range(-1.0..=1.0), 0.0)}, speed : 20.0, size : Vec2::new(10.0, 10.0)}
                                ));
                                }     
                                commands.entity(enemy_entity).despawn();
                                audio.play(sound_effect_two);
                            }
                            else if count == 1{
                                for _ in 0..2{
                                    commands.spawn((SpriteBundle{
                                        transform : Transform{
                                            translation : enemy_transform.translation + Vec3::new(10.0, 10.0, 0.0),
                                            scale : Vec3::splat(0.20),
                                            ..default()
                                        },
                                        texture : asset_server.load("Sprites/splitter.png"),
                                        ..default()
    
                                    },
                                    Enemy{health : 100, variant: EnemyType::Splitter { split_count: 2, instant: Instant::now(), direction : Vec3::new(rand::thread_rng().gen_range(-1.0..=1.0), rand::thread_rng().gen_range(-1.0..=1.0), 0.0)}, speed : 25.0, size : Vec2::new(5.0, 5.0)}
                                ));
                                }
                                let reward_chance = random::<f32>();
                                if reward_chance < 0.3{
                                    let reward_tier_chance = random::<f32>();
                                    if reward_tier_chance < 0.5{
                                        commands.spawn((SpriteBundle{
                                            transform : get_enemy_transform_0_2(enemy_transform.translation),
                                            texture : asset_server.load("Sprites/spaceParts_013.png"),
                                            ..default()
                                        },
                                        Part{part_tier : PartTier::Red, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                    ));
                                    }
                                    
                                    }else{
                                    commands.spawn((SpriteBundle{
                                        transform : get_enemy_transform_0_2(enemy_transform.translation),
                                        texture : asset_server.load("Sprites/spaceParts_008.png"),
                                        ..default()
                                    },
                                    Part{part_tier : PartTier::Blue, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                    ));
                                    }
                                commands.entity(enemy_entity).despawn();
                                audio.play(sound_effect_two);
                            }
                            else {
                                commands.entity(enemy_entity).despawn();
                                audio.play(sound_effect);
                            }
                        },

                        EnemyType::Bishop => {
                            let reward_chance = random::<f32>();
                            if reward_chance < 0.3{
                                let reward_tier_chance = random::<f32>();
                                if reward_tier_chance < 0.25{
                                    commands.spawn((SpriteBundle{
                                        transform : get_enemy_transform_0_2(enemy_transform.translation),
                                        texture : asset_server.load("Sprites/spaceParts_025.png"),
                                        ..default()
                                    },
                                    Part{part_tier : PartTier::Green, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                ));
                                }
                                else if reward_tier_chance < 0.5{
                                    commands.spawn((SpriteBundle{
                                        transform : get_enemy_transform_0_2(enemy_transform.translation),
                                        texture : asset_server.load("Sprites/spaceParts_013.png"),
                                        ..default()
                                    },
                                    Part{part_tier : PartTier::Red, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                ));
                                }
                                else{
                                    commands.spawn((SpriteBundle{
                                        transform : get_enemy_transform_0_2(enemy_transform.translation),
                                        texture : asset_server.load("Sprites/spaceParts_008.png"),
                                        ..default()
                                    },
                                    Part{part_tier : PartTier::Blue, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                                ));      
                                }
                            }
                            commands.entity(enemy_entity).despawn();
                            audio.play(sound_effect);
                        },
                        EnemyType::Propagator => {
                            let reward_tier_chance = random::<f32>();
                            if reward_tier_chance < 0.4{
                                commands.spawn((SpriteBundle{
                                    transform : get_enemy_transform_0_2(enemy_transform.translation),
                                    texture : asset_server.load("Sprites/spaceParts_025.png"),
                                    ..default()
                                },
                                Part{part_tier : PartTier::Green, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                            ));
                            }
                            else if reward_tier_chance < 0.5{
                                commands.spawn((SpriteBundle{
                                    transform : get_enemy_transform_0_2(enemy_transform.translation),
                                    texture : asset_server.load("Sprites/spaceParts_013.png"),
                                    ..default()
                                },
                                Part{part_tier : PartTier::Red, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                            ));
                            }
                            else{
                                commands.spawn((SpriteBundle{
                                    transform : get_enemy_transform_0_2(enemy_transform.translation),
                                    texture : asset_server.load("Sprites/spaceParts_008.png"),
                                    ..default()
                                },
                                Part{part_tier : PartTier::Blue, size : Vec2::new(15.0, 15.0), instant : Instant::now()}
                            ));      
                            }
                            commands.entity(enemy_entity).despawn();
                            audio.play(sound_effect);
                        },

                        _ => {
                            commands.entity(enemy_entity).despawn();
                            audio.play(sound_effect);
                        }
                        
                    }
                   
                    
                }
                
            }
        }
    }

}




pub fn base_part_collecting(mut commands: Commands, mut base_query: Query<&mut Base, (With<Base>, Without<Player>)>, mut part_query : Query<(Entity, &mut Transform, &Part), (With<Part>, Without<Player>)>, player_query: Query<(&Transform, &Player), (With<Player>, Without<Enemy>, Without<Base>)>, asset_server : Res<AssetServer>, audio : Res<Audio>){
    if let Ok((player_transform, player)) = player_query.get_single(){
        for (part_entity, mut part_transform, part) in part_query.iter_mut(){
            if collide(player_transform.translation, player.size, part_transform.translation, part.size).is_some(){
                match part.part_tier{
                    PartTier::Blue => {
                        audio.play(asset_server.load("Audio/impactMining_002.ogg"));
                    },
                    PartTier::Red => {
                        audio.play(asset_server.load("Audio/impactMining_003.ogg"));
                    }
                    PartTier::Green => {
                        audio.play(asset_server.load("Audio/impactMining_001.ogg"));
                    }
                }
                for mut base in base_query.iter_mut(){
                    base.parts.push(*part);
                    println!("{:?}", base.parts_required);
                    println!("{:?}", base.parts);
                    println!("{:?}", base.level);
                }
                commands.entity(part_entity).despawn();
            }
            if part.instant.elapsed().as_secs() > 10{
                commands.entity(part_entity).despawn();
            }
            part_transform.rotation *= Quat::from_rotation_z(-PI/360.0);
        }
    }
}

pub fn enemy_hit_player(mut commands: Commands, enemy_query: Query<(Entity, &Enemy, &Transform), (With<Enemy>, Without<Player>, Without<Base>)>, mut player_query: Query<(&mut Player, &mut Transform), (With<Player>, Without<Enemy>, Without<Base>)>, asset_server : Res<AssetServer>, base_query : Query<&Transform, (With<Base>, Without<Enemy>, Without<Player>)>, audio: Res<Audio>){
    if let Ok((mut player,  mut player_transform)) = player_query.get_single_mut(){
        for (enemy_entity, enemy, enemy_transform) in enemy_query.iter(){
           if collide(player_transform.translation, player.size, enemy_transform.translation, enemy.size).is_some(){
                //let sound_effect_enemy = ;
                player.health -= enemy.health;
                if player.health <= 0{
                    //let sound_effect_player = asset_server.load("Audio/explosionCrunch_003.ogg");
                    audio.play(asset_server.load("Audio/explosionCrunch_003.ogg"));
                    for base_transform in base_query.iter(){
                        player_transform.translation = base_transform.translation;
                        player.health = player.max_health;
                    }
                }
                commands.entity(enemy_entity).despawn();
                audio.play(asset_server.load("Audio/explosionCrunch_002.ogg"));
                
                
           }
        }
    }
}