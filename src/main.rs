
use std::collections::HashSet;

use bevy::{prelude::*, render::{camera::Viewport, view::RenderLayers}};

use render_test::{TestRenderComponent,TestRenderPlugin};

fn main() {
    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "render test".into(),
                        resolution: (800.0, 600.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
            }),
            TestRenderPlugin,
        ))

        .add_systems(Startup, ( setup_ui, ))
        .add_systems(Update, ( update_input, ))
        ;

    app.run();
}

pub fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        // RenderLayers::from_layers(&[0]),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.2, 0.8)),
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(100, 100), //not size, actually x2,y2
                ..Default::default()
            }),
            ..Default::default()
        },
    ));
    commands.spawn((
        Camera3d::default(),
        // RenderLayers::from_layers(&[0]),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.9, 0.1)),
            viewport: Some(Viewport {
                physical_position: UVec2::new(100, 0),
                physical_size: UVec2::new(500, 500), //not size, actually x2,y2
                ..Default::default()
            }),
            ..Default::default()
        },
    ));
    // commands.spawn((
    //     Camera3d {    ..Default::default() },
    //     // RenderLayers::from_layers(&[1]),
    //     Camera {
    //         clear_color: ClearColorConfig::Custom(Color::srgb(0.5, 1.0, 0.8)),
    //         order: 1,
    //         viewport: Some(Viewport {
    //             physical_position: UVec2::new(200, 0),
    //             physical_size: UVec2::new(256, 256), //not size, actually x2,y2
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     },
    // ));
    commands.spawn((
        TestRenderComponent{ col: Color::srgb(1.0,0.2,0.1), x: 0.0, y: 0.0, w: 50.0, h: 50.0, },
        // RenderLayers::layer(0),
    ));
}

fn update_input(
    mut key_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    mut last_pressed:Local<HashSet<KeyCode>>,
) {
    for ev in key_events.read() {
        //
        if ev.state==bevy::input::ButtonState::Pressed && !last_pressed.contains(&ev.key_code) {
            if ev.key_code==KeyCode::Escape || ev.key_code==KeyCode::F4 {
                exit.send(AppExit::Success);
            }
        }

        //
        if ev.state==bevy::input::ButtonState::Pressed {
            last_pressed.insert(ev.key_code);
        } else if ev.state==bevy::input::ButtonState::Released {
            last_pressed.remove(&ev.key_code);
        }
    }
}
