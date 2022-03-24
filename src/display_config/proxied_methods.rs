use dbus::{
    arg::PropMap,
    blocking::{Connection, Proxy},
};

use super::{
    models::{Crtc, LogicalMonitor, PhysicalMonitor},
    resources::Resources,
    DisplayConfig,
};

type Result<T> = std::prelude::rust_2021::Result<T, dbus::Error>;

#[derive(Debug, Clone, Copy)]
pub struct ApplyMonitor<'a> {
    pub connector: &'a str,
    pub mode_id: &'a str,
}

impl ApplyMonitor<'_> {
    pub fn serialize(&self) -> (&str, &str, PropMap) {
        (self.connector, self.mode_id, PropMap::new())
    }
}

#[derive(Debug, Clone)]
pub struct ApplyConfig<'a> {
    pub x_pos: i32,
    pub y_pos: i32,
    pub scale: f64,
    pub transform: u32,
    pub primary: bool,
    pub monitors: Vec<ApplyMonitor<'a>>,
}

impl ApplyConfig<'_> {
    pub fn from<'a>(
        logical_monitor: &LogicalMonitor,
        physical_monitor: &'a PhysicalMonitor,
    ) -> ApplyConfig<'a> {
        ApplyConfig {
            x_pos: logical_monitor.x,
            y_pos: logical_monitor.y,
            scale: logical_monitor.scale,
            transform: logical_monitor.transform.bits(),
            primary: logical_monitor.primary,
            monitors: vec![ApplyMonitor {
                connector: &physical_monitor.connector,
                mode_id: &physical_monitor
                    .modes
                    .iter()
                    .find(|mode| mode.known_properties.is_current)
                    .unwrap()
                    .id,
            }],
        }
    }

    pub fn serialize(&self) -> (i32, i32, f64, u32, bool, Vec<(&str, &str, PropMap)>) {
        (
            self.x_pos,
            self.y_pos,
            self.scale,
            self.transform,
            self.primary,
            self.monitors
                .iter()
                .map(|monitor| monitor.serialize())
                .collect(),
        )
    }
}

impl DisplayConfig {
    pub fn apply_monitors_config(
        &self,
        proxy: &Proxy<&Connection>,
        configs: Vec<ApplyConfig>,
        persistent: bool,
    ) -> Result<()> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        let result = proxy.apply_monitors_config(
            self.serial,
            if persistent { 2 } else { 1 },
            configs.iter().map(|config| config.serialize()).collect(),
            PropMap::new(),
        );

        if let Err(err) = &result {
            println!("{:?}", err);
        }
        result
    }

    pub fn get_current_state(proxy: &Proxy<&Connection>) -> Result<DisplayConfig> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        let raw_output = proxy.get_current_state()?;
        Ok(DisplayConfig::from(raw_output))
    }
}

#[derive(Debug, Clone)]
pub struct Gamma {
    pub red: Vec<u16>,
    pub green: Vec<u16>,
    pub blue: Vec<u16>,
}

impl Gamma {
    pub fn from(result: (Vec<u16>, Vec<u16>, Vec<u16>)) -> Gamma {
        Gamma {
            red: result.0,
            green: result.1,
            blue: result.2,
        }
    }

    pub fn gamma_info(&self) -> GammaInfo {
        get_gamma_info(self)
    }
}

/* Returns the index of the last value in an array < 0xffff */
fn find_last_non_clamped(arr: &[u16]) -> usize {
    for (i, el) in arr.iter().enumerate().rev() {
        if *el < 0xffff {
            return i;
        }
    }
    0
}

#[derive(Debug, Clone, Copy)]
pub struct GammaInfo {
    pub brightness: f64,
    pub red: f64,
    pub blue: f64,
    pub green: f64,
}

impl GammaInfo {
    pub fn get_gamma(&self, size: usize) -> Gamma {
        get_gamma(*self, size)
    }
}

