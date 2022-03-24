/**
 * An output represents a physical screen, connected somewhere to
 * the computer. Floating connectors are not exposed in the API.
 */
#[derive(Debug)]
pub struct Output {
    // the ID in the API
    pub id: u32,
    // the low-level ID of this output (XID or KMS handle)
    pub winsys_id: i64,
    // the CRTC that is currently driving this output, or -1 if the output is disabled
    pub current_crtc: i32,
    // all CRTCs that can control this output
    pub possible_crtcs: Vec<u32>,
    // the name of the connector to which the output is attached (like VGA1 or HDMI)
    pub name: String,
    // valid modes for this output
    pub modes: Vec<u32>,
    // valid clones for this output, ie other outputs that can be assigned the same CRTC as this one;
    // if you want to mirror two outputs that don't have each other in the clone list, you must configure two different CRTCs for the same geometry
    pub clones: Vec<u32>,

    // other high-level properties that affect this output; they are not necessarily reflected in the hardware.
    // Known properties:
    //  - "vendor" (s): (readonly) the human readable name of the manufacturer
    //  - "product" (s): (readonly) the human readable name of the display model
    //  - "serial" (s): (readonly) the serial number of this particular hardware part
    //  - "display-name" (s): (readonly) a human readable name of this output, to be shown in the UI
    //  - "backlight" (i): (readonly, use the specific interface) the backlight value as a percentage (-1 if not supported)
    //  - "primary" (b): whether this output is primary or not
    //  - "presentation" (b): whether this output is for presentation only
    //
    // Note: properties might be ignored if not consistently applied to all outputs in the same clone group.
    // In general, it's expected that presentation or primary outputs will not be cloned.
    pub properties: dbus::arg::PropMap,
}

impl Output {
    pub fn from(
        result: (
            u32,
            i64,
            i32,
            Vec<u32>,
            String,
            Vec<u32>,
            Vec<u32>,
            dbus::arg::PropMap,
        ),
    ) -> Output {
        Output {
            id: result.0,
            winsys_id: result.1,
            current_crtc: result.2,
            possible_crtcs: result.3,
            name: result.4,
            modes: result.5,
            clones: result.6,
            properties: result.7,
        }
    }
}
