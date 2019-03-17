# `fittle`

A library and set of tools for working with
[Flexible and Interoperable Data Transfer](https://www.thisisant.com/resources/fit) (FIT) files.

## Installation

1. Clone this repo

## Usage

_Disclaimer: this repo is still very much a WIP, so UX is less than desirable_

1. `cargo install --path .`
2. `fittle < run.fit`, which will produce JSON output, for example:

```json
[
  {
    "message_type": "file_id",
    "manufacturer": "garmin",
  },
  {
    "message_type": "file_creator",
    "hardware_version": 255,
    "software_version": 123
  },
  {
    "message_type": "event",
    "data": 0,
    "event": "timer",
    "event_group": 0,
    "event_type": "start",
    "timestamp": "2000-01-01T23:29:52Z"
  },
  "...",
  {
    "message_type": "record",
    "cadence": 84,
    "distance": 8694,
    "fractional_cadence": 64,
    "heart_rate": 124,
    "speed": 3322,
    "temperature": 25,
    "timestamp": "2000-01-01T23:30:20Z"
  },
  {
    "message_type": "record",
    "cadence": 84,
    "distance": 9681,
    "fractional_cadence": 64,
    "heart_rate": 127,
    "speed": 3266,
    "temperature": 25,
    "timestamp": "2000-01-01T23:30:23Z"
  },
  {
    "message_type": "activity",
    "event": "activity",
    "event_group": 255,
    "event_type": "stop",
    "local_timestamp": "2000-01-01T18:53:25-05:00",
    "num_sessions": 1,
    "timestamp": "2000-01-01T23:53:25Z",
    "total_timer_time": 1391070,
    "type_": "manual"
  }
]
```

It is recommended to use this project in tandem with [jq](https://stedolan.github.io/jq/). For
example, to get one's heart rate during a session:

```sh
cargo run < run.fit | jq -C '.[] | select(.message_type == "record") | .heart_rate'
```

## Generating profile code

The `fittle-profile-gen` binary works by generating code from a FIT profile spreadsheet. You can
find this at [www.thisisant.com](https://www.thisisant.com/developer/resources/downloads/). Once
downloaded and extracted, you can generate the profile code by running:

```sh
export FIT_PROFILE_PATH="/path/to/sdk/FitSDKRelease_20.80.00/Profile.xlsx"
cargo run -p fittle-profile-gen
```
