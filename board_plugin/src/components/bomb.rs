use bevy::prelude::Component;

/// Bomb component
#[cfg_attr(debug_assertions, derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;
