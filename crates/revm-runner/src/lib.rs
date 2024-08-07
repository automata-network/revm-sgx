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

use automata_sgx_builder::sgxlib::sgx_types;
use sgx_types::error::SgxStatus;

pub mod merkle_trie;
pub mod models;
mod runner;
pub mod utils;

// pub use runner::TestError as Error;

use runner::{find_all_json_tests, run, TestError};
use std::path::PathBuf;
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

#[no_mangle]
pub extern "C" fn run_server() -> SgxStatus {
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
