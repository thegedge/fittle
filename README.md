# `fittle`

A library and set of tools for working with
[Flexible and Interoperable Data Transfer](https://www.thisisant.com/resources/fit) (FIT) files.

## Installation

1. Clone this repo

## Usage

_Disclaimer: this repo is still very much a WIP, so UX is less than desirable_

2. `cargo install --path .`
2. `fittle < run.fit`, which will produce JSON output

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
