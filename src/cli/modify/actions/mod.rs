mod mode;
mod primary;
mod rotation;
mod scale;

use gnome_randr::display_config::{models::PhysicalMonitor, ApplyConfig};

pub use mode::ModeAction;
pub use primary::PrimaryAction;
pub use rotation::RotationAction;
pub use scale::ScaleAction;

pub trait Action<'a>: std::fmt::Display {
    fn apply(&self, config: &mut ApplyConfig<'a>, physical_monitor: &PhysicalMonitor);
}
