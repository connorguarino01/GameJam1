#![warn(clippy::all)]
use amethyst::prelude::*;
use amethyst::renderer::{ RenderingBundle, RenderToWindow, RenderFlat2D, rendy::vulkan::Backend };
use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};

mod systems;
mod pong;
use crate::pong::Pong;
mod food;

fn main() -> amethyst::Result<()> {
    let app_root = std::env::current_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    
    amethyst::start_logger(Default::default());

    
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let mut world = World::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<Backend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}