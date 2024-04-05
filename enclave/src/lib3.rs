use sgx_types::error::SgxStatus;

#[no_mangle]
pub extern "C" fn run_server() -> SgxStatus {
    SgxStatus::Success
}