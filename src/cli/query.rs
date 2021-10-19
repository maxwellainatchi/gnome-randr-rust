use gnome_randr::DisplayConfig;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandOptions {
    #[structopt(
        short,
        long,
        help = "the connector used for the physical monitor.",
        long_help = "query by the connector used for the physical monitor, e.g. \"HDMI-1\". You can find these with \"query\" (no arguments) if you're unsure."
    )]
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
            let (logical_monitor, physical_monitor) =
                config.search(connector).ok_or(Error::NotFound)?;

            format!("{}\n{}", logical_monitor, physical_monitor)
        }
        None => format!("{}", config),
    })
}
