// monitors displaying this logical monitor
#[derive(Debug)]
pub struct Monitor {
    // name of the connector (e.g. DP-1, eDP-1 etc)
    pub connector: String,

    // vendor name
    pub vendor: String,

    // product name
    pub product: String,

    // product serial
    pub serial: String,
}

impl Monitor {
    pub fn from(result: (String, String, String, String)) -> Monitor {
        Monitor {
            connector: result.0,
            vendor: result.1,
            product: result.2,
            serial: result.3,
        }
    }
}

//represent current logical monitor configuration
#[derive(Debug)]
pub struct LogicalMonitor {
    // x position
    pub x: i32,
    // y position
    pub y: i32,
    // scale
    pub scale: f64,

    /**
     * Posisble transform values:
     *   0: normal
     *   1: 90°
     *   2: 180°
     *   3: 270°
     *   4: flipped
     *   5: 90° flipped
     *   6: 180° flipped
     *   7: 270° flipped
     * TODO: change to enum
     */
    pub transform: u32,

    // true if this is the primary logical monitor
    pub primary: bool,

    // monitors displaying this logical monitor
    pub monitors: Vec<Monitor>,

    // possibly other properties
    pub properties: dbus::arg::PropMap,
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
            x: result.0,
            y: result.1,
            scale: result.2,
            transform: result.3,
            primary: result.4,
            monitors: result
                .5
                .into_iter()
                .map(|monitor| Monitor::from(monitor))
                .collect(),
            properties: result.6,
        }
    }
}
