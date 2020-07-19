## Countdown

The program sleeps for a specified time, and send a system notification.

## Usage

```
Send a system-wide notification after a countdown

USAGE:
    countdown [FLAGS] <time> [text]

ARGS:
    <time>    A float number for the countdown, unit being second, unless --minute is specified.
    <text>    A short message to show in the notification. [default: Timer]

FLAGS:
    -h, --help       Prints help information
    -m, --minute     Change the unit of time to minute, instead of second.
    -V, --version    Prints version information
```
