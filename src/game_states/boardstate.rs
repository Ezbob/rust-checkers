extern crate sdl2;

use crate::game_machine::runtime_signal::RuntimeSignal;
use crate::game_machine::state::GameStateTrait;

use crate::asset_loader::{Assets, TextureManager};
use crate::game_events::WinColorEvent;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::rect::Point;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};
use std::borrow::BorrowMut;

const BOARD_LENGTH: usize = 8;
const BOARD_SIZE: usize = BOARD_LENGTH * BOARD_LENGTH;
const CONTAINER_WIDTH: usize = 100;
const CHECKER_PADDING: usize = 20;
const OUTER_PADDING: usize = 20; // padding from the left most top corner of the screen

trait RectExtras {
    fn clear(&mut self);
    fn move_to(&mut self, rect: &rect::Rect);
    fn move_yellow(&mut self, rect: &rect::Rect);
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

    fn move_yellow(&mut self, rect: &rect::Rect) {
        self.set_x(rect.x() + CHECKER_PADDING as i32);
        self.set_y(rect.y() + CHECKER_PADDING as i32);
    }
}

struct Score {
    green: usize,
    red: usize,
}

impl Score {
    fn new() -> Score {
        Score { green: 0, red: 0 }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Checker {
    Green(usize), // sdl2 rect index
    Red(usize),
    SuperGreen(usize),
    SuperRed(usize),
    None,
}

impl Checker {
    fn is_super(&self) -> bool {
        match self {
            Checker::SuperRed(..) | Checker::SuperGreen(..) => true,
            _ => false,
        }
    }

    fn is_green(&self) -> bool {
        match self {
            Checker::SuperGreen(..) | Checker::Green(..) => true,
            _ => false,
        }
    }

    fn is_red(&self) -> bool {
        match self {
            Checker::SuperRed(..) | Checker::Red(..) => true,
            _ => false,
        }
    }

    fn is_none(&self) -> bool {
        match self {
            Checker::None => true,
            _ => false,
        }
    }

    fn is_some(&self) -> bool {
        !self.is_none()
    }

    fn is_same_color(&self, other: &Checker) -> bool {
        (self.is_red() && other.is_red())
            || (self.is_green() && other.is_green())
            || (self.is_none() && other.is_none())
    }

    fn is_different_color(&self, other: &Checker) -> bool {
        !self.is_same_color(other)
    }
}

struct RenderRectangles {
    board_tiles: [rect::Rect; BOARD_SIZE],
    black_tiles: [rect::Rect; BOARD_SIZE / 2],

    green_rectangles: [rect::Rect; BOARD_SIZE / 4],
    red_rectangles: [rect::Rect; BOARD_SIZE / 4],

    yellow_rectangles: [rect::Rect; BOARD_SIZE],

    indicator: rect::Rect,
    debug_tile_text: [rect::Rect; BOARD_SIZE],
}

impl RenderRectangles {
    fn new() -> RenderRectangles {
        RenderRectangles {
            board_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE],
            black_tiles: [rect::Rect::new(0, 0, 100, 100); BOARD_SIZE / 2],
            green_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE / 4],
            red_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE / 4],
            indicator: rect::Rect::new(0, 0, 0, 0),
            debug_tile_text: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE],
            yellow_rectangles: [rect::Rect::new(0, 0, 0, 0); BOARD_SIZE],
        }
    }
}

pub struct BoardState<'ttf> {
    is_set_up: bool,
    renderings: RenderRectangles,
    cell_mapping: [Checker; BOARD_SIZE],
    green_length: usize,
    red_length: usize,
    score: Score,
    mouse_point: Point,
    source_index: Option<usize>,
    target_index: Option<usize>,
    playing_color: Checker,
    texture_manager: TextureManager<'ttf>,
}

fn is_in_bounds(pos: i32) -> bool {
    pos >= 0 && pos < BOARD_SIZE as i32
}

fn on_left_edge(pos: usize) -> bool {
    (pos as i32) % BOARD_LENGTH as i32 == 0
}

fn on_right_edge(pos: usize) -> bool {
    ((pos as i32) + 1) % BOARD_LENGTH as i32 == 0
}

fn row_up(pos: usize, n: i32) -> i32 {
    (pos as i32) - (n * BOARD_LENGTH as i32)
}

fn row_down(pos: usize, n: i32) -> i32 {
    (pos as i32) + (n * BOARD_LENGTH as i32)
}

fn yellow_for_red(i: usize) -> usize {
    BOARD_LENGTH + i
}

