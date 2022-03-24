use std::time::Duration;

use dbus::blocking::Connection;
use structopt::StructOpt;

use gnome_randr::{display_config::resources::Resources, DisplayConfig};

pub mod adjust;
pub mod modify;
pub mod query;

#[derive(StructOpt)]
enum Command {
    #[structopt(
        about = "Query returns information about the current state of the monitors. This is the default subcommand."
    )]
    Query(query::CommandOptions),
    #[structopt(about = "Modify allows you to alter the current display configuration.")]
    Modify(modify::CommandOptions),
    Adjust(adjust::CommandOptions),
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
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
    let config = DisplayConfig::get_current_state(&proxy)?;

    // See what we're executing
    let cmd = args.cmd.unwrap_or(Command::Query(query::CommandOptions {
        connector: None,
        summary: false,
    }));

    match cmd {
        Command::Query(opts) => print!("{}", query::handle(&opts, &config)?),
        Command::Modify(opts) => modify::handle(&opts, &config, &proxy)?,
        Command::Adjust(opts) => {
            let resources = Resources::get_resources(&proxy).unwrap();

            adjust::handle(&opts, &resources, &proxy)?;
        }
    }

    Ok(())
}
