#![deny(clippy::all)]

use std::{
    ffi::{CStr, CString},
    os::{fd::RawFd, raw::c_int},
};

use napi::{Error, Result, Status};
use napi_derive::napi;
use nix::{
    fcntl::{FcntlArg, FdFlag},
    unistd,
};

#[napi]
/// Performs the execvp system call with the given file and arguments, replacing
/// the current process image with the new one.
///
/// This function does not return if successful, as the current process is
/// replaced by the new one. An error may be thrown, with an error message
/// containing the error code returned by execvp.
///
/// Note that the close-on-exec flag should be cleared for the process's file
/// descriptors. Otherwise, they will be closed automatically when the new
/// process is executed, which will likely make it fail. Use the
/// `doNotCloseOnExit` function to clear the flag for a file descriptor.
///
/// @example
/// import { execvp } from '@alphahydrae/exec';
/// execvp('ls', ['/bin/ls', '-l', '.']);
///
/// @param {string} file The file to execute. If not a path, the PATH environment
///                      variable is searched.
/// @param {string[]} args The arguments to pass to the new process. Note that
///                        the first argument, by convention, should point to
///                        the filename associated with the file being executed.
/// @throws {Error} If the execvp system call fails.
pub fn execvp(file: String, args: Vec<String>) -> Result<()> {
    let file = convert_string(file.clone())?;

    let args: Vec<CString> = args
        .iter()
        .map(|arg| convert_string(arg.to_owned()))
        .collect::<Result<Vec<CString>>>()?;
    let args: Vec<&CStr> = args.iter().map(|arg| arg.as_c_str()).collect();

    unistd::execvp(file.as_c_str(), &args)
        .map(|_| ())
        .map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("execvp failed with code {}", err),
            )
        })
}

#[napi]
/// Clears the close-on-exec flag for the given file descriptor, preventing it
/// from being closed automatically when a new process is executed with the exec
/// family of functions.
///
/// @example
/// doNotCloseOnExit(process.stdout.fd);
///
/// @param {number} file The file descriptor to close (e.g. `process.stdout.fd`).
/// @returns {undefined}
/// @throws {Error} If the fcntl system call fails.
pub fn do_not_close_on_exit(fd: i32) -> Result<()> {
    let current_flags = fcntl(fd, FcntlArg::F_GETFD)?;

    let flags_without_close_on_exec =
        FdFlag::from_bits_truncate(current_flags & !FdFlag::FD_CLOEXEC.bits());

    fcntl(fd, FcntlArg::F_SETFD(flags_without_close_on_exec))?;

    Ok(())
}

// Converts a Rust string received from JavaScript to a CString used by Unix
// functions.
fn convert_string(s: String) -> Result<CString> {
    CString::new(s).map_err(|_| Error::new(Status::GenericFailure, "Invalid string"))
}

fn fcntl(fd: RawFd, arg: FcntlArg) -> Result<c_int> {
    nix::fcntl::fcntl(fd, arg).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("fcntl failed with code {}", err),
        )
    })
}
