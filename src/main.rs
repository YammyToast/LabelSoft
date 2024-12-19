use clap::Parser;
use data::DataProject;
use env_logger::{Builder, Env, WriteStyle};
use log::{debug, error, info, log_enabled, Level, Log};
use std::{env::args, fmt::Debug, io::Write, process::exit};

mod data;
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
fn main() {
    init_logger();
    let args = Args::parse();
    let config = AppConfig {
        launch_mode: Args::match_launch_mode_str(&args.launch_mode),
    };

    log::info!("Launching with mode: {:?}", config.launch_mode);
    let test: DataProject = match DataProject::new_infer_schema("tests/assets/good.csv") {
        None => {
            error!("Could not create Data project");
            exit(1)
        }
        Some(v) => v,
    };
    info!("{:?}", test.schema);
}
