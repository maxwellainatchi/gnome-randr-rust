use gnome_randr::display_config::{physical_monitor::PhysicalMonitor, ApplyConfig};

use super::Action;

pub struct ModeAction<'a> {
    pub mode: &'a str,
}

impl<'a> Action<'a> for ModeAction<'a> {
    fn apply(&self, config: &mut ApplyConfig<'a>, physical_monitor: &PhysicalMonitor) {
        config
            .monitors
            .iter_mut()
            .find(|monitor| monitor.connector == physical_monitor.connector)
            .unwrap()
            .mode_id = self.mode;
    }
}

impl std::fmt::Display for ModeAction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting mode to {}", self.mode)
    }
}
