use std::fmt::{self};

use bitflags::bitflags;

// monitors displaying this logical monitor
#[derive(Debug, Clone)]
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

impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //      DVI-D-2 DELL S2340M

        write!(
            f,
            "{} {} {} {}",
            self.connector, self.vendor, self.product, self.serial
        )
    }
}

bitflags! {
pub struct Transform: u32 {
    const NORMAL = 0b000;
    const R90 = 0b001;
    const R180 = 0b010;
    const R270 = Self::R90.bits | Self::R180.bits;

    const FLIPPED = 0b100;
    const F90 = Self::R90.bits | Self::FLIPPED.bits;
    const F180 = Self::R180.bits | Self::FLIPPED.bits;
    const F270 = Self::R270.bits | Self::FLIPPED.bits;
}
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = if self.contains(Transform::R270) {
            "left"
        } else if self.contains(Transform::R180) {
            "inverted"
        } else if self.contains(Transform::R90) {
            "right"
        } else {
            "normal"
        };

        write!(
            f,
            "{}{}",
            if self.contains(Transform::FLIPPED) {
                "Flipped "
            } else {
                ""
            },
            display
        )
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
    pub transform: Transform,

    // true if this is the primary logical monitor
    pub primary: bool,

    // monitors displaying this logical monitor
    pub monitors: Vec<Monitor>,

    // possibly other properties
    pub properties: dbus::arg::PropMap,
}

impl Clone for LogicalMonitor {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            scale: self.scale.clone(),
            transform: self.transform.clone(),
            primary: self.primary.clone(),
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
            x: result.0,
            y: result.1,
            scale: result.2,
            transform: Transform::from_bits_truncate(result.3),
            primary: result.4,
            monitors: result
                .5
                .into_iter()
                .map(|monitor| Monitor::from(monitor))
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
            self.x,
            self.y,
            self.scale,
            self.transform.bits(),
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
            "x: {}, y: {}, scale: {}, rotation: {}, primary: {}",
            self.x,
            self.y,
            self.scale,
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
