use dbus;
use dbus::blocking::Connection;
use std::time::Duration;

mod display_config_raw;

#[derive(Debug)]
pub struct DisplayConfigLogicalMonitorsMode {
    // mode ID
    pub id: String,
    // width in physical pixels
    pub width: i32,
    // height in physical pixels
    pub height: i32,
    // refresh rate
    pub refresh_rate: f64,
    // scale preferred as per calculations
    pub preferred_scale: f64,
    // scales supported by this mode
    pub supported_scales: Vec<f64>,
    /**
     * optional properties, including:
     *   - "is-current" (b): the mode is currently active mode
     *   - "is-preferred" (b): the mode is the preferred mode
     */
    pub properties: dbus::arg::PropMap,
}

impl DisplayConfigLogicalMonitorsMode {
    pub fn from(
        result: (String, i32, i32, f64, f64, Vec<f64>, dbus::arg::PropMap),
    ) -> DisplayConfigLogicalMonitorsMode {
        DisplayConfigLogicalMonitorsMode {
            id: result.0,
            width: result.1,
            height: result.2,
            refresh_rate: result.3,
            preferred_scale: result.4,
            supported_scales: result.5,
            properties: result.6,
        }
    }
}

/// represent connected physical monitors
#[derive(Debug)]
pub struct DisplayConfigLogicalMonitor {
    // connector name (e.g. HDMI-1, DP-1, etc)
    pub connector: String,
    // vendor name
    pub vendor: String,
    // product name
    pub product: String,
    // product serial
    pub serial: String,
    // available modes
    pub modes: Vec<DisplayConfigLogicalMonitorsMode>,

    /**
     * optional properties, including:
     *   - "width-mm" (i): physical width of monitor in millimeters
     *   - "height-mm" (i): physical height of monitor in millimeters
     *   - "is-underscanning" (b): whether underscanning is enabled
     *                   (absence of this means underscanning
     *                   not being supported)
     *   - "max-screen-size" (ii): the maximum size a screen may have
     *                   (absence of this means unlimited screen
     *                   size)
     *   - "is-builtin" (b): whether the monitor is built in, e.g. a
     *           laptop panel (absence of this means it is
     *           not built in)
     *   - "display-name" (s): a human readable display name of the monitor
     *   Possible mode flags:
     *   1 : preferred mode
     *   2 : current mode
     */
    pub properties: dbus::arg::PropMap,
}

impl DisplayConfigLogicalMonitor {
    pub fn from(
        result: (
            (String, String, String, String),
            Vec<(String, i32, i32, f64, f64, Vec<f64>, dbus::arg::PropMap)>,
            dbus::arg::PropMap,
        ),
    ) -> DisplayConfigLogicalMonitor {
        DisplayConfigLogicalMonitor {
            connector: result.0 .0,
            vendor: result.0 .1,
            product: result.0 .2,
            serial: result.0 .3,
            modes: result
                .1
                .into_iter()
                .map(|mode| DisplayConfigLogicalMonitorsMode::from(mode))
                .collect(),
            properties: result.2,
        }
    }
}

#[derive(Debug)]
pub struct DisplayConfig {
    pub serial: u32,
    pub logical_monitors: Vec<DisplayConfigLogicalMonitor>,
}

impl DisplayConfig {
    pub fn from(
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
            logical_monitors: result
                .1
                .into_iter()
                .map(|logical_monitor| DisplayConfigLogicalMonitor::from(logical_monitor))
                .collect(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy(
        "org.gnome.Mutter.DisplayConfig",
        "/org/gnome/Mutter/DisplayConfig",
        Duration::from_millis(5000),
    );

    use display_config_raw::OrgGnomeMutterDisplayConfig;

    let raw_output = proxy.get_current_state()?;
    let config = DisplayConfig::from(raw_output);

    // Let's print all the names to stdout.
    println!("{:#?}", &config);

    Ok(())
}
