extern crate sdl2;

use crate::gamestate::GameStateTrait;
use crate::gamestate::Signal;

use sdl2::rect;
use sdl2::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const BOARD_LENGTH: usize = 8;
const BOARD_SIZE: usize = BOARD_LENGTH * BOARD_LENGTH;

pub struct BoardState {
    is_loaded: bool,
    board_tiles: [rect::Rect; BOARD_SIZE],
    black_tiles: [rect::Rect; BOARD_SIZE / 2],
    checker_rectangles: [rect::Rect; BOARD_SIZE],
    green_length: usize,
    red_length: usize
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            is_loaded: false,
            board_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE],
            black_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE / 2],
            checker_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE],
            green_length: 0,
            red_length: 0
        }
    }
}

impl GameStateTrait for BoardState {
    fn update(&mut self) -> Signal {
        Signal::Continue
    }

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.fill_rect(canvas.viewport())?;
        canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));

        canvas.draw_rects(&self.board_tiles)?;
        canvas.fill_rects(&self.black_tiles)?;

        canvas.set_draw_color(Color::RGB(0x0, 0xff, 0x0));
        let green_rectangles = &self.checker_rectangles[0..self.green_length];
        canvas.fill_rects(green_rectangles)?;

        canvas.set_draw_color(Color::RGB(0xff, 0x0, 0x0));
        let red_rectangles = &self.checker_rectangles[self.green_length..(self.red_length + self.green_length)];
        canvas.fill_rects(red_rectangles)?;

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Signal {
        match event {
            Event::Quit {..}
            | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => Signal::Quit,
            _ => Signal::Continue
        }
    }

    fn load(&mut self) -> Result<(), String> {
        let mut tile_index = 0;
        for y in 0..BOARD_LENGTH {
            for x in 0..BOARD_LENGTH {
                let flat_index = y * BOARD_LENGTH + x;
                let container = &mut self.board_tiles[flat_index];

                container.set_x((100 * (flat_index % BOARD_LENGTH) + 20) as i32);
                container.set_y((100 * (flat_index / BOARD_LENGTH) + 20) as i32);

                if (x % 2 == 1 && y % 2 == 0) || (x % 2 == 0 && y % 2 == 1) {
                    let black_tile = &mut self.black_tiles[tile_index];
                    black_tile.set_x(container.x());
                    black_tile.set_y(container.y());
                    tile_index += 1;
                }

                if y % 2 == 0 && y < (BOARD_LENGTH / 2) && x % 2 == 0 {
                    let checker_rect = &mut self.checker_rectangles[self.green_length];
                    checker_rect.set_x((100 * (x % BOARD_LENGTH) + 40) as i32);
                    checker_rect.set_y((container.y() + 20) as i32);
                    checker_rect.set_width(60);
                    checker_rect.set_height(60);
                    self.green_length += 1;
                    continue;
                } else if y % 2 != 0 && y < (BOARD_LENGTH / 2 - 1) && x % 2 != 0 {
                    let checker_rect = &mut self.checker_rectangles[self.green_length];
                    checker_rect.set_x((100 * (x % BOARD_LENGTH) + 40) as i32);
                    checker_rect.set_y((container.y() + 20) as i32);
                    checker_rect.set_width(60);
                    checker_rect.set_height(60);
                    self.green_length += 1;
                    continue;
                }

                if y % 2 == 0 && y > (BOARD_LENGTH / 2) && x % 2 == 0 {
                    let current_index = self.green_length + self.red_length;
                    let checker_rect = &mut self.checker_rectangles[current_index];
                    checker_rect.set_x((100 * (x % BOARD_LENGTH) + 40) as i32);
                    checker_rect.set_y((container.y() + 20) as i32);
                    checker_rect.set_width(60);
                    checker_rect.set_height(60);
                    self.red_length += 1;
                } else if y % 2 != 0 && y > (BOARD_LENGTH / 2) && x % 2 != 0 {
                    let current_index = self.green_length + self.red_length;
                    let checker_rect = &mut self.checker_rectangles[current_index];
                    checker_rect.set_x((100 * (x % BOARD_LENGTH) + 40) as i32);
                    checker_rect.set_y((container.y() + 20) as i32);
                    checker_rect.set_width(60);
                    checker_rect.set_height(60);
                    self.red_length += 1;
                }
            }
        }

        self.is_loaded = true;
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

