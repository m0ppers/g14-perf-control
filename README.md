# g14-perf-control

Small utility to set performance related settings on your Asus G14.

Alternative to https://gitlab.com/asus-linux/asus-nb-ctrl

This utility only does the basic stuff. Just setting fan profiles and boost settings.

asus-nb-ctrl does A LOT more (dbus, notify, gpu settings etc.) but needs to run as a daemon
and this utility just needs to run on demand.

```
g14-perf-control 0.1.0
A small tool to control performance settings on your Asus G14

USAGE:
    g14-perf-control [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <configfile>         [default: /etc/g14-perf-control.toml]

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    info      Print current profile
    set       Set to a specific profile
    toggle    Toggle profile
```

## Configuration

An example file is being distributed. All settings are optional. If you don't set a setting
`g14-perf-control` will not change that setting.

If you leave out fan curves the tool will still set the fan profile but leave
curves as they are (meaning the G14 firmware does whatever it does for that profile).

The profiles are fixed so you can't add custom profiles and have to work with
`normal`, `boost` and `silent` because the kernel module supports exactly these.

`g14-perf-control` needs write access to:

```
/sys/devices/system/cpu/cpufreq/boost
/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy
```

Either you have some magic udev rules or you likely need to `sudo` when executing `g14-perf-control`.

Information about configuring fan curves can be found here:

https://docs.rs/rog_fan_curve/0.1.7/rog_fan_curve/#config-string-format

### Sway integration

.config/sway/config

```
bindsym XF86Launch4 exec sudo g14-perf-control toggle
```

(needs passwordless sudo for that command. See sudo)

### Waybar integration

Waybar allows updating widgets via signals. By setting `waybar_notify` you can integrate with waybar
and `g14-perf-control` will trigger updates to your widget.

Example setup:

.config/waybar/config

```
[...]
    "modules-right": ["idle_inhibitor", "custom/asus-laptop", "pulseaudio", "network", "network#wifi", "cpu", "memory", "temperature", "battery", "clock", "tray"],
[...]
    "custom/asus-laptop": {
        "format": " {}",
        "interval": "once",
        "signal": 1,
        "exec": "g14-perf-control info"
    },
[...]
```

/etc/g14-perf-control.toml

```
waybar_notify = 1
```

Note that signal and waybar_notify have to match.