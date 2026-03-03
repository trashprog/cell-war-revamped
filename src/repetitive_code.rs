use bevy::{math::bounding::{BoundingCircle, IntersectsVolume}, prelude::*};
use rand::prelude::*;

// i did not make a create bundle function becuase the lines wont change since the params passed in is long*

//ui styles
pub const BACKGROUND_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const  NORMAL_BUTTON_COLOR : Color = Color::srgb(0.15, 0.15, 0.15);
pub const  HOVERED_BUTTON_COLOR : Color = Color::srgb(0.25, 0.25, 0.25);
pub const  PRESSED_BUTTON_COLOR : Color = Color::srgb(0.35, 0.75, 0.35);
const FONT_PATH: &str = "Fonts/FiraMono-Medium.ttf";

pub fn get_enemy_transform_0_2(enemy_translation : Vec3) -> Transform{
    Transform{
        translation : Vec3::new(enemy_translation.x, enemy_translation.y, 0.0),
        scale : Vec3::splat(0.2),
        ..default()
    }
}

pub fn generate_random_number() -> f32{
    let mut rng = rand::rng();
    let random_number: f32 = rng.random();
    random_number
}

pub fn generate_random_usize() -> usize{
    let mut rng = rand::rng();
    let random_number: usize = rng.random_range(0..=4);
    random_number
}


pub fn collide(pos_a: Vec3, size_a: Vec2, pos_b: Vec3, size_b: Vec2,) -> Option<()> {
    let a_radius = size_a.length()*0.5;
    let b_radius = size_b.length()*0.5;
    let bounding_circle_a = BoundingCircle::new(pos_a.truncate(), a_radius);
    let bounding_circle_b = BoundingCircle::new(pos_b.truncate(), b_radius);

    if bounding_circle_a.intersects(&bounding_circle_b) {
        Some(())
    } else {
        None
    }
}

pub fn get_base_button_style() -> Node {
    let mut node = Node::default();
    node.width = Val::Px(200.0);
    node.height = Val::Px(80.0);
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node

}

pub fn get_button_text_components(asset_server : &Res<AssetServer>, text: &str, font_size : f32) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: font_size,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

pub fn build_button() -> impl Bundle{
    (
        Button,
        get_base_button_style(),
        BackgroundColor(NORMAL_BUTTON_COLOR)
    )
}

pub fn build_container() -> impl Bundle{
    (
        
        get_pause_menu_container_style(),
        BackgroundColor(BACKGROUND_COLOR)
        
    )
}


pub fn get_pause_menu_style() -> Node {
    let mut node = Node::default();
    node.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    node.display = Display::Flex;                // Hidden by Default
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node
}

pub fn get_pause_menu_container_style() -> Node {
    let mut node = Node::default();
    node.display = Display::Flex;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(400.0);
    node.height = Val::Px(400.0);
    node.column_gap = Val::Px(8.0);
    node.row_gap = Val::Px(8.0);
    node
}

pub fn get_title_text_components(asset_server : &Res<AssetServer>, text: &str) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 60.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}


pub fn get_game_over_menu_style() -> Node {
    let mut node = Node::default();
    node.position_type = PositionType::Absolute;
    node.display = Display::Flex;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node
}

pub fn get_game_over_menu_container_style() -> Node {
    let mut node = Node::default();
    node.display = Display::Flex;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(400.0);
    node.height = Val::Px(400.0);
    node.column_gap = Val::Px(8.0);
    node.row_gap = Val::Px(8.0);
    node
}

pub fn get_final_score_text_components(asset_server: &Res<AssetServer>, text: &str) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

pub fn get_hud_style() -> Node {
    let mut node = Node::default();
    node.display = Display::Flex;
    node.flex_direction = FlexDirection::Row;
    node.justify_content = JustifyContent::SpaceBetween;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(30.0);
    node.height = Val::Percent(7.0);
    node.margin = UiRect {
        left: Val::Px(20.0),
        right: Val::Px(0.0),
        top: Val::Px(150.0),
        bottom: Val::Px(0.0),
    };
    node
}

pub fn get_image_style() -> Node {
    let mut node = Node::default();
    node.width = Val::Px(25.0);
    node.height = Val::Px(25.0);
    node.margin = UiRect {
        left: Val::Px(4.0),
        right: Val::Px(4.0),
        top: Val::Px(8.0),
        bottom: Val::Px(8.0),
    };
    node
}

pub fn get_main_menu_style() -> Node {
    let mut node = Node::default();
    node.display = Display::Flex;
    node.position_type = PositionType::Absolute;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node.column_gap = Val::Px(8.0);
    node.row_gap = Val::Px(8.0);
    node
}

pub fn get_title_style() -> Node {
    let mut node = Node::default();
    node.flex_direction = FlexDirection::Row;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(300.0);
    node.height = Val::Px(120.0);
    node
}


