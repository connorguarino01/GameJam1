#![warn(clippy::all)]
use amethyst::prelude::*;
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use crate::pawn::*;

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

fn skill_level_test() {
    let cook_skill = CookSkillTypes::Meat(5);
    let server_skill = ServerSkillTypes::Balance(10);
    cook_skill.add_xp(10);
    server_skill.sub_xp(14);
    assert_eq!(cook_skill.get_level(), 15);
    assert_eq!(server_skill.get_level(), 6);
}
