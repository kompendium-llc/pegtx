# PegTX

[![Crates.io](https://img.shields.io/crates/v/pegtx.svg)](https://crates.io/crates/pegtx)
[![Discord](https://img.shields.io/discord/550312670528798755.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/V6T7mCW)

PegTX is an accounting and monitoring tool for use with pegnet, given an address it will
generate a csv report that can be imported into common spreadsheet and accounting software.
By default it will search for all actions for that particular address, that includes
transactions, conversions and burns. These can be searched individually or in any combination.

## Installation

### Download

Precompiled binaries can be found on the [releases page](https://github.com/kompendium-llc/pegtx/releases).

### Cargo

```shell
cargo install pegtx
```

### Build from source
```shell
git clone 
cargo build --release
```

## Usage

Available commands can be found with `pegtx -h`

```shell
pegtx 0.1.0
An accounting tool for use with pegnet

USAGE:
    pegtx.exe [FLAGS] [OPTIONS] <address>

FLAGS:
    -b, --burn          The list of transactions will include transfers
    -c, --conversion    The list of transactions will include conversions
    -d, --descending    Retrieve newest transactions first
    -h, --help          Prints help information
    -m, --mining        The list of transactions will include miner payouts
    -t, --transfer      The list of transactions will include transfers
    -V, --version       Prints version information

OPTIONS:
    -n, --node <node>        Sets a custom pegnetd node rather than using the public node
    -o, --output <output>    Specify output file, defaults to <ADDRESS>.csv

ARGS:
    <address>    FCT address
```

## Contributing
PR's welcome. Fork the library and submit to dev branch. 
By contributing you agree to it being Apache 2.0 licensed