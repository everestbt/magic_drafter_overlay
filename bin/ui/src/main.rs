use bevy::prelude::*;
use bevy::window::{CursorOptions, WindowLevel, WindowResolution};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;

const LINE_HEIGHT: f32 = 21.;

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
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        children![
            vertically_scrolling_list(),
        ],
    ));
}

fn vertically_scrolling_list() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: px(200),
            ..default()
        },
        children![
            (
                // Title
                Text::new("Vertically Scrolling List"),
                Label,
            ),
            (
                // List
                Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    height: percent(50),
                    overflow: Overflow::clip(),
                    ..default()
                },
                Children::spawn(SpawnIter((0..15).map(move |i| {
                    (
                        Node {
                            min_height: px(LINE_HEIGHT),
                            max_height: px(LINE_HEIGHT),
                            ..default()
                        },
                        children![(
                            Text(format!("Item {i}")),
                            Label,
                        )],
                    )
                })))
            ),
        ],
    )
}