use macroquad::{prelude::*, rand::gen_range, time};

const TILE_SIZE: (f32, f32) = (8.0, 8.0);
const TILES_NUM: (u32, u32) = (64, 64);

const GRID_COLOR: Color = Color {
    r: (1f32),
    g: (1f32),
    b: (1f32),
    a: (0.25f32),
};
const GRID_THICKNESS: f32 = 0.5;

const _UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const _UP_AND_LEFT: (isize, isize) = (-1, -1);
const _UP_AND_RIGHT: (isize, isize) = (1, -1);
const DOWN_AND_LEFT: (isize, isize) = (-1, 1);
const DOWN_AND_RIGHT: (isize, isize) = (1, 1);

const GRAVITY: f32 = 9.8;

const SAND_COLOR: Color = Color {
    r: (0.96f32),
    g: (0.84f32),
    b: (0.69f32),
    a: (1f32),
};
const WATER_COLOR: Color = Color {
    r: (0.11f32),
    g: (0.64f32),
    b: (0.69f32),
    a: (0.93f32),
};

#[derive(Copy, Clone)]
struct Tile {
    color: Color,
    id: u8,
    tween_pos: (f32, f32),
    acceleration: (f32, f32),
    velocity: (f32, f32),
    updated: bool,
    mass: f32,
}

const DEFAULT_TILE: Tile = Tile {
    color: BLACK,
    id: 0,
    tween_pos: (0.0, 0.0),
    acceleration: (0.0, 0.0),
    velocity: (0.0, 0.0),
    updated: false,
    mass: 1000.0,
};

