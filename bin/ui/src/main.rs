use bevy::prelude::*;
use bevy::window::{CursorOptions, WindowLevel, WindowResolution};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;


fn main() -> Result<()> {
    color_eyre::install()?;
    let window = Window {
        title: "Magic Drafter Overlay".into(),
        transparent: true,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        #[cfg(target_os = "macos")]
        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
        #[cfg(target_os = "linux")]
        composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
        position: WindowPosition::new(IVec2::new(0, 200)), // Set to top left corner with a fixed size
        resolution: WindowResolution::new(500, 700),
        ..default()
    };
    let cursor = CursorOptions {
        hit_test: false,
        ..default()
    };
    App::new()
        // Make it render background as transparent
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(window),
                primary_cursor_options: Some(cursor),
                
                ..default()}
            )
        )
        .add_systems(Startup, setup)
        .run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);    
    commands.spawn((
        Text::new("From an &str into a Text with the default font!"),
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(15),
            ..default()
        },
    ));
}
