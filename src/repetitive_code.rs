use bevy::prelude::*;

pub fn get_enemy_transform_0_2(enemy_translation : Vec3) -> Transform{
    Transform{
        translation : Vec3::new(enemy_translation.x, enemy_translation.y, 0.0),
        scale : Vec3::splat(0.2),
        ..default()
    }
}



//ui styles
pub const BACKGROUND_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const  NORMAL_BUTTON_COLOR : Color = Color::srgb(0.15, 0.15, 0.15);
pub const  HOVERED_BUTTON_COLOR : Color = Color::srgb(0.25, 0.25, 0.25);
pub const  PRESSED_BUTTON_COLOR : Color = Color::srgb(0.35, 0.75, 0.35);

pub const BUTTON_STYLE: Style = Style {
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};

/*pub const  IMAGE_STYLE : Style = Style{
    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
    margin : UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
    
};*/

pub fn get_button_text_style(asset_server : &Res<AssetServer>) -> TextStyle{
    TextStyle{
        font : asset_server.load("Fonts/FiraMono-Medium.ttf"),
        font_size :  32.0,
        color : Color::WHITE   
        }
    
}

pub const PAUSE_MENU_STYLE: Style = Style {
    position_type: PositionType::Absolute, // Needed to display separately from HUD.
    display: Display::Flex,                // Hidden by Default
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const PAUSE_MENU_CONTAINER_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(400.0), Val::Px(400.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("Fonts/FiraMono-Medium.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    }
}

pub const GAME_OVER_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex;                // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.size.width = Val::Percent(100.0);
    style.size.height = Val::Percent(100.0);
    style
};

pub const GAME_OVER_MENU_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.size.width = Val::Px(400.0);
    style.size.height = Val::Px(400.0);
    style.gap.width = Val::Px(8.0);
    style.gap.height = Val::Px(8.0);
    style
};

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("Fonts/FiraMono-Medium.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    }
}


pub const HUD_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(30.0), Val::Percent(7.0)),
    margin : UiRect { left : Val::Px(20.0), right : Val::Px(0.0), top : Val::Px(150.0), bottom : Val::Px(0.0)},
    ..Style::DEFAULT
};


pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(25.0), Val::Px(25.0)),
    margin : UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const MAIN_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..Style::DEFAULT
};



