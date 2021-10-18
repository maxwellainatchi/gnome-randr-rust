use dbus;
use dbus::blocking::Connection;
use std::time::Duration;
use structopt::{self, StructOpt};

mod display_config;

#[derive(StructOpt)]
struct QueryCommandOpts {
    #[structopt(short, long)]
    pub connector: Option<String>,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(
        about = "Query returns information about the current state of the monitors. This is the default subcommand."
    )]
    Query(QueryCommandOpts),
}

#[derive(StructOpt)]
#[structopt(
    about = "A program to query information about and manipulate displays on Gnome with Wayland.",
    long_about = "A program to query information about and manipulate displays on Gnome with Wayland.\n\nDefault command is `query`."
)]
struct CLI {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug)]
enum QueryError {
    NotFound,
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                QueryError::NotFound => "fatal: unable to find output.",
            }
        )
    }
}

impl std::error::Error for QueryError {}

fn handle_query(
    opts: &QueryCommandOpts,
    config: &display_config::DisplayConfig,
    proxy: &dbus::blocking::Proxy<&dbus::blocking::Connection>,
) -> Result<String, Box<QueryError>> {
    Ok(match &opts.connector {
        Some(connector) => {
            let physical_monitor = config
                .monitors
                .iter()
                .find(|monitor| monitor.connector == *connector)
                .ok_or(QueryError::NotFound)?;

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
                .ok_or(QueryError::NotFound)?;

            format!("{}\n{}", logical_monitor, physical_monitor)
        }
        None => format!("{}", config),
    })
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the CLI args. We do this first to short-circuit the dbus calls if there's an invalid arg.
    let args = CLI::from_args();

    // Open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Open a proxy to the Mutter DisplayConfig
    let proxy = conn.with_proxy(
        "org.gnome.Mutter.DisplayConfig",
        "/org/gnome/Mutter/DisplayConfig",
        Duration::from_millis(5000),
    );

    // Load the config from dbus using the proxy
    let config = display_config::DisplayConfig::get_current_state(&proxy)?;

    // See what we're executing
    let cmd = args
        .cmd
        .unwrap_or(Command::Query(QueryCommandOpts { connector: None }));

    print!(
        "{}",
        match cmd {
            Command::Query(opts) => handle_query(&opts, &config, &proxy)?,
        }
    );

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
    }
}
