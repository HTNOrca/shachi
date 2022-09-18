use bevy::prelude::*;
use bevy_egui::{
    egui::{containers::panel::Side, SidePanel, Slider, Window},
    EguiContext, EguiPlugin,
};
use bevy_mod_picking::events::PickingEvent;

use crate::{
    ai::hunger::Hunger,
    camera::CameraFollow,
    orca::{Orca, PodPool},
    sim::{RunSimEvent, Simulation},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIState::default())
            .insert_resource(SimFormState::default())
            .add_plugin(EguiPlugin)
            .add_system(render_ui)
            .add_system(ui_controller)
            .add_system(select_controller)
            .add_system(deselect_controller);
    }
}

pub struct SelectedOrca(Entity);

pub struct UIState {
    show_panel: bool,
}

pub struct SimFormState {
    pod_count: usize,
    pod_size_min: usize,
    pod_size_max: usize,

    coherence: f32,
    alignment: f32,
    seperation: f32,
    randomness: f32,

    view_range: f32,
}

impl Default for SimFormState {
    fn default() -> Self {
        Self {
            pod_count: 10,
            pod_size_min: 15,
            pod_size_max: 30,

            coherence: 1.,
            alignment: 1.,
            seperation: 1.,
            randomness: 1.,

            view_range: 50.,
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
    let panel = SidePanel::new(Side::Right, "root").resizable(true);

    if ui_state.show_panel {
        panel.show(ctx.ctx_mut(), |ui| {
            ui.heading("Simulation");
            ui.separator();
            ui.label(format!("simulated orcas: {}", sim.orca_count));
            ui.label(format!("time: {}s", (sim.time * 100.).round() / 100.));
            ui.add(Slider::new(&mut sim_form_state.pod_count, 0..=50).text("Pods"));
            ui.add(Slider::new(&mut sim_form_state.pod_size_min, 0..=50).text("Pod Size Min"));
            ui.add(Slider::new(&mut sim_form_state.pod_size_max, 0..=50).text("Pod Size Max"));
            if sim_form_state.pod_size_min > sim_form_state.pod_size_max {
                sim_form_state.pod_size_max = sim_form_state.pod_size_min;
            }
            ui.separator();
            ui.add(Slider::new(&mut sim_form_state.coherence, 0.0f32..=10.).text("Coherence"));
            ui.add(Slider::new(&mut sim_form_state.alignment, 0.0f32..=10.).text("Alignment"));
            ui.add(Slider::new(&mut sim_form_state.seperation, 0.0f32..=10.).text("Seperation"));
            ui.add(Slider::new(&mut sim_form_state.randomness, 0.0f32..=10.).text("Randomness"));
            ui.add(Slider::new(&mut sim_form_state.view_range, 0.0f32..=500.).text("View Range"));
            if ui.button("Restart Simulation").clicked() {
                run_sim_writer.send(RunSimEvent {
                    pod_count: sim_form_state.pod_count,
                    pod_size_min: sim_form_state.pod_size_min,
                    pod_size_max: sim_form_state.pod_size_max,
                    pod_size: 1..6,
                    coherence: sim_form_state.coherence,
                    alignment: sim_form_state.alignment,
                    seperation: sim_form_state.seperation,
                    randomness: sim_form_state.randomness,
                    view_range: sim_form_state.view_range,
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
