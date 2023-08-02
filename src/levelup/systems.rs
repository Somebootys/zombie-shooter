use crate::components::AppState;

use bevy::prelude::*;

//This will be used as a marker for the level up screen so I can despawn it when I exit the level up screen
#[derive(Component)]
pub struct OnLevelUpScreenMarker;

// Enum that will be used as a global state for the game

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct SelectedOption;

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
//need this to know which button is pressed, so we can change the value of the stat, plus or minus.

// One of the two settings that can be set through the menu. It will be a resource in the app

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    IncreaseSpeed,
    DecreaseSpeed,
}

#[derive(Resource)]
pub struct XpPoints(pub i32);

#[derive(Resource)]
pub struct Speed(pub f32);
#[derive(Resource)]
pub struct Damage(pub f32);

#[derive(Component, Debug)]
pub enum Stats {
    Speed,
    Damage,
}

//implement slice for stats
impl Stats {
    pub fn nth(&self, n: usize) -> Self {
        match n {
            0 => Stats::Speed,
            1 => Stats::Damage,
            _ => panic!("{} is not a valid index for Stats", n),
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // get the next field of enum to be used as a button text

    // ui camera
    commands.spawn((Camera2dBundle::default(), OnLevelUpScreenMarker));

    let background: Transform = {
        let mut transform: Transform = Default::default();
        transform.scale = Vec3::new(4.0, 3.5, 0.0);
        transform
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphic/background_originalsize.png"),
            transform: background,

            ..Default::default()
        },
        OnLevelUpScreenMarker,
    ));

    commands.insert_resource(Speed(1.0_f32));
    commands.insert_resource(Damage(2.0_f32));

    commands.insert_resource(XpPoints(5));

    let main_node = commands
        .spawn((
            NodeBundle {
                style: MAIN_NODE,
                ..default()
            },
            OnLevelUpScreenMarker,
        ))
        .id();

    //buttons to spawn

