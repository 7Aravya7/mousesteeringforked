use device_query::Keycode;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error, str::FromStr};

#[derive(Debug)]
pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,

    pub steering_config: ControlConfig,
    pub throttle_config: ControlConfig,
}

#[derive(Debug)]
pub struct ControlConfig {
    pub enabled: bool,
    pub toggle_key: Keycode,
    pub precise_input: bool,
    pub snap_input: bool,
    pub snap_threshold: f32,
    pub edge_scaling: bool,
    pub scaling_threshold: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigToml {
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,

    pub steering_config: Option<ControlConfigToml>,
    pub throttle_config: Option<ControlConfigToml>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlConfigToml {
    pub enabled: Option<bool>,
    pub toggle_key: Option<String>,
    pub precise_input: Option<bool>,
    pub snap_input: Option<bool>,
    pub snap_threshold: Option<f32>,
    pub edge_scaling: Option<bool>,
    pub scaling_threshold: Option<f32>
}

impl Config {
    pub fn new() -> Self {
        let filepaths: [&str; 2] = ["./config.toml", "./Config.toml"];

        let mut content: String = String::new();
        for filepath in filepaths {
            let result: Result<String, Error> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let raw_config: ConfigToml =
            toml::from_str(&content).unwrap_or_else(|_| ConfigToml::get_defaults());

        Config::to_config(raw_config)
    }

    fn to_config(config: ConfigToml) -> Self {
        let default = ConfigToml::get_defaults();

        let steering_config = ControlConfig::to_control_config(
            config
                .steering_config
                .unwrap_or(ControlConfigToml::get_defaults()),
        );

        let throttle_config = ControlConfig::to_control_config(
            config
                .throttle_config
                .unwrap_or(ControlConfigToml::get_defaults()),
        );

        Config {
            screen_width: config.screen_width.unwrap_or(default.screen_width.unwrap()),
            screen_height: config
                .screen_height
                .unwrap_or(default.screen_height.unwrap()),

            steering_config,
            throttle_config,
        }
    }
}

impl ControlConfig {
    fn to_control_config(config: ControlConfigToml) -> Self {
        let default = ControlConfigToml::get_defaults();

        let toggle_string = config.toggle_key.unwrap_or(default.toggle_key.unwrap());
        let toggle_key = Keycode::from_str(toggle_string.as_str()).unwrap_or(Keycode::X);

        ControlConfig {
            enabled: config.enabled.unwrap_or(default.enabled.unwrap()),
            precise_input: config
                .precise_input
                .unwrap_or(default.precise_input.unwrap()),
            snap_input: config.snap_input.unwrap_or(default.snap_input.unwrap()),
            snap_threshold: config
                .snap_threshold
                .unwrap_or(default.snap_threshold.unwrap()),
            edge_scaling: config.edge_scaling.unwrap_or(default.edge_scaling.unwrap()),
            scaling_threshold: config
                .scaling_threshold
                .unwrap_or(default.scaling_threshold.unwrap()),
            toggle_key
        }
    }
}

impl ConfigToml {
    fn get_defaults() -> Self {
        ConfigToml {
            screen_width: Some(1280),
            screen_height: Some(720),

            steering_config: Some(ControlConfigToml::get_defaults()),

            throttle_config: Some(ControlConfigToml::get_defaults()),
        }
    }
}

impl ControlConfigToml {
    fn get_defaults() -> ControlConfigToml {
        ControlConfigToml {
            enabled: Some(true),
            precise_input: Some(true),
            snap_input: Some(true),
            snap_threshold: Some(0.1),
            edge_scaling: Some(false),
            scaling_threshold: Some(0.5),
            toggle_key: Some("X".to_string()),
        }
    }
}
