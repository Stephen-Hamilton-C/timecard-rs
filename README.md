# timecard-rs

This is a program I make whenever I want to learn the intricacies of a language.
I've done this before in Python, Kotlin/JVM, Kotlin/Native, and Kotlin/JS, now it's in Rust.
However, it is also meant to be useful to myself, so maybe you can find some use in it, too.

`timecard` is a library that makes it easy to build applications that keep track of time,
such as `timecard-cli`.

The `timecard` library documentation can be found [here][timecard-docs]

- [timecard-rs](#timecard-rs)
  - [Install](#install)
  - [Usage](#usage)
    - [Clocking In \& Out](#clocking-in--out)
    - [Undo](#undo)
    - [Log](#log)
    - [Notifications](#notifications)
  - [Configuration](#configuration)
    - [Duration Format](#duration-format)


## Install

Install the terminal application with Cargo:
```bash
$ cargo install timecard-cli
$ timecard
No log for today
```

Add the library to your project with Cargo:
```bash
$ cargo add timecard
```

Ensure Cargo binaries are in your path by adding this to your `~/.bashrc`:
```bash
export PATH="$PATH:$HOME/.cargo/bin"
```
Then restart your terminal or run `source ~/.bashrc`.


## Usage

### Clocking In & Out

You start time entries by clocking in, and finish them by clocking out.
You can only clock in/out at the current time, or at a time in the past.
```bash
$ timecard in 08:35
Clocked IN at 08:35 AM
$ timecard out 1hr  # current time is 11:15
Clocked OUT at 10:15 AM
```
A variety of time formats are accepted, including, but not limited to:
* An integer, representing minutes in the past.
  (e.g. timecard out 5 will clock out 5 minutes ago)
* An [RFC 3339][rfc-3339] datetime that is in the past, such as `2026-03-08T11:15`
* An intuitive [humantime][humantime-duration] string representing a duration in the past, such as `1hr7min` or `10minutes`.
* A human-readable time that is earlier today, e.g. `15:37`, `3:37:53 PM`, `3PM`
* A human-readable datetime that is in the past, e.g. `20260308T3PM`

### Undo

You can remove the last time log by running the undo command:
```bash
$ timecard undo
Successfully removed last entry.
Entries for 2026-03-08:
IN: 08:35 AM
```


### Log

You can see a log of all your time entries for the day:
```bash
$ timecard log
Entries for today:
IN: 08:35 AM    OUT: 11:15 AM
```

You can see other days by providing a date, similar to the formats used for clocking in or out:
```bash
# Assuming today is 2026-03-08
$ timecard log 1
Entries for 2026-03-07:
IN: 08:40 AM    OUT: 12:07 PM
IN: 01:02 PM    OUT: 04:59 PM
$ timecard log 2days
No entries exist for 2026-03-06
$ timecard log 2026-03-01
Entries for 2026-03-01:
IN: 08:27 AM    OUT: 12:31 PM
IN: 01:37 PM    OUT: 05:02 PM
```

You can also see logs for a date range by providing the `--from-date` and `--to-date` options:
```bash
# Assuming today is 2026-03-08
$ timecard log --from-date 2026-03-07
Entries from 2026-03-07 to today:
IN: 2026-03-07 08:40 AM    OUT: 2026-03-07 12:07 PM
IN: 2026-03-07 01:02 PM    OUT: 2026-03-07 04:59 PM
IN: 2026-03-08 08:35 AM    OUT: 2026-03-08 11:15 AM
$ timecard log --to-date 2026-03-02
Entries from forever ago to 2026-03-02:
IN: 2026-03-01 08:27 AM    OUT: 2026-03-01 12:31 PM
IN: 2026-03-01 01:37 PM    OUT: 2026-03-01 05:02 PM
$ timecard log --from-date 2026-03-01 --to-date 2026-03-07
Entries from 2026-03-01 to 2026-03-07:
IN: 2026-03-01 08:27 AM    OUT: 2026-03-01 12:31 PM
IN: 2026-03-01 01:37 PM    OUT: 2026-03-01 05:02 PM
IN: 2026-03-07 08:40 AM    OUT: 2026-03-07 12:07 PM
IN: 2026-03-07 01:02 PM    OUT: 2026-03-07 04:59 PM
```
You can even combine these two methods to get the log for the date range provided,
as well for a single day's log.

Using the `--all` flag will ignore everything and just spit all of your time entries
into the console.

You can also use the `--csv` flag to output in CSV format.
You can then pipe the output to a file to save it.
```bash
$ timecard log --csv > log.csv
$ cat log.csv
Start,End
2026-03-08 08:35 AM,2026-03-08 11:15 AM
```

If you pass an argument to the `--csv` flag, it will be used as the column separator:
```bash
$ timecard log --csv ";"
Start;End
2026-03-08 08:35 AM;2026-03-08 11:15 AM
```


### Notifications

You can get a desktop notification when you reach your expected end time
by running `timecard notify`.
This command is intended to be periodically run by a service.
If you're on Linux, you can run `timecard notify --install` to setup a systemd
service + timer unit, which will run that command approximately every minute.
This command only sends a notification once per day,
and only sends it if the current time meets or exceeds the expected end time.


## Configuration

The config file location depends on what OS you're on:
| OS      | Location |
| ------- | -------- |
| Windows | `%APPDATA%\timecard\timecard-cli.toml` |
| macOS   | `$HOME/Library/Application Support/timecard/timecard-cli.toml` |
| Linux/UNIX | `$XDG_CONFIG_HOME/timecard/timecard-cli.toml` OR `$HOME/.config/timecard/timecard-cli.toml` |

The default file that is generated is self-explanatory.
If you don't have any comments explaining how the config works,
either delete or rename the file, then just run `timecard`.
The newly generated file will contain comments explaining each field.


### Duration Format

The `duration_fmt` config field uses a custom format defined below:
| Specifier | Example |               Description                |
| --------- | ------- | ---------------------------------------- |
| `%ht`     | `2.8`   | Hours, rounded to the nearest tenth      |
| `%hq`     | `2.75`  | Hours, rounded to the nearest quarter    |
| `%hh`     | `2.5`   | Hours, rounded to the nearest half       |
| `%HR`     | `2`     | Hours, rounded to the nearest whole hour |
| `%HH`     | `2`     | Whole hours                              |
| `%MM`     | `30`    | Whole minutes                            |
| `>>H`     | `02`    | Whole hours, padded with zeros           |
| `>>M`     | `09`    | Whole minutes, padded with zeros         |


<!-- links -->
[rfc-3339]: https://www.rfc-editor.org/rfc/rfc3339
[humantime-duration]: https://docs.rs/humantime/latest/humantime/fn.parse_duration.html
[timecard-docs]: https://docs.rs/timecard/latest/timecard/
