extern crate sdl2;

use crate::gamestate::GameStateTrait;
use crate::gamestate::Signal;

use sdl2::rect;
use sdl2::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;
use std::borrow::BorrowMut;

const BOARD_LENGTH: usize = 8;
const BOARD_SIZE: usize = BOARD_LENGTH * BOARD_LENGTH;
const CONTAINER_WIDTH: usize = 100;
const CHECKER_PADDING: usize = 20;
const OUTER_PADDING: usize = 20; // padding from the left most top corner of the screen

struct Score {
    green: usize,
    red: usize
}

#[derive(Copy, Clone)]
struct BoardCell {
    occupant_index: Option<usize>,
    x: usize,
    y: usize
}

impl BoardCell {
    fn new() -> BoardCell {
        BoardCell {
            occupant_index: None,
            x: 0,
            y: 0
        }
    }
}

pub struct BoardState {
    is_loaded: bool,
    board_tiles: [rect::Rect; BOARD_SIZE],
    black_tiles: [rect::Rect; BOARD_SIZE / 2],
    checker_rectangles: [rect::Rect; BOARD_SIZE],
    cell_mapping: [BoardCell; BOARD_SIZE],
    green_length: usize,
    red_length: usize,
    score: Score,
    mouse_point: Point,
    source_index: Option<usize>,
    target_index: Option<usize>
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            is_loaded: false,
            board_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE],
            black_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE / 2],
            checker_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE],
            green_length: 0,
            red_length: 0,
            score: Score {
                green: 0,
                red: 0
            },
            mouse_point: Point::new(0,0),
            source_index: None,
            target_index: None,
            cell_mapping: [BoardCell::new(); BOARD_SIZE]
        }
    }

    /**
     * Get two mutable references to the same checker_rectangles array
     * In the special case where, we point to the same ref this just returns
     * None
     */
    fn get_double_ref_mut<T>(slice: &mut [T], first_index: usize, second_index: usize)
                           -> Option<(&mut T, &mut T)> {
        let len = slice.len();

        if first_index >= len || second_index >= len || first_index == second_index {
            None
        } else {
            unsafe {
                let ar = &mut *(slice
                    .get_unchecked_mut(first_index) as *mut _);
                let br = &mut *(slice
                    .get_unchecked_mut(second_index) as *mut _);
                Some((ar, br))
            }
        }
    }

    fn find_source_checker_rect(&mut self) -> Option<usize> {
        for i in 0..self.board_tiles.len() {
            let rect = &mut self.board_tiles[i];
            if rect.contains_point(self.mouse_point) {
                match self.cell_mapping[i].occupant_index {
                    Some(_) => return Some(i), // the cell has to be occupied
                    _ => {}
                }
            }
        }
        None
    }

    fn find_target_rect(&mut self) -> Option<usize> {
        for i in 0..self.board_tiles.len() {
            let rect = &mut self.board_tiles[i];
            if rect.contains_point(self.mouse_point) {
                return Some(i);
            }
        }
        None
    }

    fn convert_to_checker_index(&self, other_index: usize) -> usize {
        self.cell_mapping[other_index].occupant_index.unwrap()
    }

    fn move_to_empty(&mut self, source_index: usize, target_index: usize) {
        let (source_map, target_map)
            = BoardState::get_double_ref_mut(&mut self.cell_mapping, source_index, target_index).unwrap();

        let source_rect = &mut self.checker_rectangles[source_map.occupant_index.unwrap()];
        let target_tile = &self.board_tiles[target_index]; // empty so we just use the tiles

        source_rect.x = target_tile.x() + CHECKER_PADDING as i32;
        source_rect.y = target_tile.y() + CHECKER_PADDING as i32;

        target_map.occupant_index = source_map.occupant_index;
        source_map.occupant_index = None;
    }

    fn try_to_move(&mut self, x_offset: i32, y_offset: i32) {
        let cell = &self.cell_mapping[self.source_index.unwrap()];
        let x_next = cell.x as i32 + x_offset;
        let y_next = cell.y as i32 + y_offset;
        let row_length = (BOARD_LENGTH as i32);

        if !((0 <= x_next && x_next <= row_length) &&
            (0 <= y_next && y_next <= row_length)) {
            return;
        }

        let search_index: usize = (x_next + (row_length * y_next)) as usize;
        let container: &Rect = self.board_tiles.get(search_index).unwrap();
        if container.contains_point(self.mouse_point) {
            match self.cell_mapping[search_index].occupant_index {
                Some(i) => {
                    println!("occupied by {} at {}", i, search_index );
                },
                None => self.move_to_empty(self.source_index.unwrap(),search_index)
            };
        }
    }
}

