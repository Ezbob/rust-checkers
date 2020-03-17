use crate::assets::GameAssets;
use crate::gamemachine::runtime_signal::RuntimeSignal;
use crate::gamemachine::state::GameStateTrait;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::EventSubsystem;

pub struct PauseState<'a> {
    is_setup: bool,
    pause_text: Option<Surface<'a>>,
}

impl<'a> PauseState<'a> {
    pub fn new() -> PauseState<'a> {
        PauseState {
            is_setup: false,
            pause_text: None,
        }
    }
}

impl<'a> GameStateTrait for PauseState<'a> {
    fn update(&mut self, _event: &EventSubsystem) -> RuntimeSignal {
        RuntimeSignal::Continue
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        if let Some(surf) = &self.pause_text {
            let text_creator = canvas.texture_creator();
            let text = text_creator
                .create_texture_from_surface(surf)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = text.query();

            let center = canvas.viewport().center();

            let x = center.x() - (width / 2) as i32;
            let y = center.y() - (height / 2) as i32;

            canvas.copy(&text, None, Some(Rect::new(x, y, width, height)))?;
        }

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> RuntimeSignal {
        match event {
            Event::Quit { .. } => RuntimeSignal::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => RuntimeSignal::GotoState(0),
            _ => RuntimeSignal::Continue,
        }
    }

    fn setup(&mut self, ass: &GameAssets<'_>) -> Result<(), String> {
        let surf = ass
            .font_vt323_big
            .render("Game paused. Press Escape to resume")
            .solid(Color::RGB(0, 0, 0xaf))
            .map_err(|e| e.to_string())?;
        self.pause_text = Some(surf);
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        self.is_setup
    }
}
