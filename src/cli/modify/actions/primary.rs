use gnome_randr::display_config::{models::PhysicalMonitor, ApplyConfig};

use super::Action;

pub struct PrimaryAction {}

impl<'a> Action<'a> for PrimaryAction {
    fn apply(&self, config: &mut ApplyConfig<'a>, _: &PhysicalMonitor) {
        config.primary = true;
    }
}

impl std::fmt::Display for PrimaryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting monitor as primary")
    }
}
