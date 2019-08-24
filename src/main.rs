extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::Sdl;

mod gamestate;

fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;
    let video_sys = sdl_cxt.video()?;

    let _win = video_sys.window("Rust sdl2 demo", 800, 600)
        .position_centered()
        .build().unwrap();

    //let mut events = sdl_cxt.event_pump()?;

    let _gamemachine = gamestate::GameMachine::new(&sdl_cxt, vec![]);
/*
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} |
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown {x, y, mouse_btn, ..} => {
                    match mouse_btn {
                        MouseButton::Left => {
                            println!("Clicked left: ({}, {})", x, y);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

    }
*/

    Ok(())
}
