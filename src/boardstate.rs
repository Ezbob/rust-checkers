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
    board: [rect::Rect; BOARD_SIZE]
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            is_loaded: false,
            board: [rect::Rect::new(0,0,100,100); BOARD_SIZE]
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
        canvas.fill_rect(canvas.viewport());
        canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
        canvas.draw_rects(&self.board);

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
        for y in 0..BOARD_LENGTH {
            for x in 0..BOARD_LENGTH {
                let flat_index = y * BOARD_LENGTH + x;
                let container = self.board.get_mut(flat_index).unwrap();

                container.set_x((100 * (flat_index % BOARD_LENGTH) + 20) as i32);
                container.set_y((100 * (flat_index / BOARD_LENGTH) + 20) as i32);

            }
        }

        self.is_loaded = true;
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

