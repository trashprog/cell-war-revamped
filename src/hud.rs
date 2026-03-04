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
            .add_systems(Update,(update_parts, parts_gui)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)))

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
        get_hud_style(),
        BackgroundColor(BACKGROUND_COLOR),
        HUD{}
    ))
    .with_children(|parent| {
        for _ in 0..8{
            parent.spawn((
                get_image_style(),
                ImageNode{
                    image: Handle::default(),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0), 
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
pub fn update_parts(
    mut part_icon_query: Query<&mut ImageNode, With<PartIcon>>,  // Remove BackgroundColor
    base_query: Query<&Base, With<Base>>,
    asset_server: Res<AssetServer>  
) {

    // load the assets once
    let blue = asset_server.load("Sprites/spaceParts_008.png");
    let red = asset_server.load("Sprites/spaceParts_013.png");
    let green = asset_server.load("Sprites/spaceParts_025.png");

    for base in base_query.iter() {
        for (part_tier, mut part_icon) in base.parts_required.iter().zip(part_icon_query.iter_mut()) {
            let part_image: Handle<Image> = match part_tier {
                PartTier::Blue => blue.clone(),
                PartTier::Red => red.clone(),
                PartTier::Green => green.clone(),
            };
            part_icon.image = part_image;  // Access the image field directly
            part_icon.color = Color::srgba(1.0, 1.0, 1.0, 0.3);
        }
    }
}

pub fn parts_gui(
    mut part_icon_query: Query<(&mut ImageNode, &mut Transform), With<PartIcon>>,
    base_query: Query<&Base, With<Base>>
) {
    for base in base_query.iter() {
        for ((part, part_tier), (mut part_icon, mut part_icon_transform)) in 
            base.parts.iter().zip(base.parts_required.iter()).zip(part_icon_query.iter_mut()) 
        {
            if part.part_tier == *part_tier {
                part_icon.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            }
            part_icon_transform.rotation *= Quat::from_rotation_z(PI / 360.0);
        }
    }
}