use raylib::color::Color;
use raylib::prelude::*;

struct GraphicSettings {
    tile_size: u32,
}

struct WindowSettings {
    height: f32,
    width: f32,
}
pub struct Settings {
    graphic: GraphicSettings,
    window: WindowSettings,
}

pub fn draw_grid(d: &mut RaylibDrawHandle, settings: &Settings) {
    // based on the tile size:
    // how many tiles fit horizontally and vertically?
    let tile_size_f32: f32 = settings.graphic.tile_size as f32;
    let target_tile_quantity_horizontally: u32 =
        (settings.window.width / tile_size_f32).ceil() as u32;
    let target_tile_quantity_vertically: u32 =
        (settings.window.height / tile_size_f32).ceil() as u32;

    for n in 0..=target_tile_quantity_horizontally {
        d.draw_line(
            (n * settings.graphic.tile_size).try_into().unwrap(),
            0,
            (n * settings.graphic.tile_size).try_into().unwrap(),
            settings.window.height as i32,
            Color::from_hex("AAAAAA").unwrap(),
        );
    }

    for n in 0..target_tile_quantity_vertically {
        d.draw_line(
            0,
            (n * settings.graphic.tile_size).try_into().unwrap(),
            settings.window.width as i32,
            (n * settings.graphic.tile_size).try_into().unwrap(),
            Color::from_hex("AAAAAA").unwrap(),
        );
    }
}

pub fn draw_static_tile_positions(d: &mut RaylibDrawHandle, settings: &Settings) {
    // based on the tile size:
    // how many tiles fit horizontally and vertically?
    let tile_size_f32: f32 = settings.graphic.tile_size as f32;
    let target_tile_quantity_horizontally: u32 =
        (settings.window.width / tile_size_f32).ceil() as u32;
    let target_tile_quantity_vertically: u32 =
        (settings.window.height / tile_size_f32).ceil() as u32;

    let padding = 5;

    for x in 0..=target_tile_quantity_horizontally {
        for y in 0..=target_tile_quantity_vertically {
            d.draw_text(
                &format!("X: {} \nY: {}", x, y),
                ((x * settings.graphic.tile_size) + padding) as i32,
                ((y * settings.graphic.tile_size) + padding) as i32,
                8,
                Color::WHITE,
            );
        }
    }
}

pub fn draw_world_tile_positions(d: &mut RaylibDrawHandle, settings: &Settings) {
    // based on the tile size:
    // how many tiles fit horizontally and vertically?
    let tile_size_f32: f32 = settings.graphic.tile_size as f32;
    let target_tile_quantity_horizontally: u32 =
        (settings.window.width / tile_size_f32).ceil() as u32;
    let target_tile_quantity_vertically: u32 =
        (settings.window.height / tile_size_f32).ceil() as u32;

    let padding_x = 40;
    let padding_y = 35;

    for x in 0..=target_tile_quantity_horizontally {
        for y in 0..=target_tile_quantity_vertically {
            d.draw_text(
                &format!("X: {} \nY: {}", x, y),
                ((x * settings.graphic.tile_size) + padding_x) as i32,
                ((y * settings.graphic.tile_size) + padding_y) as i32,
                8,
                Color::RED,
            );
        }
    }
}

fn main() {
    let window_height = 640.0;
    let window_width = 640.0;

    let (mut rl, thread) = raylib::init()
        .size(window_height as i32, window_width as i32)
        .title("Poorguelike")
        .build();
    rl.set_target_fps(60);

    let settings = Settings {
        graphic: GraphicSettings { tile_size: 64 },
        window: WindowSettings {
            height: window_height,
            width: window_width,
        },
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        draw_grid(&mut d, &settings);
        draw_static_tile_positions(&mut d, &settings);
        draw_world_tile_positions(&mut d, &settings);
    }
}
