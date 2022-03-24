use super::models::{Crtc, Output};

/**
 * A mode represents a set of parameters that are applied to each output, such as resolution and refresh rate.
 * It is a separate object so that it can be referenced by CRTCs and outputs.
 * Multiple outputs in the same CRTCs must all have the same mode.
 */
#[derive(Debug)]
pub struct Mode {
    // the ID in the API
    pub id: u32,
    // the low-level ID of this mode
    pub winsys_id: i64,
    // the next two properties represent the resolution
    pub width: u32,
    pub height: u32,

    // refresh rate
    pub frequency: f64,
    // mode flags as defined in xf86drmMode.h and randr.h
    pub flags: u32,
}

impl Mode {
    pub fn from(result: (u32, i64, u32, u32, f64, u32)) -> Mode {
        Mode {
            id: result.0,
            winsys_id: result.1,
            width: result.2,
            height: result.3,
            frequency: result.4,
            flags: result.5,
        }
    }
}

#[derive(Debug)]
pub struct Resources {
    /**
    a unique identifier representing the current state of the screen. It must be passed back to ApplyConfiguration()
    and will be increased for every configuration change (so that mutter can detect that the new configuration is based on old state).
    */
    pub serial: u32,

    pub crtcs: Vec<Crtc>,
    pub outputs: Vec<Output>,
    pub modes: Vec<Mode>,
    pub max_screen_width: i32,
    pub max_screen_height: i32,
}

impl Resources {
    pub fn from(
        result: (
            u32,
            Vec<(
                u32,
                i64,
                i32,
                i32,
                i32,
                i32,
                i32,
                u32,
                Vec<u32>,
                dbus::arg::PropMap,
            )>,
            Vec<(
                u32,
                i64,
                i32,
                Vec<u32>,
                String,
                Vec<u32>,
                Vec<u32>,
                dbus::arg::PropMap,
            )>,
            Vec<(u32, i64, u32, u32, f64, u32)>,
            i32,
            i32,
        ),
    ) -> Resources {
        Resources {
            serial: result.0,
            crtcs: result.1.into_iter().map(Crtc::from).collect(),
            outputs: result.2.into_iter().map(Output::from).collect(),
            modes: result.3.into_iter().map(Mode::from).collect(),
            max_screen_width: result.4,
            max_screen_height: result.5,
        }
    }
}
