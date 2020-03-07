extern crate sdl2;

use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::runtime_signal::RuntimeSignal;

use sdl2::rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;
use crate::gamemachine::resource::ExtensionLibraries;

const BOARD_LENGTH: usize = 8;
const BOARD_SIZE: usize = BOARD_LENGTH * BOARD_LENGTH;
const CONTAINER_WIDTH: usize = 100;
const CHECKER_PADDING: usize = 20;
const OUTER_PADDING: usize = 20; // padding from the left most top corner of the screen

trait RectExtras {
    fn clear(& mut self);
    fn move_to(&mut self, rect: &rect::Rect);
}

impl RectExtras for rect::Rect {
    fn clear(&mut self) {
        self.set_x(0);
        self.set_y(0);
        self.set_width(0);
        self.set_height(0);
    }

    fn move_to(&mut self, rect: &rect::Rect) {
        self.set_x(rect.x() + CHECKER_PADDING as i32);
        self.set_y(rect.y() + CHECKER_PADDING as i32);
    }
}

struct Score {
    green: usize,
    red: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Checker {
    GREEN { sdl_rect: usize, container: usize }, // sdl2 rect index
    RED { sdl_rect: usize, container: usize },
    NONE
}

struct RenderRectangles {
    board_tiles: [rect::Rect; BOARD_SIZE],
    black_tiles: [rect::Rect; BOARD_SIZE / 2],
    green_rectangles: [rect::Rect; BOARD_SIZE / 4],
    red_rectangles: [rect::Rect; BOARD_SIZE / 4]
}

pub struct BoardState {
    is_loaded: bool,
    renderings: RenderRectangles,

