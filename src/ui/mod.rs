use bevy::prelude::*;


use self::{day_end_menu::DayEndPlugin, tooltips::TooltipPlugin};

mod day_end_menu;
pub mod tooltips;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DayEndPlugin, TooltipPlugin));
    }
}