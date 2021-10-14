use dbus;
use dbus::blocking::Connection;
use std::time::Duration;
use structopt::{self, StructOpt};

mod display_config;

#[derive(StructOpt)]
enum Command {
    Query,
}

#[derive(StructOpt)]
struct CLI {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy(
        "org.gnome.Mutter.DisplayConfig",
        "/org/gnome/Mutter/DisplayConfig",
        Duration::from_millis(5000),
    );

    let config = display_config::DisplayConfig::get_current_state(&proxy)?;

    let args = CLI::from_args();

    let cmd = args.cmd.unwrap_or(Command::Query);

    match cmd {
        Command::Query => println!("{}", config),
    }

    Ok(())
}