    cell_mapping: [Checker; BOARD_SIZE],
    green_length: usize,
    red_length: usize,
    score: Score,
    mouse_point: Point,
    source_index: Option<usize>,
    target_index: Option<usize>,
    playing_color: Checker
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            is_loaded: false,
            renderings: RenderRectangles {
                board_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE],
                black_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE / 2],
                green_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE / 4],
                red_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE / 4]
            },
            green_length: 0,
            red_length: 0,
            score: Score {
                green: 0,
                red: 0
            },
            mouse_point: Point::new(0,0),
            source_index: None,
            target_index: None,
            cell_mapping: [Checker::NONE; BOARD_SIZE],
            playing_color: Checker::GREEN { sdl_rect : 0, container: 0 }
        }
    }

    fn switch_turn(&mut self) {
        match self.playing_color {
            Checker::RED { sdl_rect, container} => self.playing_color = Checker::GREEN { sdl_rect, container },
            Checker::GREEN { sdl_rect, container} => self.playing_color = Checker::RED { sdl_rect, container},
            Checker::NONE => {}
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

    fn get_triple_ref_mut<T>(slice: &mut [T], first_index: usize, second_index: usize, third_index: usize)
                             -> Option<(&mut T, &mut T, &mut T)> {
        let len = slice.len();

        if first_index >= len || second_index >= len || third_index >= len
            || first_index == second_index
            || first_index == third_index
            || second_index == third_index {
            None
        } else {
            unsafe {
                let ar = &mut *(slice
                    .get_unchecked_mut(first_index) as *mut _);
                let br = &mut *(slice
                    .get_unchecked_mut(second_index) as *mut _);
                let cr = &mut *(slice
                    .get_unchecked_mut(third_index) as *mut _);
                Some((ar, br, cr))
            }
        }
    }

    fn find_source_checker_rect(&self) -> Option<usize> {
        for i in 0..self.renderings.board_tiles.len() {
            let rect = &self.renderings.board_tiles[i];
            if rect.contains_point(self.mouse_point) {

                match &self.cell_mapping[i] {
                    Checker::RED { .. }  => {
                        if let Checker::RED { .. } = &self.playing_color {
                            return Some(i);
                        }
                    },
                    Checker::GREEN { .. } => {
                        if let Checker::GREEN { .. } = &self.playing_color {
                            return Some(i);
                        }
                    },
                    _ => {}
                }
            }
        }
        None
    }

    fn find_target_rect(&mut self) -> Option<usize> {
        for i in 0..self.renderings.board_tiles.len() {
            let rect = &mut self.renderings.board_tiles[i];
            if rect.contains_point(self.mouse_point) {
                return Some(i);
            }
        }
        None
    }

    /*
    fn move_to_empty(&mut self, source_index: usize, target_index: usize) {
        let (source_map, target_map)
            = BoardState::get_double_ref_mut(&mut self.cell_mapping, source_index, target_index).unwrap();

        let rect_render_index = match source_map.occupant {
            Checker::GREEN(render) => render,
            Checker::RED(render) => render,
            Checker::NONE => panic!("No source index for move_to_empty")
        };

        let source_rect = &mut self.renderings.checker_rectangles[rect_render_index];
        let target_tile = &self.board_tiles[target_index]; // empty so we just use the tiles

        source_rect.move_to(target_tile);

        target_map.occupant = source_map.occupant;
        source_map.occupant = Checker::NONE;
        self.switch_turn()
    }

    fn overtake(&mut self, source_index: usize, target_index: usize, destination_index: usize) {
        let (source_cell, target_cell, destination_cell) =
            BoardState::get_triple_ref_mut(&mut self.cell_mapping,
                                    source_index, target_index, destination_index).unwrap();

        let (source_rect, target_rect) =
            BoardState::get_double_ref_mut(&mut self.checker_rectangles,
                                           source_cell.occupant.unwrap().rect_render_index,
                                            target_cell.occupant.unwrap().rect_render_index).unwrap();

        let destination_tile = &self.board_tiles[destination_index];

        target_rect.clear();
        source_rect.move_to(destination_tile);

        destination_cell.occupant = source_cell.occupant;
        target_cell.occupant = Checker::NONE;
        source_cell.occupant = Checker::NONE;
    }

    fn try_to_overtake(&mut self, target_index: usize, x_offset: i32, y_offset: i32) {
        let clicked_cell = &self.cell_mapping[target_index];
        let row_length = BOARD_LENGTH as i32;
        let x_next = clicked_cell.x as i32 + x_offset;
        let y_next = clicked_cell.y as i32 + y_offset;

        if !((0 <= x_next && x_next <= row_length) &&
            (0 <= y_next && y_next <= row_length)) {
            return;
        }

        let search_index: usize = (x_next + (row_length * y_next)) as usize;

        match self.cell_mapping[search_index].occupant {
            Checker::NONE => self.overtake(self.source_index.unwrap(),
                                  self.target_index.unwrap(),
                                  search_index),
            _ => {}
        }
    }

    fn try_to_move(&mut self, x_offset: i32, y_offset: i32) {
        let cell = &self.cell_mapping[self.source_index.unwrap()];
        let x_next = cell.x as i32 + x_offset;
        let y_next = cell.y as i32 + y_offset;
        let row_length = BOARD_LENGTH as i32;
        let search_index = (x_next + (row_length * y_next)) as usize;

        if !(0 <= search_index && search_index <= BOARD_SIZE) {
            return;
        }

        let container = self.board_tiles.get(search_index).expect(format!("not found at {}, tiles length {}", search_index, self.board_tiles.len()).as_str());

        if container.contains_point(self.mouse_point) {
            match &self.cell_mapping[search_index].occupant {
                Checker::RED(_) => {
                    if cell.occupant == Checker::GREEN {
                        self.try_to_overtake(search_index, x_offset, y_offset);
                    }
                },
                Checker::GREEN(_) => {
                    if cell.occupant == Checker::RED {
                        self.try_to_overtake(search_index, x_offset, y_offset);
                    }
                },
                Checker::NONE => self.move_to_empty(self.source_index.unwrap(),search_index)
            };
        }
    }
    */

    fn move_to_empty(&mut self, source: usize, target: usize) {

        match self.cell_mapping[source] {
            Checker::RED { sdl_rect, .. } => {
                let rct = &mut self.renderings.red_rectangles[sdl_rect];
                rct.move_to(&self.renderings.board_tiles[target]);

                self.cell_mapping[target] = Checker::RED {
                    sdl_rect, container: target
                };
            },
            Checker::GREEN { sdl_rect, .. } => {
                let rct = &mut self.renderings.green_rectangles[sdl_rect];
                rct.move_to(&self.renderings.board_tiles[target]);

                self.cell_mapping[target] = Checker::GREEN {
                    sdl_rect, container: target
                };
            },
            Checker::NONE => {}
        }

        self.cell_mapping[source] = Checker::NONE;
        self.switch_turn();
    }

    fn try_to_move(&mut self, source: usize, target: usize, try_n: i32) {
        let target_n = target as i32;
        if target_n == try_n {
            match &self.cell_mapping[target] {
                Checker::NONE => {
                    self.move_to_empty(source, target);
                }
                _ => {}
            }
        }
    }

    fn scan_neighbourhood(&mut self, source_pos: usize, target_pos: usize,  i: i32) {
        let lower = (source_pos as i32) + (i * BOARD_LENGTH as i32);
        let upper = (source_pos as i32) - (i * BOARD_LENGTH as i32);

        let is_on_right_edge = ((source_pos as i32) + 1) % BOARD_LENGTH as i32 == 0;
        let is_on_left_edge = (source_pos as i32) % BOARD_LENGTH as i32 == 0;
        let is_in_lower_bound = lower >= 0;
        let is_in_upper_bound = upper < (BOARD_SIZE as i32);

        if !is_on_right_edge {
            //println!("right is not edge");
            if is_in_lower_bound {
                let right_lower = lower + i;
                self.try_to_move(source_pos,target_pos, right_lower);
            }
            if is_in_upper_bound {
                let right_upper = upper + i;
                self.try_to_move(source_pos,target_pos, right_upper);
            }
        }

        if !is_on_left_edge {
            //println!("Left is not edge");
            if is_in_lower_bound {
                let left_lower = lower - i;
                self.try_to_move(source_pos,target_pos, left_lower);
            }
            if is_in_upper_bound {
                let left_upper = upper - i;
                self.try_to_move(source_pos,target_pos, left_upper);
            }
        }
    }
}

impl GameStateTrait for BoardState {
    fn update(&mut self) -> RuntimeSignal {
        if self.score.red == 0 || self.score.green == 0 {
            RuntimeSignal::GotoState(1)
        } else {

            if self.source_index != None && self.target_index != None {
                self.scan_neighbourhood(self.source_index.unwrap(), self.target_index.unwrap(), 1);

                self.source_index = None;
                self.target_index = None;
            }
            RuntimeSignal::Continue
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.fill_rect(canvas.viewport())?;
        canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));

        canvas.draw_rects(&self.renderings.board_tiles)?;
        canvas.fill_rects(&self.renderings.black_tiles)?;

        match self.source_index {
            Some(i) => {
                canvas.set_draw_color(Color::RGB(0x0, 0x0f, 0xfa));
                canvas.fill_rect(self.renderings.board_tiles[i])?;
            },
            None => {}
        };

        canvas.set_draw_color(Color::RGB(0x0, 0xff, 0x0));
        canvas.fill_rects(&self.renderings.green_rectangles)?;

        canvas.set_draw_color(Color::RGB(0xff, 0x0, 0x0));
        canvas.fill_rects(&self.renderings.red_rectangles)?;

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> RuntimeSignal {
        match event {
            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => RuntimeSignal::Quit,
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
                RuntimeSignal::Continue
            },
            _ => RuntimeSignal::Continue
        }
    }

    fn load(&mut self, _libs: &mut ExtensionLibraries) -> Result<(), String> {
        let mut tile_index = 0;

        for flat_index in 0..BOARD_SIZE {
            let x = flat_index % BOARD_LENGTH;
            let y = flat_index / BOARD_LENGTH;
            let container = &mut self.renderings.board_tiles[flat_index];

            container.set_x((CONTAINER_WIDTH * x + OUTER_PADDING) as i32);
            container.set_y((CONTAINER_WIDTH * y + OUTER_PADDING) as i32);

            if x % 2 != y % 2 {
                // black tiles
                let black_tile = &mut self.renderings.black_tiles[tile_index];
                black_tile.set_x(container.x());
                black_tile.set_y(container.y());

                tile_index += 1;
            } else if y < (BOARD_LENGTH / 2 - 1) {
                // green stuff
                let checker_rect = &mut self.renderings.green_rectangles[self.green_length];
                checker_rect.set_x((container.x() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_y((container.y() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_width((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                checker_rect.set_height((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);

                self.cell_mapping[flat_index] = Checker::GREEN { sdl_rect: self.green_length, container: flat_index };
                self.green_length += 1;
            } else if y > (BOARD_LENGTH / 2) {
                // red stuff
                let checker_rect = &mut self.renderings.red_rectangles[self.red_length];
                checker_rect.set_x((container.x() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_y((container.y() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_width((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                checker_rect.set_height((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);

                self.cell_mapping[flat_index] = Checker::RED { sdl_rect: self.red_length, container: flat_index };
                self.red_length += 1;
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

