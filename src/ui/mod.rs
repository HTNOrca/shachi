use bevy::{prelude::*, render::render_phase::Draw};
use bevy_egui::{
    egui::{containers::panel::Side, ScrollArea, SidePanel, Slider, Window},
    EguiContext, EguiPlugin,
};
use bevy_mod_picking::events::PickingEvent;
use bevy_prototype_lyon::prelude::*;
use pino_utils::{ok_or_return, some_or_return};

use crate::{
    ai::{
        hunger::Hunger,
        movement::{BoidParams, OrcaNeighbouring},
    },
    camera::CameraFollow,
    orca::{Orca, PodPool},
    sim::{RunSimEvent, Simulation},
};

#[derive(Component)]
pub struct NeighbourLine;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIState::default())
            .insert_resource(SimFormState::default())
            .add_plugin(EguiPlugin)
            .add_system(render_ui)
            .add_system(ui_controller)
            .add_system(select_controller)
            .add_system(deselect_controller)
            .add_system(neighbour_debug)
            .add_system(line_cleaner);
    }
}

pub struct SelectedOrca(Entity);

pub struct UIState {
    show_panel: bool,
}

pub struct SimFormState {
    enable_orca: bool,
    pod_count: usize,
    pod_size_min: usize,
    pod_size_max: usize,

    enable_fish: bool,
    fish_count: usize,

    orca_params: BoidParams,
    fish_params: BoidParams,
}

impl Default for SimFormState {
    fn default() -> Self {
        Self {
            enable_orca: true,
            pod_count: 4,
            pod_size_min: 15,
            pod_size_max: 30,

            enable_fish: true,
            fish_count: 100,

            orca_params: BoidParams {
                coherence: 0.5,
                seperation: 2.0,
                ..default()
            },
            fish_params: BoidParams {
                randomness: 4.0,
                view_range: 50.,
                view_angle: 60.,
                ..default()
            },
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self { show_panel: true }
    }
}

fn render_ui(
    mut ctx: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    mut sim_form_state: ResMut<SimFormState>,
    selected: Option<Res<SelectedOrca>>,
    query: Query<(&Orca, &Hunger)>,
    mut run_sim_writer: EventWriter<RunSimEvent>,
    sim: Res<Simulation>,
    pod_pool: Res<PodPool>,
) {
    if ui_state.show_panel {
        SidePanel::new(Side::Right, "root")
            .resizable(true)
            .show(ctx.ctx_mut(), |ui| {
                let scroll_area = ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Simulation");
                    ui.separator();
                    ui.label(format!("simulated orcas: {}", sim.orca_count));
                    ui.label(format!("time: {}s", (sim.time * 100.).round() / 100.));

                    ui.separator();
                    ui.label("Orca Params");
                    ui.checkbox(&mut sim_form_state.enable_orca, "Enable Orcas");
                    ui.add(Slider::new(&mut sim_form_state.pod_count, 0..=50).text("Pods"));
                    ui.add(
                        Slider::new(&mut sim_form_state.pod_size_min, 0..=50).text("Pod Size Min"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.pod_size_max, 0..=50).text("Pod Size Max"),
                    );
                    if sim_form_state.pod_size_min > sim_form_state.pod_size_max {
                        sim_form_state.pod_size_max = sim_form_state.pod_size_min;
                    }
                    ui.add_space(10.);
                    ui.add(
                        Slider::new(&mut sim_form_state.orca_params.coherence, 0.0f32..=10.)
                            .text("Coherence"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.orca_params.alignment, 0.0f32..=10.)
                            .text("Alignment"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.orca_params.seperation, 0.0f32..=10.)
                            .text("Seperation"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.orca_params.randomness, 0.0f32..=10.)
                            .text("Randomness"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.orca_params.view_range, 0.0f32..=500.)
                            .text("View Range"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.orca_params.view_angle, 0.0f32..=180.0)
                            .text("View Angle"),
                    );

                    ui.separator();
                    ui.label("Fish Params");
                    ui.checkbox(&mut sim_form_state.enable_fish, "Enable Fish");
                    ui.add(Slider::new(&mut sim_form_state.fish_count, 0..=500).text("Fish Count"));
                    ui.add_space(10.);
                    ui.add(
                        Slider::new(&mut sim_form_state.fish_params.coherence, 0.0f32..=10.)
                            .text("Coherence"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.fish_params.alignment, 0.0f32..=10.)
                            .text("Alignment"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.fish_params.seperation, 0.0f32..=10.)
                            .text("Seperation"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.fish_params.randomness, 0.0f32..=10.)
                            .text("Randomness"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.fish_params.view_range, 0.0f32..=500.)
                            .text("View Range"),
                    );
                    ui.add(
                        Slider::new(&mut sim_form_state.fish_params.view_angle, 0.0f32..=180.0)
                            .text("View Angle"),
                    );

                    ui.separator();
                    if ui.button("Restart Simulation").clicked() {
                        run_sim_writer.send(RunSimEvent {
                            enable_orca: sim_form_state.enable_orca,
                            pod_count: sim_form_state.pod_count,
                            pod_size_min: sim_form_state.pod_size_min,
                            pod_size_max: sim_form_state.pod_size_max,
                            pod_size: 1..6,

                            enable_fish: sim_form_state.enable_fish,
                            fish_count: sim_form_state.fish_count,

                            orca_params: sim_form_state.orca_params,
                            fish_params: sim_form_state.fish_params,
                        });
                    }

                    if let Some(selected) = selected {
                        if let Ok((orca, hunger)) = query.get(selected.0) {
                            ui.heading("Inspector");
                            ui.separator();
                            if let Some(pod_id) = orca.pod_id {
                                if let Some(pod) = pod_pool.get(&pod_id) {
                                    ui.label(format!("pod: {}", pod.name));
                                }
                            }
                            ui.label(format!("name: {}", orca.name));
                            ui.label(format!("gender: {}", orca.gender.to_string()));
                            ui.label(format!("age: {} years", orca.age));
                            ui.label(format!("mass: {} kg", orca.mass));
                            ui.label(format!("type: {}", orca.orca_type.to_string()));
                            ui.label(format!("hunger: {}", (hunger.0 * 100.).round() / 100.));
                        }
                    }
                });
            });
    }
}

