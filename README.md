<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# revm-sgx
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE) [![Automata SGX SDK](https://img.shields.io/badge/Power%20By-Automata%20SGX%20SDK-orange.svg)](https://github.com/automata-network/automata-sgx-sdk)

This encalve app is the SGX version of the Rust EVM executor or short REVME. The official standard version can be found [here](https://github.com/bluealloy/revm/tree/main/bins/revme). Presently, its primary application is in executing Ethereum tests.

## Prerequisite
* SGX-SDK 2.24 installation
* Cargo and Rust installation, currently requires `nightly-2024-02-01` rustc version
* Basic dev toolkits installation like cmake
* For setup the environment, refers to [Dockerfile](https://github.com/automata-network/automata-sgx-sdk/blob/main/docker/ubuntu-20.04.Dockerfile)

If you're using Azure, the following are recommended configurations:
* VM spec: `Standard_DC4s_v3`
* OS spec: `Ubuntu Server 20.04 LTS` or `Ubuntu Server 22.04 LTS`



## Getting Started

The encalve app takes a path to the directory where ethereum statetest json can be found. It recursively parses all json files in the specified directory and executes them.

Running all [ethereum tests](https://github.com/ethereum/tests) checks that revm is compliant to the ethereum specs.

To run [ethereum tests](https://github.com/ethereum/tests) locally, clone the [tests](https://github.com/ethereum/tests) repository and provide the test directory. Below, we clone the repo and execute the GeneralStateTests suite of tests.

```shell
$ git clone https://github.com/ethereum/tests.git
$ git clone https://github.com/automata-network/revm-sgx.git
$ cd revm-sgx
$ # cargo install cargo-sgx
$ cargo sgx run ../tests/GeneralStateTests/Cancun/*
```

## How it works

In order to run revm in Intel SGX, we leverage the automata-sgx-sdk implementation. See the [official docs](https://github.com/automata-network/automata-sgx-sdk) to view more details.

To ensure successful compilation, other necessary adjustments include:
* Use sgx_rand instead of the normal rand and getrandom crate, whose instructions are not compatible in SGX.
* Ensure all cryptographic algorithms are compatible in SGX.
