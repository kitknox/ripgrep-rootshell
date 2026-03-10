//! ripgrep_ios — cdylib wrapper for ripgrep, integrated with ios_system.
//!
//! Uses dup2() to redirect process FDs 0/1/2 to ios_system's thread-local
//! FDs for the duration of the rg command. This enables full pipe and file
//! redirection support without modifying ripgrep's internal I/O paths.

use std::ffi::{c_char, c_int, CStr, OsString};
use std::os::unix::ffi::OsStringExt;

unsafe extern "C" {
    fn ios_stdin() -> *mut libc::FILE;
    fn ios_stdout() -> *mut libc::FILE;
    fn ios_stderr() -> *mut libc::FILE;
}

#[unsafe(no_mangle)]
pub extern "C" fn rg_main(argc: c_int, argv: *const *const c_char) -> c_int {
    // Convert C argv to Rust OsStrings, skip argv[0] (command name).
    let args: Vec<OsString> = (1..argc)
        .map(|i| unsafe {
            let ptr = *argv.offset(i as isize);
            let bytes = CStr::from_ptr(ptr).to_bytes().to_vec();
            OsString::from_vec(bytes)
        })
        .collect();

    // Get ios_system's thread-local FDs.
    let ios_in = unsafe { libc::fileno(ios_stdin()) };
    let ios_out = unsafe { libc::fileno(ios_stdout()) };
    let ios_err = unsafe { libc::fileno(ios_stderr()) };

    // Save original FDs.
    let saved_in = unsafe { libc::dup(0) };
    let saved_out = unsafe { libc::dup(1) };
    let saved_err = unsafe { libc::dup(2) };

    // Redirect FD 0/1/2 to ios_system's FDs.
    unsafe {
        libc::dup2(ios_in, 0);
        libc::dup2(ios_out, 1);
        libc::dup2(ios_err, 2);
    }

    // Run ripgrep.
    let exit_code = ripgrep_core::run_with_args(args);

    // Restore original FDs.
    unsafe {
        libc::dup2(saved_in, 0);
        libc::dup2(saved_out, 1);
        libc::dup2(saved_err, 2);
        libc::close(saved_in);
        libc::close(saved_out);
        libc::close(saved_err);
    }

    exit_code
}
