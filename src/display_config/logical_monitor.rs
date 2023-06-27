use std::fmt::{self};

use super::monitor_models::{Transform, MonitorDescription};

//represent current logical monitor configuration
#[derive(Debug)]
pub struct LogicalMonitor {
    // The transformation that describes where and how to display this logical monitor
    pub transform: Transform,

    // true if this is the primary logical monitor
    pub primary: bool,

    // monitors displaying this logical monitor
    pub monitors: Vec<MonitorDescription>,

    // possibly other properties
    pub properties: dbus::arg::PropMap,
}

impl Clone for LogicalMonitor {
    fn clone(&self) -> Self {
        Self {
            transform: self.transform.clone(),
            primary: self.primary,
            monitors: self.monitors.clone(),
            properties: dbus::arg::PropMap::new(),
        }
    }
}

impl LogicalMonitor {
    pub fn from(
        result: (
            i32,
            i32,
            f64,
            u32,
            bool,
            Vec<(String, String, String, String)>,
            dbus::arg::PropMap,
        ),
    ) -> LogicalMonitor {
        LogicalMonitor {
            transform: Transform::from(
                result.0, 
                result.1, 
                result.2, 
                result.3
            ),
            primary: result.4,
            monitors: result
                .5
                .into_iter()
                .map(MonitorDescription::from)
                .collect(),
            properties: result.6,
        }
    }

    pub fn to_result<'a>(
        &self,
        mode_id: &'a str,
    ) -> (
        i32,
        i32,
        f64,
        u32,
        bool,
        Vec<(&str, &'a str, dbus::arg::PropMap)>,
    ) {
        (
            self.transform.displacement.x,
            self.transform.displacement.y,
            self.transform.displacement.scale,
            self.transform.orientation.bits(),
            self.primary,
            self.monitors
                .iter()
                .map(|monitor| {
                    (
                        monitor.connector.as_str(),
                        mode_id,
                        dbus::arg::PropMap::new(),
                    )
                })
                .collect(),
        )
    }
}

impl std::fmt::Display for LogicalMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // x: 0 y: 820, scale: 1.0, rotation: normal, primary: no
        // associated physical monitors:
        //      DVI-D-2 DELL S2340M

        writeln!(
            f,
            "{}, primary: {}",
            self.transform,
            if self.primary { "yes" } else { "no" }
        )?;

        writeln!(f, "associated physical monitors:")?;

        for monitor in self.monitors.iter() {
            writeln!(f, "\t{}", monitor)?
        }

        // TODO: Print properties?

        Ok(())
    }
}
