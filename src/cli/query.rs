use crate::display_config::DisplayConfig;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandOptions {
    #[structopt(short, long)]
    pub connector: Option<String>,
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

pub fn handle(opts: &CommandOptions, config: &DisplayConfig) -> Result<String, Box<Error>> {
    Ok(match &opts.connector {
        Some(connector) => {
            let physical_monitor = config
                .monitors
                .iter()
                .find(|monitor| monitor.connector == *connector)
                .ok_or(Error::NotFound)?;

            let logical_monitor = config
                .logical_monitors
                .iter()
                .find(|monitor| {
                    match monitor
                        .monitors
                        .iter()
                        .find(|pm| pm.connector == *connector)
                    {
                        Some(_) => true,
                        None => false,
                    }
                })
                .ok_or(Error::NotFound)?;

            format!("{}\n{}", logical_monitor, physical_monitor)
        }
        None => format!("{}", config),
    })
}
