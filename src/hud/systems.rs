use bevy::prelude::*;
use crate::components::{ ScoreText, AmmoText, HealthText, Score, EquippedGun, Health, Player, Ammo};

pub fn hud_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    
) {
    // Camera
    //commands.spawn(Camera2dBundle::default());



    commands.insert_resource(Score(0));


    let text_style = TextStyle {
        color: Color::ANTIQUE_WHITE,
        font_size: 30.,
        font: asset_server.load("fonts/MOUNLEY DISCKET.otf"),
        ..default()
    };

    let texture_handle_ammo = asset_server.load("graphic/ammo_icon.png");
    let texture_handle_health = asset_server.load("graphic/health_pickup.png");
    let texture_handle_crosshair = asset_server.load("graphic/crosshair.png");
    // root node

    
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
            image: texture_handle_crosshair.into(),


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
     
      
}


pub fn hud_score_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
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