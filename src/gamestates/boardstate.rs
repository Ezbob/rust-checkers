extern crate sdl2;

use crate::gamemachine::runtime_signal::RuntimeSignal;
use crate::gamemachine::state::GameStateTrait;

use crate::assets::GameAssets;
use crate::game_events::WinColorEvent;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::borrow::BorrowMut;

const BOARD_LENGTH: usize = 8;
const BOARD_SIZE: usize = BOARD_LENGTH * BOARD_LENGTH;
const CONTAINER_WIDTH: usize = 100;
const CHECKER_PADDING: usize = 20;
const OUTER_PADDING: usize = 20; // padding from the left most top corner of the screen

trait RectExtras {
    fn clear(&mut self);
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
    red: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Checker {
    GREEN(usize), // sdl2 rect index
    RED(usize),
    NONE,
}

struct RenderRectangles {
    board_tiles: [rect::Rect; BOARD_SIZE],
    black_tiles: [rect::Rect; BOARD_SIZE / 2],
    green_rectangles: [rect::Rect; BOARD_SIZE / 4],
    red_rectangles: [rect::Rect; BOARD_SIZE / 4],
    indicator: rect::Rect,
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
    playing_color: Checker,
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            is_loaded: false,
            renderings: RenderRectangles {
                board_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE],
                black_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE / 2],
                green_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE / 4],
                red_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE / 4],
                indicator: rect::Rect::new(0, 0, 0, 0),
            },
            green_length: 0,
            red_length: 0,
            score: Score { green: 0, red: 0 },
            mouse_point: Point::new(0, 0),
            source_index: None,
            target_index: None,
            cell_mapping: [Checker::NONE; BOARD_SIZE],
            playing_color: Checker::GREEN(0),
        }
    }

    fn switch_turn(&mut self) {
        match self.playing_color {
            Checker::RED(sdl_rect) => self.playing_color = Checker::GREEN(sdl_rect),
            Checker::GREEN(sdl_rect) => self.playing_color = Checker::RED(sdl_rect),
            Checker::NONE => {}
        }
    }

    fn find_source_checker_rect(&self) -> Option<usize> {
        for i in 0..self.renderings.board_tiles.len() {
            let rect = &self.renderings.board_tiles[i];
            if rect.contains_point(self.mouse_point) {
                match &self.cell_mapping[i] {
                    Checker::RED { .. } => {
                        if let Checker::RED { .. } = &self.playing_color {
                            return Some(i);
                        }
                    }
                    Checker::GREEN { .. } => {
                        if let Checker::GREEN { .. } = &self.playing_color {
                            return Some(i);
                        }
                    }
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

    fn move_to_empty(&mut self, source: usize, target: usize) {
        match self.cell_mapping[source] {
            Checker::RED(sdl_rect) => {
                let rct = &mut self.renderings.red_rectangles[sdl_rect];
                rct.move_to(&self.renderings.board_tiles[target]);

                self.cell_mapping[target] = Checker::RED(sdl_rect);
            }
            Checker::GREEN(sdl_rect) => {
                let rct = &mut self.renderings.green_rectangles[sdl_rect];
                rct.move_to(&self.renderings.board_tiles[target]);

                self.cell_mapping[target] = Checker::GREEN(sdl_rect);
            }
            Checker::NONE => {}
        }
        self.cell_mapping[source] = Checker::NONE;
    }

    fn overtake(&mut self, source: usize, victim: usize, end: usize) {
        match self.cell_mapping[victim] {
            Checker::GREEN(sdl_rect) => {
                self.renderings.green_rectangles[sdl_rect]
                    .borrow_mut()
                    .clear();
                self.score.green -= 1;
            }
            Checker::RED(sdl_rect) => {
                self.renderings.red_rectangles[sdl_rect]
                    .borrow_mut()
                    .clear();
                self.score.red -= 1;
            }
            _ => {}
        };
        self.cell_mapping[victim] = Checker::NONE;
        self.move_to_empty(source, end);
    }

    fn try_to_overtake(&mut self, source: usize, victim: usize, end: i32) {
        let is_enemy = match &self.cell_mapping[source] {
            Checker::RED { .. } => {
                if let Checker::GREEN { .. } = &self.cell_mapping[victim] {
                    true
                } else {
                    false
                }
            }
            Checker::GREEN { .. } => {
                if let Checker::RED { .. } = &self.cell_mapping[victim] {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        let is_next_empty = self.cell_mapping[end as usize] == Checker::NONE;

        if is_enemy && is_next_empty {
            self.overtake(source, victim, end as usize);
        }
    }

    fn row_up(pos: usize, n: i32) -> i32 {
        (pos as i32) - (n * BOARD_LENGTH as i32)
    }

    fn row_down(pos: usize, n: i32) -> i32 {
        (pos as i32) + (n * BOARD_LENGTH as i32)
    }

    fn on_left_edge(pos: usize) -> bool {
        (pos as i32) % BOARD_LENGTH as i32 == 0
    }

    fn on_right_edge(pos: usize) -> bool {
        ((pos as i32) + 1) % BOARD_LENGTH as i32 == 0
    }

    fn is_in_bounds(pos: i32) -> bool {
        pos >= 0 && pos < BOARD_SIZE as i32
    }

    fn check_next_down(&mut self, source_pos: usize, target_pos: usize, is_right: bool) {
        let left_right_steps = 2;
        let next_lower = BoardState::row_down(source_pos, left_right_steps);

        if !BoardState::on_right_edge(target_pos) && BoardState::is_in_bounds(next_lower) {
            let x_steps = if is_right {
                left_right_steps
            } else {
                -left_right_steps
            };
            self.try_to_overtake(source_pos, target_pos, next_lower + x_steps);
        }
    }

    fn check_next_up(&mut self, source_pos: usize, target_pos: usize, is_right: bool) {
        let left_right_steps = 2;
        let next_upper = BoardState::row_up(source_pos, left_right_steps);

        if !BoardState::on_right_edge(target_pos) && BoardState::is_in_bounds(next_upper) {
            let x_steps = if is_right {
                left_right_steps
            } else {
                -left_right_steps
            };
            self.try_to_overtake(source_pos, target_pos, next_upper + x_steps);
        }
    }

    fn scan_neighbourhood(&mut self, source_pos: usize, target_pos: usize, i: i32) {
        let lower = BoardState::row_down(source_pos, i);
        let upper = BoardState::row_up(source_pos, i);

        if !BoardState::on_right_edge(source_pos) {
            if BoardState::is_in_bounds(lower) {
                let right_lower = lower + i;
                if self.cell_mapping[target_pos] == Checker::NONE
                    && right_lower == target_pos as i32
                {
                    self.move_to_empty(source_pos, target_pos);
                    self.switch_turn();
                } else if right_lower == target_pos as i32 {
                    self.check_next_down(source_pos, target_pos, true);
                }
            }
            if BoardState::is_in_bounds(upper) {
                let right_upper = upper + i;
                if self.cell_mapping[target_pos] == Checker::NONE
                    && right_upper == target_pos as i32
                {
                    self.move_to_empty(source_pos, target_pos);
                    self.switch_turn();
                } else if right_upper == target_pos as i32 {
                    self.check_next_up(source_pos, target_pos, true);
                }
            }
        }

        if !BoardState::on_left_edge(source_pos) {
            if BoardState::is_in_bounds(lower) {
                let left_lower = lower - i;
                if self.cell_mapping[target_pos] == Checker::NONE && left_lower == target_pos as i32
                {
                    self.move_to_empty(source_pos, target_pos);
                    self.switch_turn();
                } else if left_lower == target_pos as i32 {
                    self.check_next_down(source_pos, target_pos, false);
                }
            }
            if BoardState::is_in_bounds(upper) {
                let left_upper = upper - i;
                if self.cell_mapping[target_pos] == Checker::NONE && left_upper == target_pos as i32
                {
                    self.move_to_empty(source_pos, target_pos);
                    self.switch_turn();
                } else if left_upper == target_pos as i32 {
                    self.check_next_up(source_pos, target_pos, false);
                }
            }
        }
    }
}

impl GameStateTrait for BoardState {
    fn update(&mut self, event: &sdl2::EventSubsystem) -> RuntimeSignal {
        if self.score.red <= 0 || self.score.green <= 0 {
            if self.score.red <= 0 {
                event.push_custom_event(WinColorEvent::new_green()).unwrap();
            } else {
                event.push_custom_event(WinColorEvent::new_red()).unwrap();
            }

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
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
        canvas.draw_rects(&self.renderings.board_tiles)?;
        canvas.fill_rects(&self.renderings.black_tiles)?;

        match self.source_index {
            Some(i) => {
                canvas.set_draw_color(Color::RGB(0x0, 0x0f, 0xfa));
                canvas.fill_rect(self.renderings.board_tiles[i])?;
            }
            None => {}
        };

        canvas.set_draw_color(Color::RGB(0x0, 0xff, 0x0));
        canvas.fill_rects(&self.renderings.green_rectangles)?;

        canvas.set_draw_color(Color::RGB(0xff, 0x0, 0x0));
        canvas.fill_rects(&self.renderings.red_rectangles)?;

        match self.playing_color {
            Checker::RED(..) => {
                canvas.set_draw_color(Color::RGB(0xff, 0x0, 0x0));
            }
            Checker::GREEN(..) => {
                canvas.set_draw_color(Color::RGB(0x0, 0xff, 0x0));
            }
            Checker::NONE => {}
        };

        canvas.fill_rect(self.renderings.indicator)?;

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> RuntimeSignal {
        match event {
            Event::Quit { .. } => return RuntimeSignal::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => RuntimeSignal::GotoState(2),
            Event::MouseButtonDown {
                x,
                y,
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.mouse_point.x = *x;
                self.mouse_point.y = *y;
                match self.source_index {
                    None => {
                        self.source_index = self.find_source_checker_rect();
                    }
                    Some(_) => {
                        self.target_index = self.find_target_rect();
                    }
                };
                RuntimeSignal::Continue
            }
            _ => RuntimeSignal::Continue,
        }
    }

    fn setup(&mut self, _ass: &GameAssets) -> Result<(), String> {
        let mut tile_index = 0;

        {
            let indicator = &mut self.renderings.indicator;
            let right = (CHECKER_PADDING * 3) as i32;
            let bottom = (CONTAINER_WIDTH * 8) as i32;
            indicator.set_width(20);
            indicator.set_height(20);
            indicator.set_right(right);
            indicator.set_bottom(bottom);
        }

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

                self.cell_mapping[flat_index] = Checker::GREEN(self.green_length);
                self.green_length += 1;
            } else if y > (BOARD_LENGTH / 2) {
                // red stuff
                let checker_rect = &mut self.renderings.red_rectangles[self.red_length];
                checker_rect.set_x((container.x() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_y((container.y() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_width((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                checker_rect.set_height((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);

                self.cell_mapping[flat_index] = Checker::RED(self.red_length);
                self.red_length += 1;
            }
        }

        self.score.red = self.red_length;
        self.score.green = self.green_length;

        self.is_loaded = true;
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        self.is_loaded
    }
}
