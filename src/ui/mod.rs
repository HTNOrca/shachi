use bevy::prelude::*;
use bevy_egui::{
    egui::{containers::panel::Side, SidePanel, Slider, Window},
    EguiContext, EguiPlugin,
};
use bevy_mod_picking::events::PickingEvent;

use crate::{
    ai::hunger::Hunger,
    camera::CameraFollow,
    orca::Orca,
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
}

impl Default for SimFormState {
    fn default() -> Self {
        Self {
            pod_count: 10,
            pod_size_min: 1,
            pod_size_max: 6,
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
) {
    let panel = SidePanel::new(Side::Right, "root").resizable(true);

    if ui_state.show_panel {
        panel.show(ctx.ctx_mut(), |ui| {
            ui.heading("Simulation");
            ui.separator();
            ui.label(format!("simulated orcas: {}", sim.orca_count));
            ui.label(format!("time: {}", sim.time));
            ui.add(Slider::new(&mut sim_form_state.pod_count, 0..=50).text("Pods"));
            if ui.button("Restart Simulation").clicked() {
                run_sim_writer.send(RunSimEvent {
                    pod_count: sim_form_state.pod_count,
                    pod_size: 1..6,
                });
            }

            if let Some(selected) = selected {
                if let Ok((orca, hunger)) = query.get(selected.0) {
                    ui.heading("Inspector");
                    ui.separator();
                    ui.label(format!("gender: {}", orca.gender.to_string()));
                    ui.label(format!("age: {}", orca.age));
                    ui.label(format!("mass: {}", orca.mass));
                    ui.label(format!("type: {}", orca.orca_type.to_string()));
                    ui.label(format!("hunger: {}", hunger.0));
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
