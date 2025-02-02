#![deny(clippy::all)]

use std::ffi::{CStr, CString};

use napi::{Error, Result, Status};
use napi_derive::napi;
use nix::{
    fcntl::{fcntl, FcntlArg, FdFlag},
    unistd,
};

#[napi]
pub fn execvp(file: String, args: Vec<String>) -> Result<()> {
    let file = CString::new(file.clone())
        .map_err(|_| Error::new(Status::GenericFailure, "Invalid string"))?;

    let args: Vec<CString> = args
        .iter()
        .map(|arg| {
            CString::new(arg.to_owned())
                .map_err(|_| Error::new(Status::GenericFailure, "Invalid string"))
        })
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
    let current_flags = fcntl(fd, FcntlArg::F_GETFD).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("fcntl failed with code {}", err),
        )
    })?;

    let flags_without_close_on_exec =
        FdFlag::from_bits_truncate(current_flags & !FdFlag::FD_CLOEXEC.bits());

    fcntl(fd, FcntlArg::F_SETFD(flags_without_close_on_exec)).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("fcntl failed with code {}", err),
        )
    })?;

    Ok(())
}
