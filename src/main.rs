#![warn(clippy::all)]
use amethyst::{
    { LoggerConfig, LogLevelFilter },
    prelude::*,
    renderer::{ RenderingBundle, RenderToWindow, RenderFlat2D, rendy::vulkan::Backend },
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    audio::{ AudioBundle, DjSystemDesc }
};

mod systems;
mod pong;
use crate::pong::Pong;
mod audio;
use audio::Music;
// mod food;


fn main() -> amethyst::Result<()> {
    let app_root = std::env::current_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    
    amethyst::start_logger(Default::default());
    
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<Backend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"]
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();
    Ok(())
}