use clap::Parser;
use notify_rust::error::Result;
use notify_rust::Notification;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Opts {
    /// A float number for the countdown, unit being second, unless --minute is specified.
    time: f32,

    /// A short message to show in the notification.
    #[clap(default_value = "Timer")]
    text: String,

    /// Change the unit of time to minute, instead of second.
    #[clap(short, long)]
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
        .appname("Snail Countdown")
        .summary("Time's up")
        .body(&opts.text)
        .icon("clock")
        .show()?;
    Ok(())
}
