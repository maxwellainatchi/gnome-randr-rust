use std::fmt::{self};

// Information used to identify and describe a monitor
#[derive(Debug, Clone)]
pub struct MonitorDescription {
    // name of the connector (e.g. DP-1, eDP-1 etc)
    pub connector: String,

    // vendor name
    pub vendor: String,

    // product name
    pub product: String,

    // product serial
    pub serial: String,
}

impl MonitorDescription {
    pub fn from(result: (String, String, String, String)) -> MonitorDescription {
        MonitorDescription {
            connector: result.0,
            vendor: result.1,
            product: result.2,
            serial: result.3,
        }
    }
}

impl fmt::Display for MonitorDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //      DVI-D-2 DELL S2340M

        write!(
            f,
            "{} {} {} {}",
            self.connector, self.vendor, self.product, self.serial
        )
    }
}
