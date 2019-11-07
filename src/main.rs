#![warn(clippy::all)]
use amethyst::prelude::*;
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use game_jam1_lib::pawn::*;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Starting game!");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                 Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None,
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("Computing some more whoop-ass...");
        Trans::Quit
    }
}

fn main() -> amethyst::Result<()> {
    let assets_dir = "assets/";
    let mut game = Application::new(assets_dir, GameState, GameDataBuilder::default())?;
    // amethyst::start_logger();
    game.run();
    Ok(())
}