use bevy::prelude::*;
use std::collections::HashMap;

pub struct KarpasBuilder;

impl KarpasBuilder {
    pub fn build(self) -> Result<Karpas, ()> {
        return Ok(Karpas);
    }
}

pub struct Karpas;

impl Karpas {
    pub fn boot(self) -> Result<(), ()> {
        // TODO: impl for AppBuilderの関数一発で設定をできるように分離しよう
        App::build()
            .add_plugins(DefaultPlugins)
            .init_resource::<ButtonMaterials>()
            .add_resource(TextureHandlerServer {
                container: HashMap::new(),
            })
            .add_startup_system(load_texture.system())
            .add_startup_system(setup.system())
            .add_startup_system(window_setup.system())
            .add_system(button_system.system())
            .add_system(moving.system())
            .run();

        return Ok(());
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        return ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        };
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string().into();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string().into();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.value = "Button".to_string().into();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    texture_server: Res<TextureHandlerServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Button".to_string(),
                    font: asset_server.load("fonts.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            material: color_materials.add(ColorMaterial {
                texture: Some(texture_server.container.get(&1i16).unwrap().clone()),
                color: Color::DARK_GREEN,
            }),
            ..Default::default()
        })
        .with(Mino);
}

struct Mino;

fn moving(time: Res<Time>, mut query: Query<(&Mino, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.x = transform.translation.x + (time.delta_seconds() * 100.0)
    }
}

fn window_setup(mut windows: ResMut<Windows>) {
    // TODO: windowのアイコンを指定する
    // TODO: windowのアスペクト比, 大きさを指定しましょう
    let window = windows.get_primary_mut().unwrap();
    window.set_vsync(true);
    window.set_title("Karpas, so yummy.".to_string());
}

fn load_texture(
    asset_server: Res<AssetServer>,
    mut handler_server: ResMut<TextureHandlerServer>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // TODO: 何らかの方式(テキスト)でassetsを管理し, 管理ファイルから読み込もう
    [(1i16, "mino.png")].iter().for_each(|(num, path)| {
        let texture = asset_server.load(*path);
        handler_server.container.insert(*num, texture);
    });
}

struct TextureHandlerServer {
    container: HashMap<i16, Handle<Texture>>,
}
