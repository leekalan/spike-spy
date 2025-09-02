use std::process;

use clap::Parser;

use crate::main_loop::{Extras, FrameThresholdType, SystemDetails, main_loop};

mod frame;
mod main_loop;
mod snapshot;
mod system;
mod util;

// Config
const POLL_INTERVAL: u64 = 1000; // milliseconds
const FRAME_UPDATE_INTERVAL: u64 = 5000; // milliseconds
const CPU_SPIKE_THRESHOLD: f32 = 10.0; // percent

struct Error {
    msg: String,
}

#[derive(Parser, Debug)]
#[command(name = "spike-spy", version, about = "Monitors for CPU spikes")]
pub struct Cli {
    /// Poll interval in milliseconds (>=0)
    #[arg(default_value_t = POLL_INTERVAL)]
    poll_interval: u64,

    /// CPU spike threshold (0-100%)
    #[arg(default_value_t = CPU_SPIKE_THRESHOLD)]
    spike_threshold: f32,

    /// Update interval in milliseconds (>=0)
    #[arg(default_value_t = FRAME_UPDATE_INTERVAL)]
    update_interval: u64,

    /// No update interval
    #[arg(short, long)]
    no_update: bool,

    /// Suppress update logging
    #[arg(short, long)]
    suppress_update: bool,

    /// Update displays all
    #[arg(short, long)]
    all: bool,

    /// Update displays over threshold (0-100%)
    #[arg(short, long)]
    over: Option<f32>,

    /// Update displays top N (>0)
    #[arg(short, long)]
    top: Option<usize>,
}

fn process_args() -> Result<(SystemDetails, Extras), Error> {
    let cli = Cli::parse();

    let poll_interval = cli.poll_interval;
    if !(0.0..=100.0).contains(&cli.spike_threshold) {
        return Err(Error {
            msg: "spike threshold must be between 0 and 100 percent".to_string(),
        });
    }
    let cpu_spike_threshold = cli.spike_threshold;
    let frame_update_interval = if cli.no_update {
        None
    } else {
        Some(cli.update_interval)
    };

    let suppress_frame_update_message = cli.suppress_update;
    let frame_threshold_type = match (cli.all, cli.over, cli.top) {
        (_, None, None) => FrameThresholdType::All,
        (false, Some(threshold), None) => {
            if !(0.0..=100.0).contains(&threshold) {
                return Err(Error {
                    msg: "over threshold must be between 0 and 100 percent".to_string(),
                });
            }
            FrameThresholdType::OverThreshold(threshold)
        }
        (false, None, Some(n)) => {
            if n == 0 {
                return Err(Error {
                    msg: "top N must be greater than 0".to_string(),
                });
            }
            FrameThresholdType::TopN(n)
        }
        _ => {
            return Err(Error {
                msg: "expected at most one of --all, --over, or --top".to_string(),
            });
        }
    };

    let details = SystemDetails {
        poll_interval,
        cpu_spike_threshold,
        frame_update_interval,
    };

    let extras = Extras {
        suppress_frame_update_message,
        frame_threshold_type,
    };

    Ok((details, extras))
}

fn main() {
    let (details, extras) = match process_args() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("error: {}", err.msg);
            process::exit(1)
        }
    };

    main_loop(details, extras);
}
