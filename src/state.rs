use anyhow::Result;
use std::fmt;
use std::fs;

pub const TURBO_SETTING: &str = "/sys/devices/system/cpu/cpufreq/boost";
pub const FAN_PROFILE: &str = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";

pub struct State {
    fan_profile: u8,
    turbo: bool,
}

impl State {
    pub fn from_system() -> Result<State> {
        Ok(State {
            fan_profile: fs::read_to_string(FAN_PROFILE)?.trim().parse()?,
            turbo: (fs::read_to_string(TURBO_SETTING)?.trim().parse::<u8>()?) != 0,
        })
    }

    pub fn fan_profile(&self) -> u8 {
        self.fan_profile
    }

    pub fn fan_profile_str(&self) -> &str {
        match self.fan_profile() {
            0 => "normal",
            1 => "boost",
            2 => "silent",
            _ => unreachable!(),
        }
    }

    pub fn json_string(&self) -> String {
        // well safe enough (I hope)
        format!(
            "{{\"fan_profile\": \"{}\", \"turbo\": {}}}",
            self.fan_profile_str(),
            self.turbo
        )
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fan profile: {}, Turbo: {}",
            self.fan_profile_str(),
            self.turbo
        )
    }
}