impl GameStateTrait for BoardState {
    fn update(&mut self) -> Signal {
        if self.score.red == 0 || self.score.green == 0 {
            Signal::GotoState(1)
        } else {
            let source_exists = match self.source_index {
                None => false,
                _ => true
            };
            let target_exists = match self.target_index {
                None => false,
                _ => true
            };

            if source_exists && target_exists {
                self.try_to_move(1,1);
                self.try_to_move(-1, 1);
                self.try_to_move(1, -1);
                self.try_to_move(-1,-1);

                self.source_index = None;
                self.target_index = None;
            }
            Signal::Continue
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.fill_rect(canvas.viewport())?;
        canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));

        canvas.draw_rects(&self.board_tiles)?;
        canvas.fill_rects(&self.black_tiles)?;

        match self.source_index {
            Some(i) => {
                canvas.set_draw_color(Color::RGB(0x0, 0x0f, 0xfa));
                canvas.fill_rect(self.board_tiles[i]);
            },
            None => {}
        };

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
            Event::MouseButtonDown {x, y, mouse_btn: MouseButton::Left, ..} => {
                self.mouse_point.x = *x;
                self.mouse_point.y = *y;
                match self.source_index {
                  None => {
                      self.source_index = self.find_source_checker_rect();
                  },
                  Some(_) => {
                      self.target_index = self.find_target_rect();
                  },
                };
                Signal::Continue
            },
            _ => Signal::Continue
        }
    }

    fn load(&mut self) -> Result<(), String> {
        let mut tile_index = 0;

        for y in 0..BOARD_LENGTH {
            for x in 0..BOARD_LENGTH {
                let flat_index = y * BOARD_LENGTH + x;
                let container = &mut self.board_tiles[flat_index];
                let cell = &mut self.cell_mapping[flat_index];

                cell.x = x;
                cell.y = y;

                container.set_x((CONTAINER_WIDTH * x + OUTER_PADDING) as i32);
                container.set_y((CONTAINER_WIDTH * y + OUTER_PADDING) as i32);

                if x % 2 != y % 2 {
                    let black_tile = &mut self.black_tiles[tile_index];
                    black_tile.set_x(container.x());
                    black_tile.set_y(container.y());
                    tile_index += 1;
                }

                if y % 2 == x % 2 && y < (BOARD_LENGTH / 2 - 1) {
                    // green stuff
                    let checker_rect = &mut self.checker_rectangles[self.green_length];
                    checker_rect.set_x((container.x() + CHECKER_PADDING as i32) as i32);
                    checker_rect.set_y((container.y() + CHECKER_PADDING as i32) as i32);
                    checker_rect.set_width((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                    checker_rect.set_height((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                    cell.occupant_index = Some(self.green_length);
                    self.green_length += 1;
                } else if y % 2 == x % 2 && y > (BOARD_LENGTH / 2) {
                    // red stuff
                    let index = self.green_length + self.red_length;
                    let checker_rect = &mut self.checker_rectangles[index];
                    checker_rect.set_x((container.x() + CHECKER_PADDING as i32) as i32);
                    checker_rect.set_y((container.y() + CHECKER_PADDING as i32) as i32);
                    checker_rect.set_width((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                    checker_rect.set_height((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                    cell.occupant_index = Some(index);
                    self.red_length += 1;
                }
            }
        }

        self.score.red = self.red_length;
        self.score.green = self.green_length;

        self.is_loaded = true;
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

