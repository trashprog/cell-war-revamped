use std::{ f32::consts::PI, time::Instant};
use bevy::prelude::*;
use crate::bullet::*;
use crate::enemy::*;
use super::{SimulationState, AppState};

pub struct TurretPlugin;

impl Plugin for TurretPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<TurretCooldownTimer>()

        .add_systems(
            (
               turret_timer_ticker,
                turret_movement
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        
        .add_system(despawn_turret.in_schedule(OnExit(AppState::Game)));
    }
}



pub const TURRET_RADAR :f32 = 200.0;
pub const TURRET_COOLDOWN: f32 = 0.5;


#[derive(Component)]
pub struct Turret{
    pub target : Option<Entity>
}

#[derive(Component)]

pub struct TurretMissile{
    pub target : Option<Entity>
}

#[derive(Resource)]
pub struct TurretCooldownTimer{
    pub timer: Timer,
}

impl Default for TurretCooldownTimer{
    fn default() -> TurretCooldownTimer {
        TurretCooldownTimer{timer: Timer::from_seconds(TURRET_COOLDOWN, TimerMode::Repeating)}
    }
    
}


pub fn turret_timer_ticker(mut turret_timer: ResMut<TurretCooldownTimer>, time : Res<Time>){
    turret_timer.timer.tick(time.delta());

}

pub fn turret_movement(mut commands: Commands, enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Turret>)>, mut turret_query : Query<(&mut Transform, &mut Turret), (With<Turret>, Without<Enemy>)>, asset_server : Res<AssetServer>, turret_timer: Res<TurretCooldownTimer>, audio : Res<Audio>){
    for (mut turret_transform, mut turret) in turret_query.iter_mut(){
        let mut fired = false;
        for (enemy_entity, enemy_transform) in enemy_query.iter(){
            if turret_transform.translation.distance(enemy_transform.translation) < TURRET_RADAR{
                if turret.target.is_none(){
                    turret.target = Some(enemy_entity)
                }
                if turret.target.is_some() {
                    if turret_timer.timer.just_finished() && !fired{
                        commands.spawn((SpriteBundle{
                            transform : Transform{
                                translation : Vec3::new(turret_transform.translation.x, turret_transform.translation.y, 0.0),
                                scale : Vec3::splat(0.2),
                                ..default()
                            },
                            texture : asset_server.load("Sprites/spaceMissiles_027.png"),
                            ..default()
                        },
                        Bullet{speed : BULLET_SPEED, size : Vec2::new(10.0, 10.0), direction : Vec2::new(enemy_transform.translation.x - turret_transform.translation.x, enemy_transform.translation.y - turret_transform.translation.y).normalize(), instant : Instant::now(), damage : 50}
                        
                        ));
                        let sound_effect = asset_server.load("Audio/impactGlass_heavy_001.ogg");
                        audio.play(sound_effect);
                        turret_transform.rotation = Quat::from_rotation_z((enemy_transform.translation.y - turret_transform.translation.y).atan2(enemy_transform.translation.x - turret_transform.translation.x) - PI/2.0);
                        fired = true;
                    }
                
                }
            }
        }
    
   }
}

pub fn despawn_turret(mut commands: Commands, turret_query : Query<Entity, With<Turret>>){
    for i in turret_query.iter(){
        commands.entity(i).despawn()
    }
}