use crate::components::GameScore;

use crate::game::components::{
    Ammo, AmmoText, EquippedGun, Health, HealthText, OnGameScreenMarker, Player, ScoreText,
};
use bevy::prelude::*;

pub const MAIN_NODE: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.align_items = AlignItems::FlexStart;
    style.justify_content = JustifyContent::SpaceBetween;
    //set children to be in a Row

    //style.row_gap = Val::Px(30.0 * 2.);
    style.flex_direction = FlexDirection::Row;
    style.flex_basis = Val::Percent(100.0);
    // style.margin =  UiRect { left: Val::Percent(2.0), right: Val::Percent(0.0), top: Val::Percent(0.0), bottom: Val::Percent(1.5)};
    style
};

pub const HUD_NODE_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Px(220.0);
    style.height = Val::Px(100.0);
    style.border = UiRect::all(Val::Px(15.0));
    // horizontally center child text
    style.justify_content = JustifyContent::SpaceEvenly;
    // vertically center child text
    style.flex_direction = FlexDirection::Row;

    //set at the bottom of the screen
    style.align_self = AlignSelf::FlexEnd;
    style.align_items = AlignItems::Center;
    style.margin = UiRect {
        left: Val::Percent(2.0),
        right: Val::Percent(0.0),
        top: Val::Percent(0.0),
        bottom: Val::Percent(0.0),
    };

    style
};

