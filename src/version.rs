use std::ffi::CStr;

use ffi;

/// Returns the database version level that the engine supports
pub fn flevel() -> u32 {
    unsafe { ffi::cl_retflevel() }
}

/// Gets the clamav engine version
///
/// # Example
///
/// ```
/// use clamav::{version};
///
/// println!("Running version {} flevel {}", version::version(), version::flevel());
/// ```
pub fn version() -> String {
    unsafe {
        return {
            CStr::from_ptr(ffi::cl_retver())
            .to_str()
            .expect("Invalid UTF8 string")
            .to_owned()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_success() {
        ::initialize().expect("initialize should succeed");
        assert!(version().len() > 0, "expected a version");
    }

    #[test]
    fn flevel_success() {
        ::initialize().expect("initialize should succeed");
        assert!(flevel() > 0, "expected an flevel");
    }
}
