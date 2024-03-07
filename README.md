<!-- markdownlint-disable MD014 -->

<div align="center">

  <h1><code>cargo-unc</code></h1>

  <p>
    <strong>Cargo extension for building <a href="https://github.com/unc/unc-sdk-rs">unc-sdk-rs</a> smart contracts and <a href="https://github.com/unc/abi">ABI schemas</a> on UNC</strong>
  </p>

  <p>
    <a href="https://github.com/unc/cargo-unc/actions/workflows/test.yml?query=branch%3Amain"><img src="https://github.com/unc/cargo-unc/actions/workflows/test.yml/badge.svg" alt="Github CI Build" /></a>
    <a href="https://crates.io/crates/cargo-unc"><img src="https://img.shields.io/crates/v/cargo-unc.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/cargo-unc"><img src="https://img.shields.io/crates/d/cargo-unc.svg?style=flat-square" alt="Download" /></a>
  </p>

</div>

## Release notes

**Release notes and unreleased changes can be found in the [CHANGELOG](CHANGELOG.md)**

## Installation

<details>
  <summary>Install prebuilt binaries via shell script (Linux, macOS)</summary>

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/unc/cargo-unc/releases/latest/download/cargo-unc-installer.sh | sh
```
</details>

<details>
  <summary>Install prebuilt binaries via powershell script (Windows)</summary>

```sh
irm https://github.com/unc/cargo-unc/releases/latest/download/cargo-unc-installer.ps1 | iex
```
</details>

<details>
  <summary>Install prebuilt binaries into your Node.js application</summary>

```sh
npm install cargo-unc
```
</details>

<details>
  <summary>Compile and install from source code (Cargo)</summary>

```sh
cargo install cargo-unc
```

or, install the most recent version from git repository:

```sh
$ git clone https://github.com/unc/cargo-unc
$ cargo install --path cargo-unc
```
</details>

## Usage

See `cargo unc --help` for a complete list of available commands or run `cargo unc` to dive into interactive mode. Help is also available for each individual command with a `--help` flag, e.g. `cargo unc build --help`.

```console
cargo unc
```

Starts interactive mode that will allow to explore all the available commands.

```console
cargo unc build
```

Builds a UNC smart contract along with its [ABI](https://github.com/unc/abi) (while in the directory containing contract's Cargo.toml).

You can also make this command embed ABI into your WASM artifact by adding `--embed-abi` parameter. Once deployed, this will allow you to call a view function `__contract_abi` to retrieve a [ZST](https://facebook.github.io/zstd/)-compressed ABI.

```console
cargo unc abi
```

Generates UNC smart contract's [ABI](https://github.com/unc/abi) (while in the directory containing contract's Cargo.toml).

```console
cargo unc create-dev-account
```

Guides you through creation of a new UNC account on [testnet](https://explorer.testnet.unc.org).

```console
cargo unc deploy
```

Builds the smart contract (equivalent to `cargo unc build`) and guides you to deploy it to the blockchain.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
