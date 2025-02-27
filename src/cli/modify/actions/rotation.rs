use gnome_randr::display_config::{
    physical_monitor::PhysicalMonitor, ApplyConfig, monitor_models::transform::Orientation,
};

use super::{Action};

pub struct OrientationAction {
    pub orientation: Orientation
}

impl Action<'_> for OrientationAction {
    fn apply(&self, config: &mut ApplyConfig, _: &PhysicalMonitor) {
        config.transform.orientation = self.orientation;
    }
}

impl std::fmt::Display for OrientationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting rotation to {}", self.orientation)
    }
}
