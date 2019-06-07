![](./img/logo.png)

# Overview

**NOTE: As MakeShift is still in early development, all commits should be considered Breaking Changes and all APIs are subject to change without notice.**

MakeShift is an application for generating shift schedules within complex parameters. It is built for retail environments, and exhaustively considers possible configurations in order to optimize employee flexibility and satisfaction. MakeShift is written in [Rust](https://www.rust-lang.org/) and will eventually use [Gtk](https://www.gtk.org/) bindings for its GUI.

# Installation

## Requirements
* Rust stable v1.35.0 or higher

To run MakeShift, simply clone this repository with `git clone`, navigate to the newly created `./makeshift` and use `cargo run`.

As the current version uses a CLI, you will have to provide your own `docs` folder containing `events.csv` and `roster.csv` in order for the program to properly load employee data. See **Import Formats** under Documentation below for more information on importing.

# Documentation

The most up-to-date documentation can always be found by running `cargo doc --open` in the crate's main folder. This is currently the only form of documentation for the project.

## Import Formats

The import system uses CSV (comma separated values) files containing employee and event information. The required headers and contents for each are listed below.

### Events

| Header      | Format      | Description                                   |
| ----------- | ----------- | --------------------------------------------- |
| name        | string      | Event name                                    |
| ev_type     | string      | Event type (must match empl 'abilities')      |
| day         | String      | Capitalized weekday name                      |
| start       | HH:MM       | Start time                                    |
| end         | HH:MM       | End time                                      |
| setup       | HH:MM       | Setup time requirement                        |
| cleanup     | HH:MM       | Cleanup time requirement                      |
| staff_req   | int         | Total # of required staff                     |
| fixed_emps  | string, ... | Comma-separated employees who must work this  |

### Roster

| Header         | Format      | Description                          |
| -------------- | ----------- | ------------------------------------ |
| id             | string      | Employee's name or ID                |
| cant_work_days | String, ... | Comma-separated unavailable weekdays |
| min_hours      | int         | Minimum required hours per week      |
| max_hours      | int         | Maximum required hours per week      |
| role           | string      | Manager, Associate, etc.             |
| abilities...   | int         | see below

All further columns after role are treated as a relative evaluation of the employee's competence at running an event of type \[column_header\].

# Contributing

This project follows the [AngularJS Git Commit Message Conventions](https://gist.github.com/stephenparish/9941e89d80e2bc58a153#format-of-the-commit-message) and the [Contributor Covenant Code of Conduct](./CODE_OF_CONDUCT.md). We also respect and abide by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). More thorough contribution guidelines will be posted following the first project release.

MakeShift is licensed under the Gnu Public License (GPL) version 3.0. For more information see [LICENSE](./LICENSE.txt) or the [Gnu Website](https://www.gnu.org/licenses/).

Copyright (C) 2019 Matt Langhinrichs.