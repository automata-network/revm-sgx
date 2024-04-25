# revm-sgx

This encalve app is the SGX version of the Rust EVM executor or short REVME. The official standard version can be found [here](https://github.com/automata-network/revm/tree/main/bins/revme). Presently, its primary application is in executing Ethereum tests.

## Prerequisite
* SGX-SDK 2.17.1 installation
* Cargo and Rust installation, currently teaclave-sgx-sdk requires nightly-2023-11-17 rustc version
* Basic dev toolkits installation like cmake

If you're using Azure, the following are recommended configurations:
* VM spec: `Standard_DC4s_v3`
* OS spec: `Ubuntu Server 20.04 LTS - x64 Gen2`

## Getting Started

The encalve app takes a path to the directory where ethereum statetest json can be found. It recursively parses all json files in the specified directory and executes them.

Running all [ethereum tests](https://github.com/ethereum/tests) checks that revm is compliant to the ethereum specs.

To run [ethereum tests](https://github.com/ethereum/tests) locally, clone the [tests](https://github.com/ethereum/tests) repository and provide the test directory. Below, we clone the repo and execute the GeneralStateTests suite of tests.

```shell
$ git clone https://github.com/ethereum/tests.git
$ git clone https://github.com/automata-network/revm-sgx.git
$ cd revm-sgx
$ git submodule update --init --recursive
$ make
$ cd bin
$ ./server ../../tests/GeneralStateTests/Cancun/*
```

## How it works

In order to run revm in Intel SGX, we leverage the teaclave-sgx-sdk 2.0 implementation, mainly use its `cargo-std-aware` mode in cargo build and prevent modifying a lot of dependencies to compile them into no_std. See their [official docs](https://github.com/apache/incubator-teaclave-sgx-sdk/blob/v2.0.0-preview-11-17/README.md) to view more details.

To ensure successful compilation, other necessary adjustments include:
* Fix the ctq inverse mod bug in blst when `__BLST_NO_CPUID__` is enabled, which is included in c-kzg dependency and affects the EIP4844 precompile instruction.
* Use sgx_rand instead of the normal rand and getrandom crate, whose instructions are not compatible in SGX.
* Ensure all cryptographic algorithms are compatible in SGX.
