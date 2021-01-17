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
            .add_resource(GridColumns(10, 20))
            .add_startup_system(grid_setup.system())
            .add_startup_system(wall_setup.system())
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

#[derive(Clone)]
struct GridColumns(i32, i32);

struct Grid(Vec<Vec<Option<Block>>>);

#[derive(Bundle)]
struct GridBundle {
    // won't fix FIXME: this is tetris?
    grid: Grid,
}

const GRID_SIZE: f32 = 32.0;
const WALL_THICKNESS: f32 = 10.0;

fn grid_setup(commands: &mut Commands, grid_columns: Res<GridColumns>) {
    let GridColumns(x_size, y_size) = grid_columns.clone();

    commands.spawn(GridBundle {
        grid: Grid(vec![vec![None; x_size as usize]; y_size as usize]),
    });
}

fn wall_setup(
    commands: &mut Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Grid>,
) {
    let wall_material = color_materials.add(ColorMaterial::from(Color::GRAY));
    let wall_offset = WALL_THICKNESS / 2.0;

    query.iter().for_each(|Grid(grid)| {
        let y_size = grid.len();
        let x_size = grid[0].len();

        let width = x_size as f32 * GRID_SIZE;
        let height = y_size as f32 * GRID_SIZE;

        let x_wall_translation = (width / 2.0) + wall_offset;
        let y_wall_translation = (height / 2.0) + wall_offset;

        commands
            .spawn(Camera2dBundle::default())
            .spawn(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(x_wall_translation, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(WALL_THICKNESS, height)),
                ..Default::default()
            })
            .spawn(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(-x_wall_translation, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(WALL_THICKNESS, height)),
                ..Default::default()
            })
            .spawn(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -y_wall_translation, 0.0)),
                sprite: Sprite::new(Vec2::new(width, WALL_THICKNESS)),
                ..Default::default()
            });
    });
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
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(16.0, 16.0, 0.0),
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

#[derive(Clone)]
struct Block {
    color: Color,
}

const BLOCK_SIZE: f32 = 32.0;

fn movement_system(
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

        let mut y_direction = 0.0;

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

/*
#[derive(Bundle)]
struct Tetrimino {
    rotation: TetriminoRotate,
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

// 4x4衝突
// 4x4回転
// SRS検証

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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone(), init_y.clone() - 1),
            },
            sub1: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
            },
            sub2: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
            },
            sub3: Block {
                color: Color::ALICE_BLUE,
                pos: (init_x.clone() + 2, init_y.clone() - 1),
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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::YELLOW,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub1: Block {
                color: Color::YELLOW,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub2: Block {
                color: Color::YELLOW,
                pos: (init_x.clone() + 1, init_y.clone()),
            },
            sub3: Block {
                color: Color::YELLOW,
                pos: (init_x + 1, init_y - 1),
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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::GREEN,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub1: Block {
                color: Color::GREEN,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub2: Block {
                color: Color::GREEN,
                pos: (init_x.clone() + 1, init_y.clone()),
            },
            sub3: Block {
                color: Color::GREEN,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::RED,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub1: Block {
                color: Color::RED,
                pos: (init_x.clone() - 1, init_y.clone()),
            },
            sub2: Block {
                color: Color::RED,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub3: Block {
                color: Color::RED,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::BLUE,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub1: Block {
                color: Color::BLUE,
                pos: (init_x.clone() - 1, init_y.clone()),
            },
            sub2: Block {
                color: Color::BLUE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
            },
            sub3: Block {
                color: Color::BLUE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
            },
        });
    }

    fn new_l(init_x: isize, init_y: isize) -> Tetriminos {
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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::ORANGE,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub1: Block {
                color: Color::ORANGE,
                pos: (init_x.clone() + 1, init_y.clone()),
            },
            sub2: Block {
                color: Color::ORANGE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
            },
            sub3: Block {
                color: Color::ORANGE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
            },
        });
    }

    fn new_t(init_x: isize, init_y: isize) -> Tetriminos {
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
            rotation: TetriminoRotate::UP,
            core: Block {
                color: Color::PURPLE,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub1: Block {
                color: Color::PURPLE,
                pos: (init_x.clone() - 1, init_y.clone() - 1),
            },
            sub2: Block {
                color: Color::PURPLE,
                pos: (init_x.clone(), init_y.clone()),
            },
            sub3: Block {
                color: Color::PURPLE,
                pos: (init_x.clone() + 1, init_y.clone() - 1),
            },
        });
    }
}

*/

struct FocusingTetrimino;
