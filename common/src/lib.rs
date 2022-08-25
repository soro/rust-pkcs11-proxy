pub mod types;
use types::*;

use std::ffi;

#[no_mangle]
pub extern "C" fn C_Initialize(init_args: *mut ffi::c_void) {
    let args = init_args as *mut CkCInitializeArgs;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
