use gnome_randr::display_config::{
    logical_monitor::Transform, physical_monitor::PhysicalMonitor, ApplyConfig,
};

use super::{super::Rotation, Action};

pub struct RotationAction {
    pub rotation: Rotation,
}

impl Action<'_> for RotationAction {
    fn apply(&self, config: &mut ApplyConfig, _: &PhysicalMonitor) {
        config.transform = match self.rotation {
            Rotation::Normal => Transform::NORMAL,
            Rotation::Left => Transform::R270,
            Rotation::Right => Transform::R90,
            Rotation::Inverted => Transform::R180,
        }
        .bits();
    }
}

impl std::fmt::Display for RotationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting rotation to {}", self.rotation)
    }
}
