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