fn ui_controller(keys: Res<Input<KeyCode>>, mut ui_state: ResMut<UIState>) {
    if keys.just_pressed(KeyCode::Tab) {
        ui_state.show_panel = !ui_state.show_panel;
    }
}

fn select_controller(mut cmd: Commands, mut events: EventReader<PickingEvent>) {
    for evt in events.iter() {
        match evt {
            PickingEvent::Clicked(entity) => {
                cmd.insert_resource(SelectedOrca(*entity));
                cmd.insert_resource(CameraFollow(*entity));
            },
            _ => {},
        }
    }
}

fn deselect_controller(mut cmd: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        cmd.remove_resource::<SelectedOrca>();
        cmd.remove_resource::<CameraFollow>();
    }
}

fn neighbour_debug(
    mut cmd: Commands,
    selected: Option<Res<SelectedOrca>>,
    query: Query<(&OrcaNeighbouring, &Transform)>,
) {
    let selected = some_or_return!(selected);
    let (neighbouring, self_trans) = ok_or_return!(query.get(selected.0));

    for (_, trans) in query.iter_many(neighbouring.pod_members.clone()) {
        cmd.spawn_bundle(GeometryBuilder::build_as(
            &shapes::Line(
                self_trans.translation.truncate(),
                trans.translation.truncate(),
            ),
            DrawMode::Stroke(StrokeMode::new(Color::BLACK, 0.2)),
            Transform::default(),
        ))
        .insert(NeighbourLine);
    }
}
fn line_cleaner(mut cmd: Commands, query: Query<Entity, With<NeighbourLine>>) {
    for e in &query {
        cmd.entity(e).despawn_recursive();
    }
}
