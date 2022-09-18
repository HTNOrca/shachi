use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_bobs::physics_2d::*;
use bevy_mod_picking::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    ai::{
        hunger::Hunger,
        movement::{Movement, Sight},
        AIPlugin,
    },
    camera::CameraPlugin,
    fish::FishPlugin,
    orca::{Gender, Orca, OrcaPlugin, Pod, PodPool, Type},
    sim::SimPlugin,
    ui::UIPlugin,
};

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
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugins(DefaultPickingPlugins);
    // .add_plugin(DebugEventsPickingPlugin);

    app.add_plugin(UIPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(OrcaPlugin)
        .add_plugin(FishPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SimPlugin);

    app.run();
}
