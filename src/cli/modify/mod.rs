use gnome_randr::{
    display_config::{self, logical_monitor::Transform, ApplyConfig},
    DisplayConfig,
};
use structopt::StructOpt;

#[derive(Clone, Copy)]
pub enum Rotation {
    Normal,
    Left,
    Right,
    Inverted,
}

impl std::str::FromStr for Rotation {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Rotation::Normal),
            "left" => Ok(Rotation::Left),
            "right" => Ok(Rotation::Right),
            "inverted" => Ok(Rotation::Inverted),
            _ => Err(std::fmt::Error),
        }
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Normal => "normal",
                Rotation::Left => "left",
                Rotation::Right => "right",
                Rotation::Inverted => "inverted",
            }
        )
    }
}

#[derive(StructOpt)]
pub struct CommandOptions {
    #[structopt(
        help = "the connector used for the physical monitor.",
        long_help = "the connector used for the physical monitor you want to modify, e.g. \"HDMI-1\". You can find these with \"query\" (no arguments) if you're unsure."
    )]
    pub connector: String,

    #[structopt(
        short = "r",
        long = "rotate",
        help = "Rotation can be one of 'normal', 'left', 'right' or 'inverted'",
        long_help = "Rotation can be one of 'normal', 'left', 'right' or 'inverted'. This causes the output contents to be rotated in the specified direction. 'right' specifies a clockwise rotation of the picture and 'left' specifies a counter-clockwise rotation."
    )]
    pub rotation: Option<Rotation>,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Error::NotFound => "fatal: unable to find output.",
            }
        )
    }
}

impl std::error::Error for Error {}

trait Action: std::fmt::Display {
    fn apply(&self, config: &mut ApplyConfig);
}

struct RotationAction {
    rotation: Rotation,
}

impl Action for RotationAction {
    fn apply(&self, config: &mut ApplyConfig) {
        println!("{}", self);
        config.transform = match self.rotation {
            Rotation::Normal => Transform::NORMAL,
            Rotation::Left => Transform::R270,
            Rotation::Right => Transform::R90,
            Rotation::Inverted => Transform::R180,
        }
        .bits();
    }
}

impl std::fmt::Display for RotationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting rotation to {}", self.rotation)
    }
}

pub fn handle(
    opts: &CommandOptions,
    config: &DisplayConfig,
    proxy: &dbus::blocking::Proxy<&dbus::blocking::Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (logical_monitor, physical_monitor) =
        config.search(&opts.connector).ok_or(Error::NotFound)?;

    let mut actions = Vec::<Box<dyn Action>>::new();

    if let Some(rotation) = &opts.rotation {
        actions.push(Box::new(RotationAction {
            rotation: *rotation,
        }));
    }

    if actions.is_empty() {
        println!("no changes made.");
    } else {
        let mut apply_config = ApplyConfig::from(logical_monitor, physical_monitor);

        for action in actions.iter() {
            action.apply(&mut apply_config);
        }

        let all_configs = config
            .monitors
            .iter()
            .filter_map(|monitor| {
                if monitor.connector == opts.connector {
                    return Some(apply_config.clone());
                }

                let (logical_monitor, _) = match config.search(&monitor.connector) {
                    Some(monitors) => monitors,
                    None => return None,
                };

                Some(ApplyConfig::from(logical_monitor, monitor))
            })
            .collect();

        config.apply_monitors_config(&proxy, all_configs)?;
    }

    Ok(())
}
