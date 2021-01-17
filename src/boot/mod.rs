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
            .add_system(movement_system.system())
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
        .with(FocusingTetrimino);
}

#[derive(Bundle)]
struct Block {
    color: Color,
    pos: (isize, isize),
    sprite: SpriteBundle,
}

const BLOCK_SIZE: f32 = 32.0;

fn movement_system(
    time: Res<Time>,
    mut query: Query<(&FocusingTetrimino, &mut Transform)>,
    key_input: Res<Input<KeyCode>>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut x_direction = 0.0;

        if key_input.just_pressed(KeyCode::Left) {
            x_direction -= BLOCK_SIZE;
        }

        if key_input.just_pressed(KeyCode::Right) {
            x_direction += BLOCK_SIZE;
        }

        let mut y_direction = BLOCK_SIZE;

        if key_input.just_pressed(KeyCode::Up) {
            y_direction += BLOCK_SIZE;
        }

        if key_input.just_pressed(KeyCode::Down) {
            y_direction -= 32.0;
        }

        let translation = &mut transform.translation;

        translation.x += x_direction;
        translation.y += y_direction;

        // TODO: translation.x.min().max();
        // TODO: translation.y.min().max();
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

#[derive(Bundle)]
struct Tetrimino {
    core_pos: (isize, isize),
    rotation: TetriminoRotate,
    core: Block,
    sub1: Block,
    sub2: Block,
    sub3: Block,
}

impl Tetrimino {
    fn oneblock_down(&mut self) {
        // TODO: 衝突処理を作る.
        unimplemented!();
    }

    fn rotate(&mut self, direction: RotateDirection) {
        let direction_is_right = match direction {
            RotateDirection::RIGHT => true,
            RotateDirection::LEFT => false,
        };

        match self.rotation {
            TetriminoRotate::UP => {
                self.rotation = if direction_is_right {
                    TetriminoRotate::RIGHT
                } else {
                    TetriminoRotate::LEFT
                }
            }

            TetriminoRotate::DOWN => {
                self.rotation = if direction_is_right {
                    TetriminoRotate::LEFT
                } else {
                    TetriminoRotate::RIGHT
                }
            }

            TetriminoRotate::LEFT => {
                self.rotation = if direction_is_right {
                    TetriminoRotate::UP
                } else {
                    TetriminoRotate::DOWN
                }
            }

            TetriminoRotate::RIGHT => {
                self.rotation = if direction_is_right {
                    TetriminoRotate::DOWN
                } else {
                    TetriminoRotate::UP
                }
            }
        };

        // TODO: 回転処理を作る.
        unimplemented!();
    }

    fn on_place(self) -> (Block, Block, Block, Block) {
        return (self.core, self.sub1, self.sub2, self.sub3);
    }
}

enum Tetriminos {
    I(Tetrimino),
    O(Tetrimino),
    S(Tetrimino),
    Z(Tetrimino),
    J(Tetrimino),
    L(Tetrimino),
    T(Tetrimino),
}

enum RotateDirection {
    RIGHT,
    LEFT,
}

enum TetriminoRotate {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Tetriminos {
    fn new_i(init_x: isize, init_y: isize) -> Tetriminos {
        return Tetriminos::I(Tetrimino {
            /*
               init -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                   x
                  1o23
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone(), init_y.clone() - 1),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone() + 2, init_y.clone() - 1),
                sprite: Default::default(),
            },
        });
    }

    fn new_o(init_x: isize, init_y: isize) -> Tetriminos {
        return Tetriminos::O(Tetrimino {
            /*
               init(1) -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                   x2
                   c3
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::YELLOW,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::YELLOW,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::YELLOW,
                pos: (init_x.clone() + 1, init_y.clone()),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::YELLOW,
                pos: (init_x + 1, init_y - 1),
                sprite: Default::default(),
            },
        });
    }

    fn new_s(init_x: isize, init_y: isize) -> Tetriminos {
        return Tetriminos::S(Tetrimino {
            /*
               init(sub1) -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                   x2
                  3c
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::GREEN,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::GREEN,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::GREEN,
                pos: (init.clone() + 1, init_y.clone()),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::GREEN,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
        });
    }

    fn new_z(init_x: isize, init_y: isize) -> Tetriminos {
        return Tetriminos::Z(Tetrimino {
            /*
               init(2) -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                  1x
                   c3
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::RED,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::RED,
                pos: (init_x.clone() - 1, init_y.clone()),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::RED,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::RED,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
        });
    }

    fn new_j(init_x: isize, init_y: isize) -> Tetriminos {
        return Tetriminos::J(Tetrimino {
            /*
               init -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                  1x
                  2c3
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::BLUE,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::BLUE,
                pos: (init_x.clone() - 1, init_y.clone()),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::BLUE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::BLUE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
        });
    }

    fn new_l(core_x: isize, core_y: isize) -> Tetriminos {
        return Tetriminos::L(Tetrimino {
            /*
               init -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                   x1
                  2c3
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::ORANGE,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::ORANGE,
                pos: (init_x.clone() + 1, init_y.clone()),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::ORANGE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::ORANGE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
        });
    }

    fn new_t(core_x: isize, core_y: isize) -> Tetriminos {
        return Tetriminos::T(Tetrimino {
            /*
               init(2) -> x
               core -> c
               sub -> 1~3

               y: ?
                  |
                  0

            x: 0123456789
               xxxxOoxxxx
                   x
                  1c3
               */
            core_pos: (init_x.clone(), init_y.clone() - 1),
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::BLUE,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub1: Block {
                color: Color::BLUE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
            sub2: Block {
                color: Color::BLUE,
                pos: (init_x.clone(), init_y.clone()),
                sprite: Default::default(),
            },
            sub3: Block {
                color: Color::BLUE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
                sprite: Default::default(),
            },
        });
    }
}

struct FocusingTetrimino;
