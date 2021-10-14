pub mod logical_monitor;
pub mod physical_monitor;
mod raw;

// Config properties/comments are sourced from https://github.com/jadahl/gnome-monitor-config/blob/master/src/org.gnome.Mutter.DisplayConfig.xml

use logical_monitor::LogicalMonitor;
use physical_monitor::PhysicalMonitor;

#[derive(Debug)]
pub struct DisplayConfig {
    pub serial: u32,
    pub monitors: Vec<PhysicalMonitor>,
    pub logical_monitors: Vec<LogicalMonitor>,
    // @layout_mode current layout mode represents the way logical monitors
    // are layed out on the screen. Possible modes include:
    //   1 : physical
    //   2 : logical
    //
    // With physical layout mode, each logical monitor has the same dimensions
    // an the monitor modes of the associated monitors assigned to it, no
    // matter what scale is in use.
    // With logical mode, the dimension of a logical monitor is the dimension
    // of the monitor mode, divided by the logical monitor scale.
    //
    // Possible @properties are:
    // * "supports-mirroring" (b): FALSE if mirroring not supported; TRUE or not
    //                             present if mirroring is supported.
    // * "layout-mode" (u): Represents in what way logical monitors are laid
    // 		     out on the screen. The layout mode can be either
    // 		     of the ones listed below. Absence of this property
    // 		     means the layout mode cannot be changed, and that
    // 		     "logical" mode is assumed to be used.
    //     * 1 : logical  - the dimension of a logical monitor is derived from
    // 		     the monitor modes associated with it, then scaled
    // 		     using the logical monitor scale.
    //     * 2 : physical - the dimension of a logical monitor is derived from
    // 		     the monitor modes associated with it.
    // * "supports-changing-layout-mode" (b): True if the layout mode can be
    // 				       changed. Absence of this means the
    // 				       layout mode cannot be changed.
    // * "global-scale-required" (b): True if all the logical monitors must
    // 			       always use the same scale. Absence of
    // 			       this means logical monitor scales can
    // 			       differ.
    pub properties: dbus::arg::PropMap,
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
            logical_monitors: result
                .2
                .into_iter()
                .map(|monitor| LogicalMonitor::from(monitor))
                .collect(),
            properties: result.3,
        }
    }
}

impl std::fmt::Display for DisplayConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}
