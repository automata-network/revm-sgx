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

use automata_sgx_builder::sgxlib::{sgx_types, sgx_urts};

use std::path::PathBuf;
use sgx_types::error::SgxStatus;
use sgx_types::types::*;
use sgx_urts::enclave::SgxEnclave;

static ENCLAVE_FILE: &str = "librevm_runner.signed.so";

extern "C" {
    fn run_server(eid: EnclaveId, retval: *mut SgxStatus) -> SgxStatus;
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let enclave_path = PathBuf::new().join(&args[0]).parent().unwrap().join(ENCLAVE_FILE);
    let enclave = match SgxEnclave::create(enclave_path, true) {
        Ok(enclave) => {
            println!("[+] Init Enclave Successful {}!", enclave.eid());
            enclave
        }
        Err(err) => {
            println!("[-] Init Enclave Failed {}!", err.as_str());
            return;
        }
    };

    let mut retval = SgxStatus::Success;
    let result = unsafe { run_server(enclave.eid(), &mut retval) };
    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => println!("[-] ECall Enclave Failed {}!", result.as_str()),
    }
}
