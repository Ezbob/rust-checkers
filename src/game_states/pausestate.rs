use crate::asset_loader::{Assets, TextureManager};
use crate::game_machine::runtime_signal::RuntimeSignal;
use crate::game_machine::state::GameStateTrait;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};
use sdl2::EventSubsystem;

const PAUSE_TEXT: usize = 0;

pub struct PauseState<'ttf> {
    is_setup: bool,
    texture_manager: TextureManager<'ttf>,
}

impl<'ttf> PauseState<'ttf> {
    pub fn new(texture_creator: &'ttf TextureCreator<WindowContext>) -> PauseState<'ttf> {
        PauseState {
            is_setup: false,
            texture_manager: TextureManager::new(texture_creator),
        }
    }
}

impl<'ttf> GameStateTrait for PauseState<'ttf> {
    fn update(&mut self, _event: &EventSubsystem) -> Result<RuntimeSignal, String> {
        Ok(RuntimeSignal::Continue)
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.clear();

        if let Some(text_info) = self.texture_manager.get_texture(PAUSE_TEXT) {
            let TextureQuery { width, height, .. } = text_info.get_texture_info_ref();

            let center = canvas.viewport().center();

            let x = center.x() - (width / 2) as i32;
            let y = center.y() - (height / 2) as i32;

            canvas.copy(
                text_info.get_texture_ref(),
                None,
                Some(Rect::new(x, y, *width, *height)),
            )?;
        }

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<RuntimeSignal, String> {
        match event {
            Event::Quit { .. } => Ok(RuntimeSignal::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Ok(RuntimeSignal::GotoState(0)),
            _ => Ok(RuntimeSignal::Continue)
        }
    }

    fn setup(&mut self, ass: &Assets<'_>) -> Result<(), String> {
        let font = ass.font_collection.b612_regular[&30].font_ref();

        self.texture_manager.insert_surface_as_texture(
            PAUSE_TEXT,
            font.render("Game paused. Press Escape to resume")
                .blended(Color::RGB(0, 0, 0xaf))
                .map_err(|e| e.to_string())?,
        )?;

        self.is_setup = true;
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        self.is_setup
    }
}
