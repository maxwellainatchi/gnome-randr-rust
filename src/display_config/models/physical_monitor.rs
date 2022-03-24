#[derive(Debug)]
pub struct KnownModeProperties {
    pub is_current: bool,
    pub is_preferred: bool,
}

const KNOWN_MODE_PROPERTY_KEYS: [&str; 2] = ["is-current", "is-preferred"];
impl KnownModeProperties {
    fn from(properties: &dbus::arg::PropMap) -> KnownModeProperties {
        // TODO: move into a helper
        let as_bool = |key: &str| -> Option<bool> {
            match properties.get(key).map(|val| val.0.as_u64()).flatten() {
                Some(1) => Some(true),
                Some(0) => Some(false),
                _ => None,
            }
        };

        KnownModeProperties {
            is_current: as_bool("is-current").unwrap_or(false),
            is_preferred: as_bool("is-preferred").unwrap_or(false),
        }
    }
}

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

    pub known_properties: KnownModeProperties,
    pub properties: dbus::arg::PropMap,
}

impl Mode {
    pub fn from(result: (String, i32, i32, f64, f64, Vec<f64>, dbus::arg::PropMap)) -> Mode {
        let all_properties = result.6;
        let known_properties = KnownModeProperties::from(&all_properties);

        Mode {
            id: result.0,
            width: result.1,
            height: result.2,
            refresh_rate: result.3,
            preferred_scale: result.4,
            supported_scales: result.5,
            known_properties,
            properties: all_properties
                .into_iter()
                .filter(|(key, _)| !KNOWN_MODE_PROPERTY_KEYS.contains(&key.as_str()))
                .collect(),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn icon(val: bool, icon: &str) -> &str {
            if val {
                icon
            } else {
                ""
            }
        }
        fn column(val: String, width: usize, right: bool) -> String {
            if right {
                format!("{:>width$}", val, width = width)
            } else {
                format!("{:<width$}", val, width = width)
            }
        }

        write!(
            f,
            "{}\t{}\t{}\t{}",
            column(format!("{}", self.id), 30, true),
            column(format!("{}x{}", self.width, self.height), 10, false),
            column(
                format!(
                    "{:.2}{}{}",
                    self.refresh_rate,
                    icon(self.known_properties.is_current, "*"),
                    icon(self.known_properties.is_preferred, "+")
                ),
                10,
                false
            ),
            format!(
                "[{}]",
                self.supported_scales
                    .iter()
                    .map(|scale| format!(
                        "x{:.2}{}",
                        scale,
                        icon(*scale == self.preferred_scale, "+")
                    ))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        )
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
            modes: result.1.into_iter().map(Mode::from).collect(),
            properties: result.2,
        }
    }
}

impl std::fmt::Display for PhysicalMonitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {}",
            self.connector, self.vendor, self.product, self.serial
        )?;

        for mode in self.modes.iter() {
            writeln!(f, "{}", &mode)?;
        }

        // TODO: improve logging of properties
        for (prop, value) in self.properties.iter() {
            writeln!(f, "{}: {:?}", prop, &value.0)?;
        }
        Ok(())
    }
}
