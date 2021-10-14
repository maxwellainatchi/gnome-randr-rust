use dbus;
use dbus::blocking::Connection;
use std::time::Duration;
use structopt::{self, StructOpt};

mod display_config;

#[derive(StructOpt)]
enum Command {
    #[structopt(
        about = "Query returns information about the current state of the monitors. This is the default subcommand."
    )]
    Query,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let cmd = args.cmd.unwrap_or(Command::Query);

    match cmd {
        Command::Query => println!("{}", config),
    }

    Ok(())
}
