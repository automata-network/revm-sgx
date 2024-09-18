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

use automata_sgx_sdk::types::SgxStatus;

automata_sgx_sdk::enclave! {
    name: RevmRunner,
    ecall: {
        fn run_server() -> SgxStatus;
    }
}

fn main() {
    let result = RevmRunner::new().run_server().unwrap();
    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => println!("[-] ECall Enclave Failed {}!", result.as_str()),
    }
}