    //vector of string name of stats
    let stats_btn_text = vec!["Speed   ", "Damage"];
    for i in 0..stats_btn_text.len() {
        let stat_node = commands
            .spawn(NodeBundle {
                style: STAT_NODE_STYLE,
                border_color: BorderColor(Color::NONE),
                ..default()
            })
            .id();

        let btn_plus = commands
            .spawn((
                ButtonBundle {
                    style: BTN_STYLE_PLUS,
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::IncreaseSpeed,
                Stats::Speed.nth(i),
            ))
            .id();

        let btn_minus = commands
            .spawn((
                ButtonBundle {
                    style: BTN_STYLE_MINUS,
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::DecreaseSpeed,
                Stats::Speed.nth(i),
            ))
            .id();

        let plus_text = commands
            .spawn(TextBundle::from_section(
                " + ",
                TextStyle {
                    font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                    font_size: 80.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ))
            .id();

        let speed_text = commands
            .spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        stats_btn_text[i].to_string() + &": \n",
                        TextStyle {
                            // This font is loaded and will be used instead of the default font.
                            font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        // If no font is specified, it will use the default font.
                        ..default()
                    }),
                ]),
                Stats::Speed.nth(i),
            ))
            .id();

        let minus_text = commands
            .spawn(TextBundle::from_section(
                " - ",
                TextStyle {
                    font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                    font_size: 80.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ))
            .id();

        //pushing children to the main node
        //commands.entity(main_node).push_children(&[stat_node]).push_children(&[btn_plus, btn_minus]).push_children(&[plus_text, speed_text, minus_text]);

        commands.entity(btn_minus).push_children(&[minus_text]);
        commands.entity(btn_plus).push_children(&[plus_text]);
        commands
            .entity(stat_node)
            .push_children(&[btn_minus, speed_text, btn_plus]);
        commands.entity(main_node).push_children(&[stat_node]);
    }

    // make a continue button
    let stat_node = commands
        .spawn((
            ButtonBundle {
                style: STAT_NODE_STYLE,
                border_color: BorderColor(Color::NONE),
                ..default()
            },
            NextLevel,
        ))
        .id();

    let continue_text = commands
        .spawn(TextBundle::from_section(
            " Continue ",
            TextStyle {
                font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                font_size: 80.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .id();

    commands.entity(stat_node).push_children(&[continue_text]);
    commands.entity(main_node).push_children(&[stat_node]);
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction, &Stats),
        (Changed<Interaction>, With<Button>),
    >,
    mut speed: ResMut<Speed>,
    mut damage: ResMut<Damage>,
    mut xp_points: ResMut<XpPoints>,
) {
    for (interaction, menu_button_action, stat_node) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match stat_node {
                Stats::Speed => match menu_button_action {
                    MenuButtonAction::IncreaseSpeed => {
                        if xp_points.0 > 0 {
                            speed.0 += 1_f32;
                            xp_points.0 -= 1;
                        } else {
                            println!("Not enough points")
                        }
                    }
                    MenuButtonAction::DecreaseSpeed => {
                        if speed.0 > 1_f32 {
                            speed.0 -= 1_f32;
                            xp_points.0 += 1;
                        } else {
                            println!("Cannot go below one")
                        }
                    }
                },
                Stats::Damage => match menu_button_action {
                    MenuButtonAction::IncreaseSpeed => {
                        if xp_points.0 > 0 {
                            damage.0 += 1_f32;
                            xp_points.0 -= 1;
                        } else {
                            println!("Not enough points")
                        }
                    }
                    MenuButtonAction::DecreaseSpeed => {
                        if damage.0 > 1_f32 {
                            damage.0 -= 1_f32;
                            xp_points.0 += 1;
                        } else {
                            println!("Cannot go below one")
                        }
                    }
                },
            }
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    //mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, _children) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                //if minus button pressed

                //text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub const MAIN_NODE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    //set children to be in a column
    style.flex_direction = FlexDirection::Column;
    style.flex_basis = Val::Percent(100.0);
    style
};

pub const STAT_NODE_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Px(400.0);
    style.height = Val::Px(100.0);
    style.border = UiRect::all(Val::Px(15.0));
    // horizontally center child text
    style.justify_content = JustifyContent::SpaceEvenly;
    // vertically center child text
    style.align_items = AlignItems::Center;

    style
};

pub const BTN_STYLE_MINUS: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Px(100.0);
    style.height = Val::Px(90.0);
    style.border = UiRect::all(Val::Px(5.0));
    // horizontally center child text
    style.justify_content = JustifyContent::Center;
    // vertically center child text
    style.align_items = AlignItems::Center;
    style
};

pub const BTN_STYLE_PLUS: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Px(100.0);
    style.height = Val::Px(90.0);
    style.border = UiRect::all(Val::Px(5.0));
    // horizontally center child text
    style.justify_content = JustifyContent::Center;
    // vertically center child text
    style.align_items = AlignItems::Center;
    style
};

// function to update the text of the button
pub fn update_stats(
    speed: Res<Speed>,
    damage: Res<Damage>,
    mut query: Query<(&mut Text, &mut Stats)>,
    //mut que
) {
    for (mut text, stat) in &mut query.iter_mut() {
        match *stat {
            Stats::Speed => text.sections[1].value = speed.0.to_string(),
            Stats::Damage => text.sections[1].value = damage.0.to_string(),
        }
    }
}

#[derive(Component)]
pub struct NextLevel;

pub fn continue_to_next_level(
    interaction_query: Query<(&Interaction, &NextLevel), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<AppState>>,
    // mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            menu_state.set(AppState::InGame);
            // game_state.set(GameState::Game);

            println!("Next level");
        }
    }
}

pub fn _debug_current_state() {
    println!("Current menu state:");
    // println!("Current game state: {:?}", game_state);
}
