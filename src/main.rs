#[macro_use]
extern crate clap;

use clap::Clap;
use notify_rust::error::Result;
use notify_rust::Notification;
use std::time::Duration;

#[derive(Clap)]
#[clap(
    version = "1.0.2",
    author = "PowerSnail <hj@powersnail.com>",
    about = "Send a system-wide notification after a countdown"
)]
struct Opts {
    #[clap(
        about = "A float number for the countdown, unit being second, unless --minute is specified."
    )]
    time: f32,

    #[clap(
        default_value = "Timer",
        about = "A short message to show in the notification."
    )]
    text: String,

    #[clap(
        short,
        long,
        about = "Change the unit of time to minute, instead of second."
    )]
    minute: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let time_unit = match opts.minute {
        true => 60.0,
        false => 1.0,
    };
    std::thread::sleep(Duration::from_secs_f32(opts.time * time_unit));
    Notification::new()
        .summary("Time's up")
        .body(&opts.text)
        .icon("clock")
        .show()?;
    Ok(())
}
