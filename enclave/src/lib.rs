// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

extern crate sgx_libc;
extern crate sgx_types;

// use alloy_primitives::Address;
use std::collections::HashMap;
use models::{TestSuite, TestUnit};
use serde::Deserialize;
use sgx_types::error::SgxStatus;

pub mod merkle_trie;
pub mod models;
mod runner;
pub mod utils;

// pub use runner::TestError as Error;

use runner::{find_all_json_tests, run, TestError};
use std::{collections::BTreeMap, path::PathBuf};
use structopt::StructOpt;

/// Statetest command
#[derive(StructOpt, Debug)]
pub struct Cmd {
    /// Path to folder or file containing the tests. If multiple paths are specified
    /// they will be run in sequence.
    ///
    /// Folders will be searched recursively for files with the extension `.json`.
    #[structopt(required = true)]
    path: Vec<PathBuf>,
    /// Run tests in a single thread.
    #[structopt(short = "s", long)]
    single_thread: bool,
    /// Output results in JSON format.
    /// It will stop second run of evm on failure.
    #[structopt(long)]
    json: bool,
    /// Output outcome in JSON format. If json is true, this is implied.
    /// It will stop second run of evm on failure.
    #[structopt(short = "o", long)]
    json_outcome: bool,
}

impl Cmd {
    /// Run statetest command.
    pub fn run(&self) -> Result<(), TestError> {
        for path in &self.path {
            println!("\nRunning tests in {}...", path.display());
            let test_files = find_all_json_tests(path);
            run(test_files, self.single_thread, self.json, self.json_outcome)?
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = match Cmd::from_args_safe() {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return Ok(());
        }
    };

    if let Err(e) = cmd.run() {
        println!("{e}");
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn run_server() -> SgxStatus {
    // let result = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .max_blocking_threads(4)
    //     .enable_all()
    //     .build()
    //     .map(|rt| rt.block_on(main()));

    // match result {
    //     Ok(Ok(_)) => SgxStatus::Success,
    //     Ok(Err(e)) => {
    //         println!("Failed to run server: {e}");
    //         SgxStatus::Unexpected
    //     }
    //     Err(e) => {
    //         println!("Failed to create tokio runtime in enclave: {e}");
    //         SgxStatus::Unexpected
    //     }
    // }
    let s = {
        r#"{
        "13_tloadStaticCall" : {
            "_info" : {
                "comment" : "Transient storage cannot be manipulated in a static context, tstore reverts.",
                "filling-rpc-server" : "evm version 1.13.11-unstable-765f2904-20240124",
                "filling-tool-version" : "retesteth-0.3.2-cancun+commit.ea13235b.Linux.g++",
                "generatedTestHash" : "0bd696bf1aff4e81e3e3d7597c2b3d4ece9fa9e35ccdd047f19b860cf3826cb2",
                "lllcversion" : "Version: 0.5.14-develop.2023.7.11+commit.c58ab2c6.mod.Linux.g++",
                "solidity" : "Version: 0.8.21+commit.d9974bed.Linux.g++",
                "source" : "src/GeneralStateTestsFiller/Cancun/stEIP1153-transientStorage/13_tloadStaticCallFiller.yml",
                "sourceHash" : "b08740abef0f7387ccfa683474ba9c62239a9486d01ef39a39b1645d643c2f58"
            },
            "env" : {
                "currentBaseFee" : "0x0a",
                "currentCoinbase" : "0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba",
                "currentDifficulty" : "0x020000",
                "currentExcessBlobGas" : "0x00",
                "currentGasLimit" : "0x10000000000000",
                "currentNumber" : "0x01",
                "currentRandom" : "0x0000000000000000000000000000000000000000000000000000000000020000",
                "currentTimestamp" : "0x03e8"
            },
            "post" : {
                "Cancun" : [
                    {
                        "hash" : "0x77825674bc60bc90ff40c07a1947bc3d4330a61e69948ed4f707b5ff01ab0674",
                        "indexes" : {
                            "data" : 0,
                            "gas" : 0,
                            "value" : 0
                        },
                        "logs" : "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                        "txbytes" : "0x02f8690180808207d083061a8094a00000000000000000000000000000000000000a80843f371692c080a0743568d659bf985457cf172809a70ead397472fa1556b80a02817cc0f0c08738a066863f0fc471d35cf71f17ee40a9df4d1bb4033af9beceb9584dbf6e34b6c1e4"
                    }
                ]
            },
            "pre" : {
                "0xa00000000000000000000000000000000000000a" : {
                    "balance" : "0x0de0b6b3a7640000",
                    "code" : "0x5f3560e01c80633f371692146030578063611e535a14602757630172927514602357005b603d565b50602e6037565b005b50602e6046565b60035f5d565b5f5c5f5260205ff35b60025f5d63308f29ad60e11b5f525f806020813061fffffa5f555f5c600155630172927560e01b5f5260205f8181305afa6002555f5160035556",
                    "nonce" : "0x00",
                    "storage" : {
                        "0x00" : "0xffff"
                    }
                },
                "0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b" : {
                    "balance" : "0x3635c9adc5dea00000",
                    "code" : "0x",
                    "nonce" : "0x00",
                    "storage" : {
                    }
                }
            },
            "transaction" : {
                "accessLists" : [
                    [
                    ]
                ],
                "data" : [
                    "0x3f371692"
                ],
                "gasLimit" : [
                    "0x061a80"
                ],
                "maxFeePerGas" : "0x07d0",
                "maxPriorityFeePerGas" : "0x00",
                "nonce" : "0x00",
                "secretKey" : "0x45a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d8",
                "sender" : "0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b",
                "to" : "0xa00000000000000000000000000000000000000a",
                "value" : [
                    "0x00"
                ]
            }
        }
    }"#
    };
    // let s = r#"{"a": {"env": {"address":"0xa00000000000000000000000000000000000000a"}}}"#;

    let _json: TestSuite = match serde_json::from_str(s) {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return SgxStatus::Unexpected;
        }
    };

    let cmd = match Cmd::from_args_safe() {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return SgxStatus::Success;
        }
    };

    if let Err(e) = cmd.run() {
        println!("{e}");
    }

    SgxStatus::Success
}
