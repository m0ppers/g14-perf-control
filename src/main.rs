mod control;
mod state;

use anyhow::{Context, Result};
use std::error::Error;
use structopt::StructOpt;

use state::State;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "g14-perf-control",
    about = "A small tool to control performance settings on your Asus G14"
)]
struct G14PerfControl {
    #[structopt(short, default_value = "/etc/g14-perf-control.toml")]
    configfile: String,
    #[structopt(subcommand)]
    command: G14PerfControlCommand,
}

#[derive(Debug, StructOpt)]
#[structopt()]
enum G14PerfControlCommand {
    #[structopt(about = "Toggle profile")]
    Toggle,
    #[structopt(about = "Print current profile")]
    Info {
        #[structopt(long)]
        json: bool,
    },
    #[structopt(about = "Set to a specific profile")]
    Set { profile_name: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = G14PerfControl::from_args();

    let state =
        State::from_system().with_context(|| "Couldn't determine state. Are you using a G14?")?;

    match &opt.command {
        G14PerfControlCommand::Toggle => {
            let control = control::Control::new(&state, &opt.configfile)
                .with_context(|| format!("Failed to read configfile `{}`", &opt.configfile))?;
            control
                .toggle_profile()
                .with_context(|| format!("Couldn't change profile"))?;
        }
        G14PerfControlCommand::Set { profile_name } => {
            let control = control::Control::new(&state, &opt.configfile)
                .with_context(|| format!("Failed to read configfile `{}`", &opt.configfile))?;
            control
                .set(&profile_name)
                .with_context(|| format!("Couldn't change profile"))?;
        }
        G14PerfControlCommand::Info { json } => {
            if *json {
                println!("{}", state.json_string());
            } else {
                println!("{}", state);
            }
        }
    }

    Ok(())
}
