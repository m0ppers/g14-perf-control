// #[macro_use]
// extern crate num_derive;

use crate::state::*;
use anyhow::Result;
use procfs;
use rog_fan_curve::{Board, Curve, Fan};
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;
use std::fs;

// hmmm...sigrtmin seems to be unsupported as of now and the nix kill() thing only allows known signals :S
extern "C" {
    fn __libc_current_sigrtmin() -> i64;
    fn kill(pid: i64, signal: i64);
}

#[derive(Debug)]
enum ControlError {
    InvalidBoard,
    UnknownProfile(String),
}

impl StdError for ControlError {}

impl fmt::Display for ControlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ControlError::InvalidBoard => write!(f, "Invalid Board. Are you using a G14?"),
            ControlError::UnknownProfile(s) => write!(f, "Unknown Profile {}", s),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ProfileConfig {
    turbo: Option<bool>,
    cpu_curve: Option<Curve>,
    gpu_curve: Option<Curve>,
}

#[derive(Debug, Deserialize, Default)]
struct Profiles {
    normal: Option<ProfileConfig>,
    boost: Option<ProfileConfig>,
    silent: Option<ProfileConfig>,
}

#[derive(Debug, Deserialize)]
struct Config {
    waybar_notify: Option<u8>,
    #[serde(default)]
    profiles: Profiles,
}

const NUM_PROFILES: u8 = 3;

pub struct Control<'a> {
    config: Config,
    state: &'a State,
}

impl<'a> Control<'a> {
    pub fn new(state: &'a State, configfile: &str) -> Result<Control<'a>> {
        let config_slice = fs::read(configfile)?;
        let config: Config = toml::from_slice(&config_slice)?;
        Ok(Control { config, state })
    }

    fn set_profile(&self, fan_profile: u8, config: Option<&ProfileConfig>) -> Result<()> {
        let board = match Board::from_board_name() {
            Some(b) => b,
            None => return Err(ControlError::InvalidBoard.into()),
        };

        fs::write(FAN_PROFILE, fan_profile.to_string())?;
        if let Some(new_profile) = config {
            if let Some(turbo) = new_profile.turbo {
                fs::write(TURBO_SETTING, (turbo as u8).to_string())?;
            }
            if let Some(cpu_curve) = &new_profile.cpu_curve {
                cpu_curve.apply(board, Fan::Cpu)?;
            }
            if let Some(gpu_curve) = &new_profile.gpu_curve {
                gpu_curve.apply(board, Fan::Gpu)?;
            }
        }
        if let Some(waybar_notify) = self.config.waybar_notify {
            let waybar_pid = procfs::process::all_processes()?
                .iter()
                .find_map(|process| match process.stat() {
                    Ok(stat) => match stat.comm.as_str() {
                        "waybar" => Some(stat.pid),
                        _ => None,
                    },
                    Err(_) => None,
                });
            if let Some(waybar_pid) = waybar_pid {
                // hmmm didn't find something useful for this :S
                unsafe {
                    let signal = __libc_current_sigrtmin() + waybar_notify as i64;
                    kill(waybar_pid as i64, signal);
                }
            }
        }

        Ok(())
    }

    pub fn set(&self, profile_name: &str) -> Result<()> {
        let new_profile = match profile_name {
            "normal" => (0, self.config.profiles.normal.as_ref()),
            "boost" => (1, self.config.profiles.boost.as_ref()),
            "silent" => (2, self.config.profiles.silent.as_ref()),
            _ => return Err(ControlError::UnknownProfile(profile_name.to_owned()).into()),
        };

        self.set_profile(new_profile.0, new_profile.1)
    }

    pub fn toggle_profile(&self) -> Result<()> {
        let next = (self.state.fan_profile() + 1) % NUM_PROFILES;

        let new_profile = match next {
            0 => (0, self.config.profiles.normal.as_ref()),
            1 => (1, self.config.profiles.boost.as_ref()),
            2 => (2, self.config.profiles.silent.as_ref()),
            _ => unreachable!(),
        };
        self.set_profile(new_profile.0, new_profile.1)
    }
}
