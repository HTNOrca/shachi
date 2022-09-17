use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_pancam::{PanCam, PanCamPlugin};
use pino_utils::ok_or_return;

pub struct CameraFollow(pub Entity);

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanCamPlugin::default())
            .add_startup_system(spawn_camera)
            .add_system(camera_follow);
    }
}

fn camera_follow(
    follow: Option<Res<CameraFollow>>,
    entity_query: Query<&Transform, Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Some(follow) = follow {
        let follow_trans = ok_or_return!(entity_query.get(follow.0));
        let mut camera_trans = ok_or_return!(camera_query.get_single_mut());

        camera_trans.translation = follow_trans.translation;
    }
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        ..default()
    })
    .insert(PanCam::default())
    .insert_bundle(PickingCameraBundle::default())
    .insert(MainCamera);
}
