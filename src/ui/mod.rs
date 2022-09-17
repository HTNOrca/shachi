use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContext, egui::{Window, SidePanel, containers::panel::Side}};

use crate::orca::Orca;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIState::default())
            .add_plugin(EguiPlugin)
            .add_system(render_ui)
            .add_system(update_state);
    }
}

pub struct UIState {
    orca_count: u32,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            orca_count: 0,
        }
    }
}

fn render_ui(mut ctx: ResMut<EguiContext>, ui_state: Res<UIState>) {
    SidePanel::new(Side::Right, "root").resizable(true).show(ctx.ctx_mut(), |ui| {
        ui.heading("Analyzer");
        ui.separator();
        ui.label(format!("simulated orcas: {}", ui_state.orca_count));
    });
}

fn update_state(query: Query<&Orca>, mut ui_state: ResMut<UIState>) {
    ui_state.orca_count = query.iter().len() as u32;
}
