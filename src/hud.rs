use bevy::prelude::*;
use crate::{base::*, part::PartTier, repetitive_code::*};
use super::{AppState,SimulationState};
use std::f32::consts::PI;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app


            // OnEnter Systems
            .add_systems(OnEnter(AppState::Game), spawn_hud)

            // Systems
            .add_systems(Update,(update_parts, parts_gui).run_if(in_state(AppState::Game).and(in_state(SimulationState::Running))))

            // OnExit Systems
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}



//Components

#[derive(Component)]
pub struct HUD{}

#[derive(Component)]
pub struct PartIcon{}

//Layout


pub fn spawn_hud(mut commands: Commands) {
    build_hud(&mut commands);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn build_hud(commands: &mut Commands) -> Entity {
    let hud_entity = commands.spawn((
        NodeBundle{
            style : HUD_STYLE,
            background_color : BACKGROUND_COLOR.into(),
            ..default()
        },
        HUD{}
    ))
    .with_children(|parent| {
        for _ in 0..8{
            parent.spawn((
                ImageBundle{
                    style: IMAGE_STYLE,
                    background_color : Color::srgb(1.0, 1.0, 1.0).into(),
                    ..default()
                },
                PartIcon{}
            ));
        }
    })
        

        .id();
    hud_entity
}





//Updates
pub fn update_parts(mut part_icon_query : Query<(&mut UiImage, &mut BackgroundColor), With<PartIcon>>, base_query : Query<&Base, With<Base>>, asset_server: Res<AssetServer>){
    for base in base_query.iter(){
        for (part_tier, (mut part_icon, mut part_icon_bgcolor)) in base.parts_required.iter().zip(part_icon_query.iter_mut()){
            let part_image: Handle<Image> = match part_tier{
                PartTier::Blue => {asset_server.load("Sprites/spaceParts_008.png")},
                PartTier::Red => {asset_server.load("Sprites/spaceParts_013.png")},
                PartTier::Green => {asset_server.load("Sprites/spaceParts_025.png")}
            };
            part_icon.texture = part_image.clone();
            part_icon_bgcolor.0.set_a(0.3);
        }
        
        
    }
}

pub fn parts_gui(mut part_icon_query : Query<(&mut BackgroundColor, &mut Transform), With<PartIcon>>, base_query : Query<&Base, With<Base>>){
    for base in base_query.iter(){
       for ((part, part_tier), (mut part_icon_bgcolor, mut part_icon_transform)) in base.parts.iter().zip(base.parts_required.iter()).zip(part_icon_query.iter_mut()){
            if part.part_tier == *part_tier{
                part_icon_bgcolor.0.set_a(1.0);
            }
            part_icon_transform.rotation *= Quat::from_rotation_z(PI/360.0)
       }
    }
}