fn get_gamma_info(crtc_gamma: &Gamma) -> GammaInfo {
    let size = crtc_gamma.red.len();

    /*
     * Here is a bit tricky because gamma is a whole curve for each
     * color.  So, typically, we need to represent 3 * 256 values as 3 + 1
     * values.  Therefore, we approximate the gamma curve (v) by supposing
     * it always follows the way we set it: a power function (i^g)
     * multiplied by a brightness (b).
     * v = i^g * b
     * so g = (ln(v) - ln(b))/ln(i)
     * and b can be found using two points (v1,i1) and (v2, i2):
     * b = e^((ln(v2)*ln(i1) - ln(v1)*ln(i2))/ln(i1/i2))
     * For the best resolution, we select i2 at the highest place not
     * clamped and i1 at i2/2. Note that if i2 = 1 (as in most normal
     * cases), then b = v2.
     */
    let last_red = find_last_non_clamped(&crtc_gamma.red);
    let last_green = find_last_non_clamped(&crtc_gamma.green);
    let last_blue = find_last_non_clamped(&crtc_gamma.blue);
    let mut best_array = &crtc_gamma.red;
    let mut last_best = last_red;
    if last_green > last_best {
        last_best = last_green;
        best_array = &crtc_gamma.green;
    }
    if last_blue > last_best {
        last_best = last_blue;
        best_array = &crtc_gamma.blue;
    }
    if last_best == 0 {
        last_best = 1;
    }

    let middle = (last_best / 2) as usize;
    let i1 = (middle + 1) as f64 / size as f64;
    let v1 = (best_array[middle]) as f64 / 65535.0;
    let i2 = (last_best + 1) as f64 / size as f64;
    let v2 = (best_array[last_best]) as f64 / 65535.0;

    if v2 < 0.0001 {
        GammaInfo {
            brightness: 0.0,
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    } else {
        let brightness = if (last_best + 1) == size {
            v2
        } else {
            ((v2.ln() * i1.ln() - v1.ln() * i2.ln()) / (i1 / i2).ln()).exp()
        };

        let calc = |channel: &Vec<u16>, last: usize| {
            (channel[last / 2] as f64 / brightness / 65535.0).ln()
                / (((last / 2) + 1) as f64 / size as f64).ln()
        };
        GammaInfo {
            brightness,
            red: calc(&crtc_gamma.red, last_red),
            green: calc(&crtc_gamma.green, last_green),
            blue: calc(&crtc_gamma.blue, last_blue),
        }
    }
}

fn get_gamma(mut gamma: GammaInfo, size: usize) -> Gamma {
    if gamma.red == 0.0 {
        gamma.red = 1.0;
    }
    if gamma.green == 0.0 {
        gamma.green = 1.0;
    }
    if gamma.blue == 0.0 {
        gamma.blue = 1.0;
    }

    let gamma_red = 1.0 / gamma.red;
    let gamma_green = 1.0 / gamma.green;
    let gamma_blue = 1.0 / gamma.blue;

    let mut result = Gamma {
        red: vec![],
        green: vec![],
        blue: vec![],
    };

    for i in 0..size {
        let apply = |channel: &mut Vec<u16>, param: f64| {
            if (param - 1.0).abs() < f64::EPSILON && (gamma.brightness - 1.0).abs() < f64::EPSILON {
                channel.push((i as f64 / (size - 1) as f64 * 65535.0) as u16);
            } else {
                channel.push(
                    (((i as f64 / (size - 1) as f64).powf(param) * gamma.brightness).min(1.0)
                        * 65535.0) as u16,
                )
            }
        };

        apply(&mut result.red, gamma_red);
        apply(&mut result.green, gamma_green);
        apply(&mut result.blue, gamma_blue);
    }

    result
}

impl Resources {
    pub fn get_resources(proxy: &Proxy<&Connection>) -> Result<Resources> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        let raw_output = proxy.get_resources()?;
        Ok(Resources::from(raw_output))
    }

    pub fn get_crtc_gamma(&self, proxy: &Proxy<&Connection>, crtc: &Crtc) -> Result<Gamma> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        let result = proxy.get_crtc_gamma(self.serial, crtc.id)?;
        Ok(Gamma::from(result))
    }

    pub fn set_crtc_gamma(
        &self,
        proxy: &Proxy<&Connection>,
        crtc: &Crtc,
        gamma: Gamma,
    ) -> Result<()> {
        use super::raw::OrgGnomeMutterDisplayConfig;

        proxy.set_crtc_gamma(self.serial, crtc.id, gamma.red, gamma.green, gamma.blue)
    }
}
