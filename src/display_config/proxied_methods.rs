use dbus::{
    arg::PropMap,
    blocking::{Connection, Proxy},
};

use super::{logical_monitor::LogicalMonitor, physical_monitor::PhysicalMonitor, DisplayConfig};

type Result<T> = std::prelude::rust_2021::Result<T, dbus::Error>;

#[derive(Debug, Clone, Copy)]
pub struct ApplyMonitor<'a> {
    pub connector: &'a str,
    pub mode_id: &'a str,
}

impl ApplyMonitor<'_> {
    pub fn serialize(&self) -> (&str, &str, PropMap) {
        (self.connector, self.mode_id, PropMap::new())
    }
}

#[derive(Debug, Clone)]
pub struct ApplyConfig<'a> {
    pub x_pos: i32,
    pub y_pos: i32,
    pub scale: f64,
    pub transform: u32,
    pub primary: bool,
    pub monitors: Vec<ApplyMonitor<'a>>,
}

impl ApplyConfig<'_> {
    pub fn from<'a>(
        logical_monitor: &LogicalMonitor,
        physical_monitor: &'a PhysicalMonitor,
    ) -> ApplyConfig<'a> {
        ApplyConfig {
            x_pos: logical_monitor.x,
            y_pos: logical_monitor.y,
            scale: logical_monitor.scale,
            transform: logical_monitor.transform.bits(),
            primary: logical_monitor.primary,
            monitors: vec![ApplyMonitor {
                connector: &physical_monitor.connector,
                mode_id: &physical_monitor
                    .modes
                    .iter()
                    .find(|mode| mode.known_properties.is_current)
                    .unwrap()
                    .id,
            }],
        }
    }

    pub fn serialize(&self) -> (i32, i32, f64, u32, bool, Vec<(&str, &str, PropMap)>) {
        (
            self.x_pos,
            self.y_pos,
            self.scale,
            self.transform,
            self.primary,
            self.monitors
                .iter()
                .map(|monitor| monitor.serialize())
                .collect(),
        )
    }
}

impl DisplayConfig {
    pub fn apply_monitors_config(
        &self,
        proxy: &Proxy<&Connection>,
        configs: Vec<ApplyConfig>,
        persistent: bool,
    ) -> Result<()> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        let result = proxy.apply_monitors_config(
            self.serial,
            if persistent { 2 } else { 1 },
            configs.iter().map(|config| config.serialize()).collect(),
            PropMap::new(),
        );

        if let Err(err) = &result {
            println!("{:?}", err);
        }
        result
    }

    pub fn get_current_state(proxy: &Proxy<&Connection>) -> Result<DisplayConfig> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        let raw_output = proxy.get_current_state()?;
        Ok(DisplayConfig::from(raw_output))
    }
}