fn yellow_for_green(i: usize) -> usize {
    i
}

fn on_last_line(i: usize) -> bool {
    (BOARD_SIZE - BOARD_LENGTH) <= i && i < BOARD_SIZE
}

fn on_first_line(i: usize) -> bool {
    i <= BOARD_LENGTH
}

impl<'ttf> BoardState<'ttf> {
    pub fn new(t_creator: &'ttf TextureCreator<WindowContext>) -> BoardState<'ttf> {
        BoardState {
            is_set_up: false,
            renderings: RenderRectangles::new(),
            green_length: 0,
            red_length: 0,
            score: Score::new(),
            mouse_point: Point::new(0, 0),
            source_index: None,
            target_index: None,
            cell_mapping: [Checker::None; BOARD_SIZE],
            playing_color: Checker::Green(0),
            texture_manager: TextureManager::new(t_creator),
        }
    }

    fn get_source_checker_ref(&self) -> Option<(&Checker, usize)> {
        if let Some(i) = self.source_index {
            Some((&self.cell_mapping[i], i))
        } else {
            None
        }
    }

    fn switch_turn(&mut self) {
        match self.playing_color {
            Checker::Red(sdl_rect) => self.playing_color = Checker::Green(sdl_rect),
            Checker::Green(sdl_rect) => self.playing_color = Checker::Red(sdl_rect),
            _ => {}
        }
    }

    fn find_source_checker_rect(&self) -> Option<usize> {
        for i in 0..self.renderings.board_tiles.len() {
            let rect = &self.renderings.board_tiles[i];
            if rect.contains_point(self.mouse_point) {
                let cell = &self.cell_mapping[i];
                if cell.is_same_color(&self.playing_color) {
                    return Some(i);
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
            Checker::Red(sdl_rect) => {
                let rct = &mut self.renderings.red_rectangles[sdl_rect];
                rct.move_to(&self.renderings.board_tiles[target]);

                if on_first_line(target) {
                    // first line
                    {
                        self.cell_mapping[target] = Checker::SuperRed(sdl_rect);
                    }
                    if let Some(a) = self
                        .renderings
                        .yellow_rectangles
                        .get_mut(yellow_for_red(sdl_rect))
                    {
                        a.set_x(rct.x());
                        a.set_y(rct.y());
                        a.set_width(20);
                        a.set_height(20);
                    }
                } else {
                    self.cell_mapping[target] = Checker::Red(sdl_rect);
                }
            }
            Checker::Green(sdl_rect) => {
                let rct = &mut self.renderings.green_rectangles[sdl_rect];
                rct.move_to(&self.renderings.board_tiles[target]);

                if on_last_line(target) {
                    // last line
                    {
                        self.cell_mapping[target] = Checker::SuperGreen(sdl_rect);
                    }

                    if let Some(a) = self
                        .renderings
                        .yellow_rectangles
                        .get_mut(yellow_for_green(sdl_rect))
                    {
                        a.set_x(rct.x());
                        a.set_y(rct.y());
                        a.set_width(20);
                        a.set_height(20);
                    }
                } else {
                    self.cell_mapping[target] = Checker::Green(sdl_rect);
                }
            }
            Checker::SuperGreen(sdl_rect) => {
                let target_rct = &self.renderings.board_tiles[target];
                {
                    let rct = &mut self.renderings.green_rectangles[sdl_rect];
                    rct.move_to(target_rct);
                }

                {
                    let rct = &mut self.renderings.yellow_rectangles[yellow_for_green(sdl_rect)];
                    rct.move_yellow(target_rct);
                }

                self.cell_mapping[target] = Checker::SuperGreen(sdl_rect);
            }
            Checker::SuperRed(sdl_rect) => {
                let target_rct = &self.renderings.board_tiles[target];
                {
                    let rct = &mut self.renderings.red_rectangles[sdl_rect];
                    rct.move_to(target_rct);
                }

                {
                    let rct = &mut self.renderings.yellow_rectangles[yellow_for_red(sdl_rect)];
                    rct.move_yellow(target_rct);
                }

                self.cell_mapping[target] = Checker::SuperRed(sdl_rect);
            }
            _ => {}
        }
        self.cell_mapping[source] = Checker::None;
    }

    fn overtake(&mut self, source: usize, victim: usize, end: usize) {
        match self.cell_mapping[victim] {
            Checker::Green(sdl_rect) => {
                self.renderings.green_rectangles[sdl_rect]
                    .borrow_mut()
                    .clear();
                self.score.green -= 1;
            }
            Checker::Red(sdl_rect) => {
                self.renderings.red_rectangles[sdl_rect]
                    .borrow_mut()
                    .clear();
                self.score.red -= 1;
            }
            Checker::SuperGreen(sdl_rect) => {
                {
                    self.renderings.green_rectangles[sdl_rect]
                        .borrow_mut()
                        .clear();
                }
                self.renderings.yellow_rectangles[yellow_for_green(sdl_rect)]
                    .borrow_mut()
                    .clear();
                self.score.green -= 1;
            }
            Checker::SuperRed(sdl_rect) => {
                {
                    self.renderings.red_rectangles[sdl_rect]
                        .borrow_mut()
                        .clear();
                }
                self.renderings.yellow_rectangles[yellow_for_red(sdl_rect)]
                    .borrow_mut()
                    .clear();
                self.score.red -= 1;
            }
            _ => {}
        };
        self.cell_mapping[victim] = Checker::None;
        self.move_to_empty(source, end);
    }

    fn try_to_overtake(&mut self, source: usize, victim: usize, end: usize) {
        let is_enemy = self.cell_mapping[source].is_different_color(&self.cell_mapping[victim]);
        let is_next_empty = self.cell_mapping[end].is_none();

        if is_enemy && is_next_empty {
            self.overtake(source, victim, end);
        }
    }

    fn check_next(&mut self, source_pos: usize, target_pos: usize, is_up: bool, is_left: bool) {
        let next_diagonal = if is_up {
            if is_left {
                row_up(target_pos, 1) - 1
            } else {
                row_up(target_pos, 1) + 1
            }
        } else {
            if is_left {
                row_down(target_pos, 1) - 1
            } else {
                row_down(target_pos, 1) + 1
            }
        };
        if !(on_right_edge(target_pos) || on_left_edge(target_pos)) && is_in_bounds(next_diagonal) {
            self.try_to_overtake(source_pos, target_pos, next_diagonal as usize);
        }
    }

    fn scan_neighbourhood(&mut self, source_pos: usize, target_pos: usize) {
        if !on_right_edge(source_pos) {
            // east-west boundary check
            self.iterator_diagonal(source_pos, target_pos, 1, false, false);
            self.iterator_diagonal(source_pos, target_pos, 1, true, false);
        }

        if !on_left_edge(source_pos) {
            // east-west boundary check
            self.iterator_diagonal(source_pos, target_pos, 1, false, true);
            self.iterator_diagonal(source_pos, target_pos, 1, true, true);
        }
    }

    fn iterator_diagonal(
        &mut self,
        source_pos: usize,
        target_pos: usize,
        j: i32,
        is_up: bool,
        is_left: bool,
    ) -> bool {
        let next_diagonal = if is_up {
            if is_left {
                row_up(source_pos, j) - j
            } else {
                row_up(source_pos, j) + j
            }
        } else {
            if is_left {
                row_down(source_pos, j) - j
            } else {
                row_down(source_pos, j) + j
            }
        };

        let is_target = next_diagonal as usize == target_pos;

        if is_in_bounds(next_diagonal) {
            if self.cell_mapping[target_pos].is_none() && is_target {
                self.move_to_empty(source_pos, target_pos);
                self.switch_turn();
                return false;
            } else if self.cell_mapping[next_diagonal as usize].is_some() {
                if is_target {
                    self.check_next(source_pos, target_pos, is_up, is_left);
                }
                return false;
            }
        } else {
            return false;
        }
        true
    }

    fn scan_diagonals(&mut self, source_pos: usize, target_pos: usize) {
        if !on_right_edge(source_pos) {
            for i in 1..=BOARD_LENGTH {
                if !self.iterator_diagonal(source_pos, target_pos, i as i32, false, false) {
                    break;
                }
            }
            for i in 1..=BOARD_LENGTH {
                if !self.iterator_diagonal(source_pos, target_pos, i as i32, true, false) {
                    break;
                }
            }
        }

        if !on_left_edge(source_pos) {
            for i in 1..=BOARD_LENGTH {
                if !self.iterator_diagonal(source_pos, target_pos, i as i32, false, true) {
                    break;
                }
            }

            for i in 1..=BOARD_LENGTH {
                if !self.iterator_diagonal(source_pos, target_pos, i as i32, true, true) {
                    break;
                }
            }
        }
    }
}

impl GameStateTrait for BoardState<'_> {
    fn update(&mut self, event: &sdl2::EventSubsystem) -> Result<RuntimeSignal, String> {
        if self.score.red <= 0 || self.score.green <= 0 {
            if self.score.red <= 0 {
                event.push_custom_event(WinColorEvent::new_green()).unwrap();
            } else {
                event.push_custom_event(WinColorEvent::new_red()).unwrap();
            }

            Ok(RuntimeSignal::GotoState(1))
        } else {
            if self.source_index.is_some() && self.target_index.is_some() {
                let (source_checker, source_i) = self.get_source_checker_ref().unwrap();
                let target_i = self.target_index.unwrap();

                if source_checker.is_super() {
                    self.scan_diagonals(source_i, target_i);
                } else {
                    self.scan_neighbourhood(source_i, target_i);
                }

                self.source_index = None;
                self.target_index = None;
            }
            Ok(RuntimeSignal::Continue)
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rects(&self.renderings.board_tiles)?;

        canvas.set_draw_color(Color::RGB(0x1f, 0x1f, 0x1f));
        canvas.fill_rects(&self.renderings.black_tiles)?;

        for i in 0..BOARD_SIZE {
            if let Some(twi) = self.texture_manager.get_texture(i) {
                canvas.copy(
                    twi.get_texture_ref(),
                    None,
                    self.renderings.debug_tile_text[i],
                )?;
            }
        }

        if let Some(i) = self.source_index {
            canvas.set_draw_color(Color::RGB(0x0, 0x0f, 0xfa));
            canvas.fill_rect(self.renderings.board_tiles[i])?;
        }

        canvas.set_draw_color(Color::RGB(0x0, 0xff, 0x0));
        canvas.fill_rects(&self.renderings.green_rectangles)?;

        canvas.set_draw_color(Color::RGB(0xff, 0x0, 0x0));
        canvas.fill_rects(&self.renderings.red_rectangles)?;

        canvas.set_draw_color(Color::RGB(0xef, 0xef, 0x00));
        canvas.fill_rects(&self.renderings.yellow_rectangles)?;

        if self.playing_color.is_green() {
            canvas.set_draw_color(Color::RGB(0x0, 0xff, 0x0));
        } else if self.playing_color.is_red() {
            canvas.set_draw_color(Color::RGB(0xff, 0x0, 0x0));
        } else {
            canvas.set_draw_color(Color::RGB(0xea, 0xea, 0xea));
        }

        canvas.fill_rect(self.renderings.indicator)?;

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<RuntimeSignal, String> {
        match event {
            Event::Quit { .. } => return Ok(RuntimeSignal::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Ok(RuntimeSignal::GotoState(2)),
            Event::MouseButtonDown {
                x,
                y,
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.mouse_point.x = *x;
                self.mouse_point.y = *y;
                match self.source_index {
                    None => self.source_index = self.find_source_checker_rect(),
                    Some(_) => self.target_index = self.find_target_rect(),
                };
                Ok(RuntimeSignal::Continue)
            }
            _ => Ok(RuntimeSignal::Continue)
        }
    }

    fn setup(&mut self, ass: &Assets) -> Result<(), String> {
        let mut tile_index = 0;

        {
            // indicating which turn it current is
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

                self.cell_mapping[flat_index] = Checker::Green(self.green_length);
                self.green_length += 1;
            } else if y > (BOARD_LENGTH / 2) {
                // red stuff
                let checker_rect = &mut self.renderings.red_rectangles[self.red_length];
                checker_rect.set_x((container.x() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_y((container.y() + CHECKER_PADDING as i32) as i32);
                checker_rect.set_width((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);
                checker_rect.set_height((CONTAINER_WIDTH - CHECKER_PADDING * 2) as u32);

                self.cell_mapping[flat_index] = Checker::Red(self.red_length);
                self.red_length += 1;
            }

            if let Some(font_with_info) = ass.font_collection.b612_regular.get(&12) {
                let font = font_with_info.font_ref();
                self.texture_manager.insert_surface_as_texture(
                    flat_index,
                    font.render(format!("{}", flat_index).as_ref())
                        .blended(Color::RGB(0x0, 0x0, 0x0))
                        .map_err(|err| err.to_string())?,
                )?;
            }

            if let Some(text) = self.texture_manager.get_texture(flat_index) {
                let TextureQuery { width, height, .. } = text.get_texture_info_ref();
                if let Some(r) = self.renderings.debug_tile_text.get_mut(flat_index) {
                    r.set_width(*width);
                    r.set_height(*height);
                    r.set_x(container.x() + 5);
                    r.set_y(container.y() + 5);
                }
            }
        }

        self.score.red = self.red_length;
        self.score.green = self.green_length;

        self.is_set_up = true;
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        self.is_set_up
    }
}
