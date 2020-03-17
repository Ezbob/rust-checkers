extern crate sdl2;

use crate::gamemachine::runtime_signal::RuntimeSignal;
use crate::gamemachine::state::GameStateTrait;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

use crate::assets::GameAssets;
use crate::game_events::WinColorEvent;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

pub struct WinState<'a> {
    red_text: Option<Surface<'a>>,
    green_text: Option<Surface<'a>>,
    is_set_up: bool,
    is_green_win: bool,
}

impl<'a> WinState<'a> {
    pub fn new() -> WinState<'a> {
        WinState {
            red_text: None,
            green_text: None,
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
        canvas.set_draw_color(Color::RGB(0xff,0xff,0xff));
        canvas.clear();

        let texture_creator = canvas.texture_creator();

        if self.is_green_win {
            if let Some(surf) = &self.green_text {
                let text = texture_creator
                    .create_texture_from_surface(surf)
                    .map_err(|e| e.to_string())?;

                let center = canvas.viewport().center();
                let TextureQuery { width, height, .. } = text.query();

                let half_x = center.x() - (width as i32 / 2);
                let half_y = center.y() - (height as i32 / 2);

                let dst = Some(sdl2::rect::Rect::new(half_x, half_y, width, height));

                canvas.copy(&text, None, dst)?;
            }
        } else {
            if let Some(surf) = &self.red_text {
                let text = texture_creator
                    .create_texture_from_surface(surf)
                    .map_err(|e| e.to_string())?;

                let center = canvas.viewport().center();
                let TextureQuery { width, height, .. } = text.query();

                let half_x = center.x() - (width as i32 / 2);
                let half_y = center.y() - (height as i32 / 2);

                let dst = Some(sdl2::rect::Rect::new(half_x, half_y, width, height));

                canvas.copy(&text, None, dst)?;
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

    fn setup(&mut self, ass: &GameAssets) -> Result<(), String> {
        let redtext = ass
            .font_vt323_big
            .render("Red wins!")
            .solid(Color::RGB(0xef, 0x0, 0x0))
            .map_err(|e| e.to_string())?;
        let greentext = ass
            .font_vt323_big
            .render("Green wins!")
            .solid(Color::RGB(0x0, 0xef, 0x0))
            .map_err(|e| e.to_string())?;
        self.red_text = Some(redtext);
        self.green_text = Some(greentext);
        self.is_set_up = true;
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        self.is_set_up
    }
}
