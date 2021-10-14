pub mod physical_monitor;
mod raw;

use physical_monitor::PhysicalMonitor;

#[derive(Debug)]
pub struct DisplayConfig {
    pub serial: u32,
    pub monitors: Vec<PhysicalMonitor>,
}

impl DisplayConfig {
    pub fn get_current_state(
        proxy: &dbus::blocking::Proxy<&dbus::blocking::Connection>,
    ) -> Result<DisplayConfig, dbus::Error> {
        use raw::OrgGnomeMutterDisplayConfig;

        let raw_output = proxy.get_current_state()?;
        Ok(DisplayConfig::from(raw_output))
    }

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
        DisplayConfig {
            serial: result.0,
            monitors: result
                .1
                .into_iter()
                .map(|monitor| PhysicalMonitor::from(monitor))
                .collect(),
        }
    }
}
