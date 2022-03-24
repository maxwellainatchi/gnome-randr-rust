pub mod models;
pub mod proxied_methods;
pub mod resources;

mod raw;

use models::LogicalMonitor;
use models::PhysicalMonitor;
pub use proxied_methods::{ApplyConfig, ApplyMonitor};

// Config properties/comments are sourced from https://github.com/jadahl/gnome-monitor-config/blob/master/src/org.gnome.Mutter.DisplayConfig.xml

/// Current layout mode represents the way logical monitors are layed out on the screen.
#[derive(Debug)]
pub enum LayoutMode {
    /// The dimension of a logical monitor is the dimension of the monitor mode, divided by the logical monitor scale.
    Logical,
    /// Each logical monitor has the same dimensions as the monitor modes of the associated monitors assigned to it, no matter what scale is in use.
    Physical,
}

impl LayoutMode {
    fn from(result: u64) -> LayoutMode {
        match result {
            2 => LayoutMode::Physical,
            _ => LayoutMode::Logical,
        }
    }
}

impl std::fmt::Display for LayoutMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LayoutMode::Logical => "logical",
                LayoutMode::Physical => "physical",
            }
        )
    }
}

const KNOWN_PROPERTY_KEYS: [&str; 4] = [
    "supports-mirroring",
    "layout-mode",
    "supports-changing-layout-mode",
    "global-scale-required",
];

#[derive(Debug)]
pub struct KnownProperties {
    pub supports_mirroring: bool,
    /** Represents in what way logical monitors are laid out on the screen. The layout mode can be either of the ones listed below.
     * Absence of this property means the layout mode cannot be changed, and that "logical" mode is assumed to be used. TODO: implement this
     *   - 1 : logical  - the dimension of a logical monitor is derived from the monitor modes associated with it, then scaled using the logical monitor scale.
     *   - 2 : physical - the dimension of a logical monitor is derived from the monitor modes associated with it.
     */
    pub layout_mode: LayoutMode,
    pub supports_changing_layout_mode: bool,

    // True if all the logical monitors must always use the same scale
    pub global_scale_required: bool,
}

impl KnownProperties {
    fn from(result: &dbus::arg::PropMap) -> KnownProperties {
        let as_bool = |prop: &str| -> Option<bool> {
            match result.get(prop).map(|val| val.0.as_u64()).flatten() {
                Some(1) => Some(true),
                Some(0) => Some(false),
                _ => None,
            }
        };

        KnownProperties {
            supports_mirroring: as_bool("supports-mirroring").unwrap_or(true),
            layout_mode: result
                .get("layout-mode")
                .map(|val| val.0.as_u64())
                .flatten()
                .map_or(LayoutMode::Logical, LayoutMode::from),
            supports_changing_layout_mode: as_bool("supports-changing-layout-mode")
                .unwrap_or(false),
            global_scale_required: as_bool("global-scale-required").unwrap_or(false),
        }
    }
}

impl std::fmt::Display for KnownProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "supports-mirroring: {}", self.supports_mirroring)?;
        writeln!(f, "layout-mode: {}", self.layout_mode)?;
        writeln!(
            f,
            "supports-changing-layout-mode: {}",
            self.supports_changing_layout_mode
        )?;
        writeln!(f, "global-scale-required: {}", self.global_scale_required)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DisplayConfig {
    pub serial: u32,
    pub monitors: Vec<PhysicalMonitor>,
    pub logical_monitors: Vec<LogicalMonitor>,
    pub known_properties: KnownProperties,
    pub properties: dbus::arg::PropMap,
}

impl DisplayConfig {
    fn from(
        result: (
            u32,
            Vec<(
                (String, String, String, String),
                Vec<(String, i32, i32, f64, f64, Vec<f64>, dbus::arg::PropMap)>,
                dbus::arg::PropMap,
            )>,
            Vec<(
                i32,
                i32,
                f64,
                u32,
                bool,
                Vec<(String, String, String, String)>,
                dbus::arg::PropMap,
            )>,
            dbus::arg::PropMap,
        ),
    ) -> DisplayConfig {
        let all_properties = result.3;
        let known_properties = KnownProperties::from(&all_properties);

        DisplayConfig {
            serial: result.0,
            monitors: result.1.into_iter().map(PhysicalMonitor::from).collect(),
            logical_monitors: result.2.into_iter().map(LogicalMonitor::from).collect(),
            properties: all_properties
                .into_iter()
                .filter(|(key, _)| !KNOWN_PROPERTY_KEYS.contains(&key.as_str()))
                .collect(),
            known_properties,
        }
    }

    pub fn search(&self, connector: &str) -> Option<(&LogicalMonitor, &PhysicalMonitor)> {
        let physical_monitor = self
            .monitors
            .iter()
            .find(|monitor| monitor.connector == *connector);

        let logical_monitor = self.logical_monitors.iter().find(|monitor| {
            monitor
                .monitors
                .iter()
                .find(|pm| pm.connector == *connector)
                .is_some()
        });

        physical_monitor
            .map(|physical_monitor| {
                logical_monitor.map(|logical_monitor| (logical_monitor, physical_monitor))
            })
            .flatten()
    }

    pub fn format(&self, writer: &mut dyn std::fmt::Write, summary: bool) -> std::fmt::Result {
        if !summary {
            // Print known and unknown properties.
            write!(writer, "{}", self.known_properties)?;
            // TODO: this should be sorted
            for (prop, value) in self.properties.iter() {
                if !KNOWN_PROPERTY_KEYS.contains(&prop.as_str()) {
                    writeln!(writer, "{}: {:?}", prop, &value.0)?;
                }
            }
            writeln!(writer)?;
        }

        // Print logical monitors
        for (i, monitor) in self.logical_monitors.iter().enumerate() {
            writeln!(writer, "logical monitor {}:\n{}", i, monitor)?
        }

        if !summary {
            for monitor in self.monitors.iter() {
                writeln!(writer, "{}", monitor)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for DisplayConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f, false)
    }
}
