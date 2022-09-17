use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_bobs::physics_2d::*;

use crate::{orca::{Orca, Gender, Type, OrcaPlugin, Pod, PodPool}, ai::{hunger::Hunger, movement::{Sight, Movement}, AIPlugin}, fish::FishPlugin, camera::CameraPlugin, ui::UIPlugin};

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

    app.add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(OrcaPlugin)
        .add_plugin(FishPlugin)
        .add_plugin(CameraPlugin);

    app.run();

}
