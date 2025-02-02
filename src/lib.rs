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
