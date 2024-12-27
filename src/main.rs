use clap::Parser;
use data::DataProject;
use env_logger::{Builder, Env, WriteStyle};
use ini::ini;
use log::{debug, error, info, log_enabled, Level, Log};

use std::{env::args, fmt::Debug, io::Write, path::Path, process::exit};

mod data;
mod renderables;
// ======================
// Launch Args
// ======================

#[derive(Debug)]
enum AppLaunchMode {
    DEFAULT,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Launch mode for the app. Default will run the executable as the user-facing windowed editor.
    #[arg(long, default_value = "1")]
    launch_mode: String, // String reference to enum.
}

impl Args {
    fn match_launch_mode_str(__arg_string: &String) -> AppLaunchMode {
        match __arg_string.as_str() {
            "default" | "0" | _ => AppLaunchMode::DEFAULT,
        }
    }
}

// ======================
// Config
// ======================

/// Config Struct bundling together launch arguments that describe the desired app behaviour.
struct AppConfig {
    launch_mode: AppLaunchMode,
}

// ======================
// Init
// ======================

// Initialize the app's global logger and filter. Currently uses default formatter.
fn init_logger() {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).default_format().init();
}

fn get_config_version(__ini_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if !Path::new(__ini_path).exists() {
        return Err(format!("Could not locate version file: {}", __ini_path).into());
    }
    let conf = ini!(__ini_path);
    let version = conf["meta"]["version"].clone();
    if version.is_none() {
        return Err(format!(
            "Could not find version attribute in file: \'{:?}\'",
            __ini_path
        )
        .into());
    }
    return Ok(version.unwrap());
}

fn main() {
    init_logger();
    let args = Args::parse();
    let config = AppConfig {
        launch_mode: Args::match_launch_mode_str(&args.launch_mode),
    };

    log::info!("Launching with mode: {:?}", config.launch_mode);

    let version_ini_filepath = "VERSION.ini";
    let software_version_fmt = match get_config_version(&version_ini_filepath) {
        Err(e) => {
            error!("Could not load version config: {:?}", e);
            exit(1);
        }
        Ok(v) => v,
    };
    println!("{:?}", software_version_fmt);
}
