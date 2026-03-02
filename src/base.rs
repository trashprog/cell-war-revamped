use std::f32::consts::PI;
use std::time::Instant;
use bevy::{ prelude::*, window::PrimaryWindow};

use crate::player::*;
use crate::turret::*;
use crate::part::*;
use crate::enemy::*;
use super::{GameOver, AppState, SimulationState};
use crate::repetitive_code::*;

pub struct BasePlugin;

impl Plugin for BasePlugin{
    fn build(&self, app: &mut App) {
        app


        //On enter game appstate
        .add_systems(OnEnter(AppState::Game), spawn_base)
        
        .add_systems(Update, (base_leveling, base_levels, cheat_leveling, enemy_hit_base)
            .run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))

        //on exit game appstate
        .add_systems(OnExit(AppState::Game), ( despawn_base, despawn_parts, despawn_base_buildings));
        
    }
}

#[derive(Component, Debug, PartialEq)]
pub struct Base{
    pub health : i64,
    pub level : i64,
    pub parts : Vec<Part>,
    pub parts_required: Vec<PartTier>,
    pub max_parts : usize,
    pub leveled_up : bool,
    pub size : Vec2,
    instant : Instant
}

#[derive(Component)]

pub struct BaseBuilding;


impl Base{
    fn push_part_required(&mut self, part_tier : PartTier) -> Result<(), ()>{
        if self.parts_required.len() >= self.max_parts{
            return Err(());
        }
        else{
            self.parts_required.push(part_tier);
            Ok(())
        }
    }
}

#[derive(Bundle)]
pub struct BaseBundle<T: Component>{
    pub base_type: T,
    pub sprite: Sprite,
    pub transform: Transform,
}

pub fn despawn_base(mut commands: Commands, base_query : Query<Entity, With<Base>>){
    for i in base_query.iter(){
        commands.entity(i).despawn()
    }
}
pub fn despawn_parts(mut commands: Commands, part_query : Query<Entity, With<Part>>){
    for i in part_query.iter(){
        commands.entity(i).despawn()
    }
}

pub fn despawn_base_buildings(mut commands: Commands, base_building_query : Query<Entity, With<BaseBuilding>>){
    for i in base_building_query.iter(){
        commands.entity(i).despawn()
    }
}


pub fn spawn_base(mut commands: Commands, asset_server : Res<AssetServer>, window_query : Query<&Window, With<PrimaryWindow>>){
    let window = window_query.single().unwrap();
    commands.spawn(
    BaseBundle{
        sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_005.png"), ..default()},
        transform: Transform{
            translation : Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
            scale :Vec3::splat(0.3),
            ..default()
        },
        base_type: BaseBuilding{}
        });
}




