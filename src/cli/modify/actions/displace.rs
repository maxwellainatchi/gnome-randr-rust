use gnome_randr::display_config::{
    physical_monitor::PhysicalMonitor, ApplyConfig, monitor_models::transform::Displacement,
};

use super::{Action};

pub struct DisplacementAction {
    pub displacement: Displacement
}

impl Action<'_> for DisplacementAction {
    fn apply(&self, config: &mut ApplyConfig, _: &PhysicalMonitor) {
        config.transform.displacement = self.displacement;
    }
}

impl std::fmt::Display for DisplacementAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting displacement to {}", self.displacement)
    }
}