impl Default for Tile {
    fn default() -> Tile {
        DEFAULT_TILE
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut brush_size: usize = 1;

    let mut map: Vec<Vec<Tile>> =
        vec![vec![DEFAULT_TILE; TILES_NUM.1 as usize]; TILES_NUM.0 as usize];

    let sand_tile = Tile {
        color: SAND_COLOR,
        id: 1,
        acceleration: (0.0, 0.0),
        velocity: (0.0, GRAVITY),
        ..Default::default()
    };
    let water_tile = Tile {
        color: WATER_COLOR,
        id: 2,
        acceleration: (0.0, 0.0),
        velocity: (0.0, GRAVITY),
        ..Default::default()
    };
    let mut current_tile = sand_tile;

    loop {
        let delta_time = time::get_frame_time();

        // PROCESSING
        for y in 0..TILES_NUM.1 {
            for x in 0..TILES_NUM.0 {
                let pos = (x as usize, y as usize);
                let mut tile = &mut map[pos.0][pos.1];
                if tile.id == 0 {
                    continue;
                }
                tile.updated = false;
            }
        }
        for y in 0..TILES_NUM.1 {
            for x in 0..TILES_NUM.0 {
                let pos = (x as usize, y as usize);
                let mut tile = &mut map[pos.0][pos.1];
                if tile.id == 0 {
                    continue;
                }
                if tile.updated {
                    continue;
                }

                tile.acceleration.1 = GRAVITY;
                tile.velocity = vector_add_vector(
                    tile.velocity,
                    vector_multiply(tile.acceleration, delta_time),
                );
                tile.tween_pos =
                    vector_add_vector(tile.tween_pos, vector_multiply(tile.velocity, delta_time));

                if tile.id == 1 {
                    update_sand(pos, &mut map);
                } else if tile.id == 2 {
                    update_water(pos, &mut map);
                }
            }
        }

        // INTERACTION
        let mouse_tile_pos = (
            (mouse_tile_pos().0 as f32 - f32::floor((brush_size as f32 - 1.0) / 2.0)) as usize,
            (mouse_tile_pos().1 as f32 - f32::floor((brush_size as f32 - 1.0) / 2.0)) as usize,
        );
        let mouse_tile_pixel_pos = (
            mouse_tile_pos.0 as f32 * TILE_SIZE.0 as f32,
            mouse_tile_pos.1 as f32 * TILE_SIZE.1 as f32,
        );

        if mouse_tile_pos.0 < TILES_NUM.0 as usize && mouse_tile_pos.1 < TILES_NUM.1 as usize {
            if is_mouse_button_down(MouseButton::Left) {
                for y in 0..brush_size {
                    for x in 0..brush_size {
                        if pos_in_map((mouse_tile_pos.0 + x, mouse_tile_pos.1 + y)) {
                            if map[mouse_tile_pos.0 + x][mouse_tile_pos.1 + y].id == 0 {
                                map[mouse_tile_pos.0 + x][mouse_tile_pos.1 + y] = current_tile;
                            }
                        }
                    }
                }
            }
            if is_mouse_button_down(MouseButton::Right) {
                for y in 0..brush_size {
                    for x in 0..brush_size {
                        if pos_in_map((mouse_tile_pos.0 + x, mouse_tile_pos.1 + y)) {
                            if map[mouse_tile_pos.0 + x][mouse_tile_pos.1 + y].id != 0 {
                                map[mouse_tile_pos.0 + x][mouse_tile_pos.1 + y] = DEFAULT_TILE;
                            }
                        }
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Equal) {
            brush_size += 1;
        } else if is_key_pressed(KeyCode::Minus) && brush_size != 0 {
            brush_size -= 1;
        }

        if is_key_pressed(KeyCode::Key1) {
            current_tile = sand_tile;
        } else if is_key_pressed(KeyCode::Key2) {
            current_tile = water_tile;
        }

        // RENDERING
        clear_background(BLACK);
        draw_bg_grid();
        render_tiles(&mut map);
        draw_rectangle(
            mouse_tile_pixel_pos.0,
            mouse_tile_pixel_pos.1,
            TILE_SIZE.0 * brush_size as f32,
            TILE_SIZE.1 * brush_size as f32,
            WHITE,
        );
        draw_text(
            format!(
                "COORD:{:?} FPS:{} MS:{}",
                mouse_tile_pos,
                time::get_fps(),
                delta_time
            )
            .as_str(),
            0.0,
            TILE_SIZE.1 * TILES_NUM.1 as f32 + TILE_SIZE.1,
            14.0,
            WHITE,
        );
        next_frame().await
    }
}

fn translate(pos: (usize, usize), dir: (isize, isize), map: &mut Vec<Vec<Tile>>) {
    let new_pos = point_add_dir(pos, dir);
    map[new_pos.0][new_pos.1] = map[pos.0][pos.1];
    map[pos.0][pos.1] = DEFAULT_TILE;
}
fn valid_pos(pos: (usize, usize), map: &mut Vec<Vec<Tile>>) -> bool {
    if pos.0 >= TILES_NUM.0 as usize || pos.1 >= TILES_NUM.1 as usize {
        return false;
    }
    if map[pos.0][pos.1].id != 0 {
        return false;
    }
    true
}
fn pos_in_map(pos: (usize, usize)) -> bool {
    if pos.0 >= TILES_NUM.0 as usize || pos.1 >= TILES_NUM.1 as usize {
        return false;
    }
    true
}
fn point_add_dir(pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
    (
        (pos.0 as isize + dir.0) as usize,
        (pos.1 as isize + dir.1) as usize,
    )
}
fn vector_add_vector(vec1: (f32, f32), vec2: (f32, f32)) -> (f32, f32) {
    (vec1.0 + vec2.0, vec1.1 + vec2.1)
}
fn vector_multiply(vec1: (f32, f32), mult: f32) -> (f32, f32) {
    (vec1.0 * mult, vec1.1 * mult)
}

fn update_sand(pos: (usize, usize), map: &mut Vec<Vec<Tile>>) {
    map[pos.0][pos.1].updated = true;
    if f32::abs(map[pos.0][pos.1].tween_pos.0) < 1.0
        && f32::abs(map[pos.0][pos.1].tween_pos.1) < 1.0
    {
        return;
    }
    if map[pos.0][pos.1].tween_pos.1 >= 1.0 {
        let rand_dir = gen_range(0.0f32, 1.0f32).round();
        if valid_pos(point_add_dir(pos, DOWN), map) {
            translate(pos, DOWN, map);
            map[pos.0][pos.1].tween_pos.1 = 0.0;
        } else if rand_dir == 0.0 {
            if valid_pos(point_add_dir(pos, DOWN_AND_RIGHT), map) {
                translate(pos, DOWN_AND_RIGHT, map);
            } else if valid_pos(point_add_dir(pos, DOWN_AND_LEFT), map) {
                translate(pos, DOWN_AND_LEFT, map);
            }
        } else {
            if valid_pos(point_add_dir(pos, DOWN_AND_LEFT), map) {
                translate(pos, DOWN_AND_LEFT, map);
            } else if valid_pos(point_add_dir(pos, DOWN_AND_RIGHT), map) {
                translate(pos, DOWN_AND_RIGHT, map);
            }
        }
    }
}
fn update_water(pos: (usize, usize), map: &mut Vec<Vec<Tile>>) {
    map[pos.0][pos.1].updated = true;
    if f32::abs(map[pos.0][pos.1].tween_pos.0) < 1.0
        && f32::abs(map[pos.0][pos.1].tween_pos.1) < 1.0
    {
        return;
    }
    if map[pos.0][pos.1].tween_pos.1 >= 1.0 {
        let rand_dir = gen_range(0.0f32, 1.0f32).round();
        if valid_pos(point_add_dir(pos, DOWN), map) {
            translate(pos, DOWN, map);
            map[pos.0][pos.1].tween_pos.1 = 0.0;
        } else if rand_dir == 0.0 {
            let rand_dir = gen_range(0.0f32, 1.0f32).round();
            if valid_pos(point_add_dir(pos, DOWN_AND_RIGHT), map) {
                translate(pos, DOWN_AND_RIGHT, map);
            } else if valid_pos(point_add_dir(pos, DOWN_AND_LEFT), map) {
                translate(pos, DOWN_AND_LEFT, map);
            } else if rand_dir == 0.0 {
                if valid_pos(point_add_dir(pos, RIGHT), map) {
                    translate(pos, RIGHT, map);
                } else if valid_pos(point_add_dir(pos, LEFT), map) {
                    translate(pos, LEFT, map);
                }
            } else {
                if valid_pos(point_add_dir(pos, LEFT), map) {
                    translate(pos, LEFT, map);
                } else if valid_pos(point_add_dir(pos, RIGHT), map) {
                    translate(pos, RIGHT, map);
                }
            }
        } else {
            let rand_dir = gen_range(0.0f32, 1.0f32).round();
            if valid_pos(point_add_dir(pos, DOWN_AND_LEFT), map) {
                translate(pos, DOWN_AND_LEFT, map);
            } else if valid_pos(point_add_dir(pos, DOWN_AND_RIGHT), map) {
                translate(pos, DOWN_AND_RIGHT, map);
            } else if rand_dir == 0.0 {
                if valid_pos(point_add_dir(pos, RIGHT), map) {
                    translate(pos, RIGHT, map);
                } else if valid_pos(point_add_dir(pos, LEFT), map) {
                    translate(pos, LEFT, map);
                }
            } else {
                if valid_pos(point_add_dir(pos, LEFT), map) {
                    translate(pos, LEFT, map);
                } else if valid_pos(point_add_dir(pos, RIGHT), map) {
                    translate(pos, RIGHT, map);
                }
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Powder".to_string(),
        window_resizable: false,
        window_width: TILE_SIZE.0 as i32 * TILES_NUM.0 as i32,
        window_height: TILE_SIZE.1 as i32 * (TILES_NUM.1 as i32 + 2i32),
        ..Default::default()
    }
}
fn render_tiles(map: &mut Vec<Vec<Tile>>) {
    for y in 0..TILES_NUM.1 {
        for x in 0..TILES_NUM.0 {
            if map[x as usize][y as usize].id == 0 {
                continue;
            }
            draw_rectangle(
                TILE_SIZE.0 * x as f32,
                TILE_SIZE.1 * y as f32,
                TILE_SIZE.0,
                TILE_SIZE.1,
                map[x as usize][y as usize].color,
            );
        }
    }
}
fn draw_bg_grid() {
    for y in 0..TILES_NUM.1 {
        for x in 0..TILES_NUM.0 {
            draw_rectangle_lines(
                TILE_SIZE.0 * x as f32,
                TILE_SIZE.1 * y as f32,
                TILE_SIZE.0,
                TILE_SIZE.1,
                GRID_THICKNESS,
                GRID_COLOR,
            );
        }
    }
}

fn mouse_tile_pos() -> (usize, usize) {
    let mouse_tile_x = (mouse_position().0 - (mouse_position().0 % TILE_SIZE.0)) as usize;
    let mouse_tile_y = (mouse_position().1 - (mouse_position().1 % TILE_SIZE.1)) as usize;
    (
        mouse_tile_x / TILE_SIZE.0 as usize,
        mouse_tile_y / TILE_SIZE.1 as usize,
    )
}
