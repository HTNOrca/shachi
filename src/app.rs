use bevy::{prelude::*, render::texture::ImageSettings};

use crate::orca::Orca;

pub fn app() {
    let mut window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "sakamata".into(),
        ..default()
    };

    window_descriptor.width = 800.;
    window_descriptor.height = 600.;

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(window_descriptor);

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(debug);

    app.run();

}

fn debug(mut cmd: Commands) {

    let id = cmd.spawn();

    // spawn.insert(Orca);

}
