mod mode;
mod primary;
mod rotation;

use gnome_randr::display_config::{physical_monitor::PhysicalMonitor, ApplyConfig};

pub use mode::ModeAction;
pub use primary::PrimaryAction;
pub use rotation::RotationAction;

pub trait Action<'a>: std::fmt::Display {
    fn apply(&self, config: &mut ApplyConfig<'a>, physical_monitor: &PhysicalMonitor);
}