pub fn hud_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    //commands.spawn(Camera2dBundle::default());

    let text_style = TextStyle {
        color: Color::ANTIQUE_WHITE,
        font_size: 30.,
        font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
        ..default()
    };

    let texture_handle_ammo = asset_server.load("graphic/ammo_icon.png");
    let texture_handle_health = asset_server.load("graphic/health_pickup.png");
    let texture_handle_crosshair = asset_server.load("graphic/crosshair.png");

    let main_node = commands
        .spawn((
            NodeBundle {
                style: MAIN_NODE,
                ..default()
            },
            OnGameScreenMarker,
        ))
        .id();

    let hud_node_ammo = commands
        .spawn(NodeBundle {
            style: HUD_NODE_STYLE,
            ..default()
        })
        .id();

    let hud_node_health = commands
        .spawn(NodeBundle {
            style: HUD_NODE_STYLE,
            ..default()
        })
        .id();

    let hud_node_score = commands
        .spawn(NodeBundle {
            style: HUD_NODE_STYLE,
            ..default()
        })
        .id();

    //spawn the ammo text
    let ammo_node = commands
        .spawn((ImageBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                ..default()
            },
            image: texture_handle_ammo.clone().into(),

            ..default()
        },))
        .id();

    let ammo_text = commands
        .spawn(TextBundle::from_sections([
            // This is the 0th section, 0
            TextSection::new("Ammo:  ".to_string(), text_style.clone()),
            // This is the first section, 1
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
            // This is the second section, 2
            TextSection::new("  /  ".to_string(), text_style.clone()),
            // This is the third section, 3
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]))
        .insert(AmmoText)
        .id();

    let health_node = commands
        .spawn((ImageBundle {
            style: Style {
                width: Val::Px(65.),
                height: Val::Px(65.),
                margin: UiRect {
                    left: Val::Percent(5.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            image: texture_handle_health.clone().into(),

            ..default()
        },))
        .id();

    let health_text = commands
        .spawn(TextBundle::from_sections([
            // This is the 0th section, 0
            TextSection::new("Health: ".to_string(), text_style.clone()),
            // This is the first section, 1
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]))
        .insert(HealthText)
        .id();

    //I should not use, I should just use the main node.

    let score_node = commands
        .spawn((ImageBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                ..default()
            },
            image: texture_handle_crosshair.clone().into(),

            ..default()
        },))
        .id();

    let score_text = commands
        .spawn(TextBundle::from_sections([
            // This is the 0th section, 0
            TextSection::new("Score:  ".to_string(), text_style.clone()),
            // This is the first section, 1
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]))
        .insert(ScoreText)
        .id();

    //commands.entity(ammo_node).push_children(&[]);
    commands
        .entity(hud_node_ammo)
        .push_children(&[ammo_node, ammo_text]);

    commands
        .entity(hud_node_health)
        .push_children(&[health_node, health_text]);

    commands
        .entity(hud_node_score)
        .push_children(&[score_node, score_text]);

    commands
        .entity(main_node)
        .push_children(&[hud_node_ammo, hud_node_health, hud_node_score]);

    /*

    commands
        .spawn(NodeBundle {
            style: Style {
                //this is where I can change position relative to text
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Start,

                align_self: AlignSelf::End,
                align_items: AlignItems::Center,
                row_gap: Val::Px(text_style.font_size * 2.),
                margin:  UiRect { left: Val::Percent(2.0), right: Val::Percent(0.0), top: Val::Percent(0.0), bottom: Val::Percent(1.5) },


                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((ImageBundle {
                style: Style {
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    ..default()
                },
                image: texture_handle_ammo.into(),


                ..default()
            },));
            parent.spawn(TextBundle::from_sections([

                // This is the 0th section, 0
                TextSection::new("Ammo:  ".to_string(), text_style.clone()),

                 // This is the first section, 1


                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                    font_size: 30.0,
                    color: Color::GOLD,
                }),

                // This is the second section, 2

                TextSection::new("  /  ".to_string(), text_style.clone()),

                // This is the third section, 3

                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                    font_size: 30.0,
                    color: Color::GOLD,
                }),
            ])).insert(AmmoText);
        }).with_children(|parent| {
            parent.spawn((ImageBundle {
                style: Style {
                    width: Val::Px(65.),
                    height: Val::Px(65.),
                    margin: UiRect { left: Val::Percent(5.0), right: Val::Px(0.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
                    ..default()
                },
                image: texture_handle_health.into(),


                ..default()
            },));
            parent.spawn(TextBundle::from_sections([

                // This is the 0th section, 0
                TextSection::new("Health: ".to_string(), text_style.clone()),

                 // This is the first section, 1


                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                    font_size: 30.0,
                    color: Color::GOLD,
                }),



            ])).insert(HealthText);
        });


        //spawn the score text
         // root node
    commands
    .spawn(NodeBundle {
        style: Style {
            //this is where I can change position relative to text
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::End,

            align_self: AlignSelf::End,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0 * 2.),
            margin:  UiRect { left: Val::Percent(0.0), right: Val::Percent(2.0), top: Val::Percent(0.0), bottom: Val::Percent(1.5) },


            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((ImageBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                ..default()
            },
            image: texture_handle_crosshair.clone().into(),


            ..default()
        },));
        parent.spawn(TextBundle::from_sections([

            // This is the 0th section, 0
            TextSection::new("Score:  ".to_string(), text_style.clone()),

             // This is the first section, 1


            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),



        ])).insert(ScoreText);
    });




        //spawn the health text

      // root node

      */
}

pub fn hud_score_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<GameScore>,
) {
    for mut text in &mut query {
        let score: i32 = score.0;

        // Update the value of the second section
        text.sections[1].value = format!("{score:.2}");
    }
}

pub fn hud_ammo_update_system(
    mut query: Query<&mut Text, With<AmmoText>>,
    query_ammo: Query<&Ammo, With<Player>>,
    eq_gun: Res<EquippedGun>,
) {
    for mut text in &mut query {
        if let Some(ammo) = query_ammo.iter().next() {
            let bullet_in_mag = eq_gun.bullets_in_magasine;

            let bullets_in_inventory = ammo.vec[0];

            // Update the value of the second section
            text.sections[1].value = format!("{bullet_in_mag:.2}");
            text.sections[3].value = format!("{bullets_in_inventory:.2}");
        }
    }
}

pub fn hud_health_update_system(
    mut query: Query<&mut Text, With<HealthText>>,
    health_query: Query<&Health, With<Player>>,
) {
    for mut text in &mut query {
        if let Some(health) = health_query.iter().next() {
            let hp = health.hp;

            // Update the value of the second section
            text.sections[1].value = format!("{hp:.2}");
        }
    }
}
