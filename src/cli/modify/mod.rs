use gnome_randr::{
    display_config::{logical_monitor::Transform, physical_monitor::PhysicalMonitor, ApplyConfig},
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
        short,
        long = "rotate",
        help = "One of 'normal', 'left', 'right' or 'inverted'",
        long_help = "One of 'normal', 'left', 'right' or 'inverted'. This causes the output contents to be rotated in the specified direction. 'right' specifies a clockwise rotation of the picture and 'left' specifies a counter-clockwise rotation."
    )]
    pub rotation: Option<Rotation>,

    #[structopt(
        short,
        long,
        help = "A valid mode for the given display.",
        long_help = "A valid mode for the given display. To find valid modes use the \"query\" subcommand"
    )]
    pub mode: Option<String>,

    #[structopt(long, help = "Set the given monitor as the primary logical monitor")]
    pub primary: bool,
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

trait Action<'a>: std::fmt::Display {
    fn apply(&self, config: &mut ApplyConfig<'a>, physical_monitor: &PhysicalMonitor);
}

struct RotationAction {
    rotation: Rotation,
}

impl Action<'_> for RotationAction {
    fn apply(&self, config: &mut ApplyConfig, _: &PhysicalMonitor) {
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

struct ModeAction<'a> {
    mode: &'a str,
}

impl<'a> Action<'a> for ModeAction<'a> {
    fn apply(&self, config: &mut ApplyConfig<'a>, physical_monitor: &PhysicalMonitor) {
        config
            .monitors
            .iter_mut()
            .find(|monitor| monitor.connector == physical_monitor.connector)
            .unwrap()
            .mode_id = self.mode;
    }
}

impl std::fmt::Display for ModeAction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting mode to {}", self.mode)
    }
}

struct PrimaryAction {}

impl<'a> Action<'a> for PrimaryAction {
    fn apply(&self, config: &mut ApplyConfig<'a>, _: &PhysicalMonitor) {
        config.primary = true;
    }
}

impl std::fmt::Display for PrimaryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting monitor as primary")
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
    let mut primary_is_changing = false;

    if let Some(rotation) = &opts.rotation {
        actions.push(Box::new(RotationAction {
            rotation: *rotation,
        }));
    }

    if let Some(mode_id) = &opts.mode {
        actions.push(Box::new(ModeAction { mode: mode_id }))
    }

    if opts.primary {
        actions.push(Box::new(PrimaryAction {}));
        primary_is_changing = true;
    }

    if actions.is_empty() {
        println!("no changes made.");
    } else {
        let mut apply_config = ApplyConfig::from(logical_monitor, physical_monitor);

        for action in actions.iter() {
            println!("{}", &action);
            action.apply(&mut apply_config, physical_monitor);
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

                let mut apply_config = ApplyConfig::from(logical_monitor, monitor);

                if primary_is_changing {
                    apply_config.primary = false;
                }

                Some(apply_config)
            })
            .collect();

        config.apply_monitors_config(proxy, all_configs)?;
    }

    Ok(())
}
