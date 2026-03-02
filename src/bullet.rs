use std::{ f32::consts::PI, time::Instant};
use bevy::{ prelude::*, window::PrimaryWindow};


use crate::base::*;
use super::AppState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {
        app

        .add_systems(Update, update_bullets.run_if(in_state(AppState::Game))) //only run if in the game state
        .add_systems(OnExit(AppState::Game), despawn_bullets);
    
    }
}






pub const BULLET_LIFETIME : f32 = 1.5;
pub const BULLET_SPEED: f32= 250.0;
pub const BULLET_SIZE : f32 = 16.0;

#[derive(Component)]
pub struct Bullet{
    pub speed: f32,
    pub direction : Vec2,
    pub size : Vec2,
    pub damage : i64,
    pub instant : Instant
    
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub(crate) bullet: Bullet,
    pub(crate) sprite: Sprite,
    pub(crate) transform: Transform,
}

pub fn update_bullets(mut commands: Commands, mut bullet_query: Query<(&mut Transform, &mut Bullet,Entity), (With<Bullet>, Without<Base>)>, time: Res<Time>, window_query : Query<&Window, With<PrimaryWindow>>, base_query: Query<&Base, (With<Base>, Without<Bullet>)>){
    let window = window_query.single().unwrap();
    for (mut transform, mut bullet, entity) in bullet_query.iter_mut(){
        
        let direction = Vec3::new(bullet.direction.x, bullet.direction.y, 0.0);
        transform.translation += direction*bullet.speed*time.delta_secs();
        transform.rotation = Quat::from_rotation_z(bullet.direction.y.atan2(bullet.direction.x) - PI/2.0);
        if bullet.instant.elapsed().as_secs() > BULLET_LIFETIME as u64{
            commands.entity(entity).despawn();
            
        }
        let half_bullet_size = BULLET_SIZE/2.0;
        let x_lim = half_bullet_size;
        let x_max = window.width() - half_bullet_size;
        let y_lim = half_bullet_size;
        let y_max = window.height() - half_bullet_size;

        let translation = transform.translation;
        if translation.x < x_lim{commands.entity(entity).despawn();}
        else if translation.x > x_max {commands.entity(entity).despawn();}
        else if translation.y < y_lim {commands.entity(entity).despawn();}
        else if translation.y > y_max {commands.entity(entity).despawn();}
        for base in base_query.iter(){
            match  base.level {
                1 => {
                    bullet.speed = 300.0;
                    bullet.damage = 50;
                },
                2 =>{
                    bullet.speed = 350.0;
                    bullet.damage = 60;
                },
                3 =>{
                    bullet.speed = 400.0;
                    bullet.damage = 70;
                },
                4 =>{
                    bullet.speed = 450.0;
                    bullet.damage = 80;
                },
                5 =>{
                    bullet.speed = 500.0;
                    bullet.damage = 90;
                },
                6 => {
                    bullet.speed = 550.0;
                    bullet.damage = 100;
                },
                _ =>{
                    bullet.speed = 600.0;
                    bullet.damage = 100;
                }

                
            }
        }
    }
          
}

pub fn despawn_bullets(mut commands: Commands, bullet_query : Query<Entity, With<Bullet>>){
    for i in bullet_query.iter(){
        commands.entity(i).despawn()
    }
}