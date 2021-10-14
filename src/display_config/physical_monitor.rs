#[derive(Debug)]
pub struct Mode {
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

impl Mode {
    pub fn from(result: (String, i32, i32, f64, f64, Vec<f64>, dbus::arg::PropMap)) -> Mode {
        Mode {
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
pub struct PhysicalMonitor {
    // connector name (e.g. HDMI-1, DP-1, etc)
    pub connector: String,
    // vendor name
    pub vendor: String,
    // product name
    pub product: String,
    // product serial
    pub serial: String,
    // available modes
    pub modes: Vec<Mode>,

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

impl PhysicalMonitor {
    pub fn from(
        result: (
            (String, String, String, String),
            Vec<(String, i32, i32, f64, f64, Vec<f64>, dbus::arg::PropMap)>,
            dbus::arg::PropMap,
        ),
    ) -> PhysicalMonitor {
        PhysicalMonitor {
            connector: result.0 .0,
            vendor: result.0 .1,
            product: result.0 .2,
            serial: result.0 .3,
            modes: result.1.into_iter().map(|mode| Mode::from(mode)).collect(),
            properties: result.2,
        }
    }
}
