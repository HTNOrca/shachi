use bevy::prelude::*;
use bevy_egui::{
    egui::{containers::panel::Side, SidePanel, Window},
    EguiContext, EguiPlugin,
};

use crate::orca::Orca;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIState::default())
            .add_plugin(EguiPlugin)
            .add_system(render_ui)
            .add_system(update_state)
            .add_system(ui_controller);
    }
}

pub struct UIState {
    show_panel: bool,
    orca_count: u32,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            show_panel: true,
            orca_count: 0,
        }
    }
}

fn render_ui(mut ctx: ResMut<EguiContext>, ui_state: Res<UIState>) {
    let panel = SidePanel::new(Side::Right, "root").resizable(true);

    if ui_state.show_panel {
        panel.show(ctx.ctx_mut(), |ui| {
            ui.heading("Analyzer");
            ui.separator();
            ui.label(format!("simulated orcas: {}", ui_state.orca_count));
        });
    }
}

fn update_state(query: Query<&Orca>, mut ui_state: ResMut<UIState>) {
    ui_state.orca_count = query.iter().len() as u32;
}

fn ui_controller(keys: Res<Input<KeyCode>>, mut ui_state: ResMut<UIState>) {
    if keys.just_pressed(KeyCode::Escape) {
        ui_state.show_panel = !ui_state.show_panel;
    }
}
