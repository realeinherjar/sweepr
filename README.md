# sweepr

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/realeinherjar/sweepr/branch/main/graph/badge.svg?token=7NRUC51LLY)](https://codecov.io/gh/realeinherjar/sweepr)

A sweepr CLI for the hodlr that just wants to sweep the funds from a seed to an address.

## Usage

By default sweepr will generate segwit v0 descriptors.
In the future, when taproot is more widely adopted, this will be the default.

```bash
$ sweepr --help
A sweepr CLI for the hodlr that just wants to sweep the funds from a seed to an address

Usage: sweepr [OPTIONS] <SEED> <ADDRESS>

Arguments:
  <SEED>     Seed to sweep funds from
  <ADDRESS>  Address to withdraw to

Options:
  -n, --network <NETWORK>  Network to use [default: mainnet]
  -u, --url <URL>          Esplora server to use [default: https://mempool.space/api]
  -h, --help               Print help
  -V, --version            Print version
```

## Example

```bash
# Mainnet and a bech32 address
sweepr "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon cactus" bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq

# Testnet and a legacy address
sweepr -n testnet -u "https://mempool.space/testnet/api" "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon cactus" mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn

# Regtest and a legacy address
sweepr -n regtest -u "http://localhost:3000/api" "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon cactus" mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn
```
