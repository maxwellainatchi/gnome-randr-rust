use dbus::arg::{RefArg, Variant};
use gnome_randr::display_config::{physical_monitor::PhysicalMonitor, ApplyConfig};

use super::Action;

pub struct PropertyChangeAction<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> Action<'a> for PropertyChangeAction<'a> {
    fn apply(&self, config: &mut ApplyConfig, physical_monitor: &PhysicalMonitor) {
        let monitor = config
            .monitors
            .iter_mut()
            .find(|monitor| monitor.connector == physical_monitor.connector)
            .unwrap();

        monitor.properties.insert(
            self.name.to_string(),
            Variant {
                0: Box::new(self.value.to_string()) as Box<dyn RefArg>,
            },
        );
    }
}

impl std::fmt::Display for PropertyChangeAction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting property {} to {}", self.name, self.value)
    }
}
