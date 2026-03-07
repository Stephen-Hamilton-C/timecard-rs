# timecard-rs

My 5th rewrite of this program because I'm utterly deranged

## Install

Install the terminal application with Cargo:
```bash
cargo install timecard-cli
```

Add the library to your project with Cargo:
```bash
cargo add timecard
```

## Duration Format

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
