use bevy::prelude::*;

use crate::assets::Fonts;

use self::{day_end_menu::DayEndPlugin, tooltips::TooltipPlugin, player_ui::PlayerUiPlugin};

mod day_end_menu;
mod player_ui;
mod game_over;
pub mod tooltips;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum MenuState {
    #[default]
    None,
    Fading,
    Done,
}

pub const PARCHMENT: Color = Color::rgb(0.9216, 0.8353, 0.702);

#[derive(SystemSet, Hash, Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct NeedsTextSet;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()
            .add_plugins((DayEndPlugin, TooltipPlugin, PlayerUiPlugin));

        app.configure_sets(Update, NeedsTextSet.run_if(resource_exists::<Fonts>()));
    }
}
