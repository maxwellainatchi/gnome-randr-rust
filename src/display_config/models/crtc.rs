/**
 * A CRTC (CRT controller) is a logical monitor, ie a portion
 * of the compositor coordinate space. It might correspond
 * to multiple monitors, when in clone mode, but not that
 * it is possible to implement clone mode also by setting different
 * CRTCs to the same coordinates.
 *
 * The number of CRTCs represent the maximum number of monitors
 * that can be set to expand and it is a HW constraint; if more
 * monitors are connected, then necessarily some will clone. This
 * is complementary to the concept of the encoder (not exposed in
 * the API), which groups outputs that necessarily will show the
 * same image (again a HW constraint).
*/
#[derive(Debug)]
pub struct Crtc {
    // the ID in the API of this CRTC
    pub id: u32,
    // the low-level ID of this CRTC (which might be a XID, a KMS handle or something entirely different)
    pub winsys_id: i64,

    // the next 4 properties represent the geometry of this CRTC (might be invalid if the CRTC is not in use)
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    // the current mode of the CRTC, or -1 if this CRTC is not used
    // Note: the size of the mode will always correspond to the width and height of the CRTC
    pub current_mode: i32,
    // the current transform (espressed according to the wayland protocol)
    pub current_transform: u32,
    // all possible transforms
    pub transforms: Vec<u32>,
    // other high-level properties that affect this CRTC; they are not necessarily reflected in the hardware.
    // No property is specified in this version of the API.
    pub properties: dbus::arg::PropMap,
}

impl Crtc {
    pub fn from(
        result: (
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
        ),
    ) -> Crtc {
        Crtc {
            id: result.0,
            winsys_id: result.1,
            x: result.2,
            y: result.3,
            width: result.4,
            height: result.5,
            current_mode: result.6,
            current_transform: result.7,
            transforms: result.8,
            properties: result.9,
        }
    }
}
