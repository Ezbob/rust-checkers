extern crate sdl2;

use crate::game_machine::runtime_signal::RuntimeSignal;
use crate::game_machine::state::GameStateTrait;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};

use crate::asset_loader::{Assets, TextureManager};
use crate::game_events::WinColorEvent;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const GREEN_TEXT_WIN: usize = 0;
const RED_TEXT_WIN: usize = 1;

pub struct WinState<'ttf> {
    texture_manager: TextureManager<'ttf>,
    is_set_up: bool,
    is_green_win: bool,
}

impl<'ttf> WinState<'ttf> {
    pub fn new(text_creator: &'ttf TextureCreator<WindowContext>) -> WinState<'ttf> {
        WinState {
            texture_manager: TextureManager::new(text_creator),
            is_set_up: false,
            is_green_win: false,
        }
    }
}

impl<'a> GameStateTrait for WinState<'a> {
    fn update(&mut self, _event: &sdl2::EventSubsystem) -> RuntimeSignal {
        RuntimeSignal::Continue
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        if self.is_green_win {
            if let Some(txtr) = self.texture_manager.get_texture(GREEN_TEXT_WIN) {
                let center = canvas.viewport().center();
                let TextureQuery { width, height, .. } = txtr.get_texture_info_ref();

                let half_x = center.x() - (*width as i32 / 2);
                let half_y = center.y() - (*height as i32 / 2);

                let dst = Some(Rect::new(half_x, half_y, *width, *height));

                canvas.copy(txtr.get_texture_ref(), None, dst)?;
            }
        } else {
            if let Some(txtr) = self.texture_manager.get_texture(RED_TEXT_WIN) {
                let center = canvas.viewport().center();
                let TextureQuery { width, height, .. } = txtr.get_texture_info_ref();

                let half_x = center.x() - (*width as i32 / 2);
                let half_y = center.y() - (*height as i32 / 2);

                let dst = Some(Rect::new(half_x, half_y, *width, *height));

                canvas.copy(txtr.get_texture_ref(), None, dst)?;
            }
        }

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> RuntimeSignal {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return RuntimeSignal::Quit,
            _ => {}
        };

        if event.is_user_event() {
            match &event.as_user_event_type::<WinColorEvent>() {
                Some(wce) => {
                    self.is_green_win = wce.is_green();
                }
                _ => {}
            }
        }

        RuntimeSignal::Continue
    }

    fn setup(&mut self, ass: &Assets) -> Result<(), String> {
        let font = ass.font_collection.share_tech_mono_regular[&52].font_ref();

        self.texture_manager.insert_surface_as_texture(
            RED_TEXT_WIN,
            font.render("Red wins!")
                .blended(Color::RGB(0xef, 0x0, 0x0))
                .map_err(|e| e.to_string())?,
        )?;

        self.texture_manager.insert_surface_as_texture(
            GREEN_TEXT_WIN,
            font.render("Green wins!")
                .blended(Color::RGB(0x0, 0xef, 0x0))
                .map_err(|e| e.to_string())?,
        )?;

        self.is_set_up = true;
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        self.is_set_up
    }
}
