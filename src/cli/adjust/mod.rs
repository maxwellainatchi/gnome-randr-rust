use gnome_randr::display_config::{proxied_methods::GammaInfo, resources::Resources};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct ActionOptions {
    #[structopt(long, help = "Set the brightness by adjusting the gamma")]
    pub brightness: Option<f64>,
}

#[derive(StructOpt)]
pub struct CommandOptions {
    #[structopt(
        help = "the connector used for the physical monitor.",
        long_help = "the connector used for the physical monitor you want to modify, e.g. \"HDMI-1\". You can find these with \"query\" (no arguments) if you're unsure."
    )]
    pub connector: String,

    #[structopt(flatten)]
    pub actions: ActionOptions,

    #[structopt(long, help = "List changes without actually applying them")]
    dry_run: bool,
}

pub fn handle(
    opts: &CommandOptions,
    resources: &Resources,
    proxy: &dbus::blocking::Proxy<&dbus::blocking::Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(brightness) = opts.actions.brightness {
        let output = resources
            .outputs
            .iter()
            .find(|output| output.name == opts.connector)
            .unwrap();
        let crtc = resources
            .crtcs
            .iter()
            .find(|crtc| crtc.id == output.current_crtc as u32)
            .unwrap();

        let crtc_gamma = resources.get_crtc_gamma(&proxy, crtc)?;
        let crtc_gamma_info = crtc_gamma.gamma_info();

        let new_gamma_info = GammaInfo {
            brightness,
            ..crtc_gamma_info
        };
        let final_gamma = new_gamma_info.get_gamma(crtc_gamma.red.len());

        if !opts.dry_run {
            resources.set_crtc_gamma(&proxy, crtc, final_gamma)?;
        }
    }

    if opts.dry_run {
        println!("dry run: no changes made.");
        return Ok(());
    }

    Ok(())
}