pub fn base_leveling(mut base_query: Query<&mut Base, (With<Base>, Without<Player>)>){
    for mut base in base_query.iter_mut(){
        if base.parts_required.is_empty() || base.parts.is_empty(){
            match base.level{
                1 => {
                    for _ in 0..base.max_parts{
                        match base.push_part_required(PartTier::Blue){
                            Ok(_) => {},
                            Err(_) => {},    
                        }
                    }
                },
                2 => {
                    for _ in 0..base.max_parts{
                        let part_chance = generate_random_number();
                        if part_chance < 0.3{
                            match base.push_part_required(PartTier::Red){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else{
                            match base.push_part_required(PartTier::Blue){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                    }
                },
                3 => {
                    for _ in 0..base.max_parts{
                        let part_chance = generate_random_number();
                        if part_chance < 0.4{
                            match base.push_part_required(PartTier::Red){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else{
                            match base.push_part_required(PartTier::Blue){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                    }
                },
                4 => {
                    for _ in 0..base.max_parts{
                        let part_chance = generate_random_number();
                        if part_chance < 0.5{
                            match base.push_part_required(PartTier::Red){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else{
                            match base.push_part_required(PartTier::Blue){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                    }
                },
                5 => {
                    for _ in 0..base.max_parts{
                        let part_chance = generate_random_number();
                        if part_chance < 0.3{
                            match base.push_part_required(PartTier::Green){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else if part_chance < 0.5{
                            match base.push_part_required(PartTier::Red){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else{
                            match base.push_part_required(PartTier::Blue){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                    }
                },
                6 => {
                    for _ in 0..base.max_parts{
                        let part_chance = generate_random_number();
                        if part_chance < 0.5{
                            match base.push_part_required(PartTier::Green){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else {
                            match base.push_part_required(PartTier::Red){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                    }
                },
                _ => {
                    for _ in 0..base.max_parts{
                        let part_chance = generate_random_number();
                        if part_chance < 1.0/3.0{
                            match base.push_part_required(PartTier::Green){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else if part_chance < 2.0/3.0{
                            match base.push_part_required(PartTier::Red){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                        else{
                            match base.push_part_required(PartTier::Blue){
                                Ok(_) => {},
                                Err(_) => {},    
                            }
                        }
                    }
                }
            }
        }
    }
    for mut base in base_query.iter_mut(){
        for (part, part_tier) in base.parts.iter().zip(base.parts_required.iter()){
            if part.part_tier != *part_tier{
                base.parts.clear();
                base.parts_required.clear();
                break;
            }
            else{
                if base.parts.len() == base.parts_required.len(){
                base.parts.clear();
                base.parts_required.clear();
                if base.max_parts < 8{
                    base.max_parts += 1;
                }
                base.level += 1;
                base.leveled_up = true;
                break;
                }
            }
        }
    }

}

pub fn base_levels(mut commands: Commands, mut base_query: Query<(&Transform, &mut Base), (With<Base>, Without<Player>)>, mut player_query: Query<&mut Player, (With<Player>, Without<Base>)>, mut blaster_timer: ResMut<BlasterCooldownTimer>, asset_server : Res<AssetServer>){
    if let Ok(mut player) = player_query.single_mut(){
        for (base_transform, mut base) in base_query.iter_mut(){
            match base.level {
                2 => {
                    if base.leveled_up{
                        commands.spawn(
                    BaseBundle{
                        sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_001.png"), ..default()},
                        transform: Transform{
                            translation : Vec3::new(base_transform.translation.x, base_transform.translation.y, 0.0) + Vec3::new(34.0, 5.0, 0.0),
                            scale : Vec3::splat(0.2),
                            ..default()
                        },
                        base_type: BaseBuilding{}
                    });
                        let sound_effect = asset_server.load("Audio/computerNoise_000.ogg");
                        commands.spawn(AudioPlayer::new(sound_effect));
                        blaster_timer.set_cooldown(0.4);
                        player.speed = 300.0;
                        player.max_health = 150;
                        player.health = 150;
                        base.health = 1000;
                        base.size = Vec2::new(55.0, 55.0);
                        base.leveled_up = false;
                    }
                   
                },
                3 => {
                    if base.leveled_up{
                        commands.spawn(
                        BaseBundle{
                            sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_002.png"), ..default()},
                            transform: Transform{
                                translation : Vec3::new(base_transform.translation.x, base_transform.translation.y, 0.0) + Vec3::new(-24.0, -6.0, 0.0),
                                scale : Vec3::splat(0.4),
                                ..default()
                            },
                            base_type: BaseBuilding{}
                        });
                        let sound_effect = asset_server.load("Audio/computerNoise_001.ogg");
                        commands.spawn(AudioPlayer::new(sound_effect));
                        blaster_timer.set_cooldown(0.3);
                        player.speed = 350.0;
                        player.max_health = 200;
                        player.health = 200;
                        base.health = 1500;
                        base.size = Vec2::new(60.0, 60.0);
                        base.leveled_up = false;
                    }
                },
                4 => {
                    if base.leveled_up{
                        commands.spawn(
                        BaseBundle{
                            sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_018.png"), ..default()},
                            transform: Transform{
                                translation : Vec3::new(base_transform.translation.x, base_transform.translation.y, 0.0) + Vec3::new(-15.0, -25.0, 0.0),
                                scale : Vec3::splat(0.5),
                                ..default()
                            },
                            base_type: BaseBuilding{}
                        });
                        let sound_effect = asset_server.load("Audio/computerNoise_001.ogg");
                        commands.spawn(AudioPlayer::new(sound_effect));
                        blaster_timer.set_cooldown(0.2);
                        player.speed = 400.0;
                        player.max_health = 250;
                        player.health = 250;
                        base.health = 2000;
                        base.size = Vec2::new(65.0, 65.0);
                        base.leveled_up = false;
                    }
                },
                5 => {
                    if base.leveled_up{
                        commands.spawn(
                    BaseBundle{
                        sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_020.png"), ..default()},
                        transform: Transform{
                            translation : Vec3::new(base_transform.translation.x, base_transform.translation.y, 0.0) + Vec3::new(5.0, -27.0, 0.0),
                            scale : Vec3::splat(0.3),
                            rotation : Quat::from_rotation_z(-PI),
                            ..default()
                        },
                        base_type: Turret{target : None}
                    });
                    let sound_effect = asset_server.load("Audio/computerNoise_002.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                    blaster_timer.set_cooldown(0.1);
                    player.speed = 450.0;
                    player.max_health = 300;
                    player.health = 300;
                    base.health = 2500;
                    base.size = Vec2::new(70.0, 70.0);
                    base.leveled_up = false;
                    }
                },
                6 => {
                    if base.leveled_up{
                        commands.spawn(
                        BaseBundle{
                            sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_020.png"), ..default()},
                            transform: Transform{
                                translation : Vec3::new(base_transform.translation.x, base_transform.translation.y, 0.0) + Vec3::new(15.0, 36.0, 0.0),
                                scale : Vec3::splat(0.3),
                                ..default()
                            },
                            base_type: Turret{target : None}
                        });
                        commands.spawn(
                
                        BaseBundle{
                            sprite: Sprite{image:asset_server.load("Sprites/spaceBuilding_018.png"), ..default()},
                            transform: Transform{
                                translation : Vec3::new(base_transform.translation.x, base_transform.translation.y, 0.0) + Vec3::new(15.0, 30.0, 0.0),
                                scale : Vec3::splat(0.4),
                                ..default()
                            },
                            base_type: BaseBuilding{}
                        });
                        let sound_effect = asset_server.load("Audio/computerNoise_003.ogg");
                        commands.spawn(AudioPlayer::new(sound_effect));
                        player.speed = 500.0;
                        player.max_health = 300;
                        player.health = 300;
                        base.health = 3000;
                        base.leveled_up = false;
                    }

                },
                _ => {
                }
            }
        }
    }
}

pub fn cheat_leveling(mut base_query: Query<&mut Base, With<Base>>, keyboard_input: Res<ButtonInput<KeyCode>>){
    for mut base in base_query.iter_mut(){
        if keyboard_input.just_pressed(KeyCode::Tab){
            base.level += 1;
            base.leveled_up = true;
            println!("{}", base.level);
        }
    }
}


pub fn enemy_hit_base(mut commands: Commands, enemy_query: Query<(Entity, &Transform, &Enemy), With<Enemy>>, mut base_query: Query<(&Transform, &mut Base)>, asset_server: Res<AssetServer>, mut game_over_event_writer: EventWriter<GameOver>) {
    for (base_transform, mut base) in base_query.iter_mut(){
        for (enemy_entity, enemy_transform, enemy) in enemy_query.iter() {
            if collide(base_transform.translation, base.size, enemy_transform.translation, enemy.size).is_some(){
                let sound_effect = asset_server.load("Audio/footstep_snow_002.ogg");
                commands.spawn(AudioPlayer::new(sound_effect));
                commands.entity(enemy_entity).despawn();
                base.health -= enemy.health;
                if base.health <= 0 {
                    let sound_effect = asset_server.load("Audio/explosionCrunch_002.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                    game_over_event_writer.write(GameOver{time_alive : base.instant.elapsed().as_secs(), base_level : base.level});
                }
            }
        }
    }
        
}