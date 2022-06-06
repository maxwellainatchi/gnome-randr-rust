mod mode;
mod primary;
mod rotation;
mod scale;
mod displace;

use gnome_randr::display_config::{physical_monitor::PhysicalMonitor, ApplyConfig};

pub use mode::ModeAction;
pub use primary::PrimaryAction;
pub use rotation::OrientationAction;
pub use scale::ScaleAction;
pub use displace::DisplacementAction;

pub trait Action<'a>: std::fmt::Display {
    fn apply(&self, config: &mut ApplyConfig<'a>, physical_monitor: &PhysicalMonitor);
}
