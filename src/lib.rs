#![deny(clippy::all)]

use std::ffi::{CStr, CString};

use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi]
pub fn execvp(file: String, args: Vec<String>) -> Result<()> {
    let file_cstring = CString::new(file.clone())
        .map_err(|_| Error::new(Status::GenericFailure, "Invalid string"))?;

    let args: Vec<CString> = args
        .iter()
        .map(|arg| {
            CString::new(arg.to_owned())
                .map_err(|_| Error::new(Status::GenericFailure, "Invalid string"))
        })
        .collect::<Result<Vec<CString>>>()?;
    let args: Vec<&CStr> = args.iter().map(|arg| arg.as_c_str()).collect();

    nix::unistd::execvp(file_cstring.as_c_str(), &args)
        .map(|_| ())
        .map_err(|_| Error::new(Status::GenericFailure, "execvp failed"))
}

#[napi]
pub fn do_not_close_on_exit(fd: i32) -> Result<()> {
    let flags = nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFD)
        .map_err(|_| Error::new(Status::GenericFailure, "fcntl failed"))?;

    let flags_without_close_on_exec = flags & !nix::fcntl::FdFlag::FD_CLOEXEC.bits();

    let new_flags = nix::fcntl::FdFlag::from_bits_truncate(flags_without_close_on_exec);
    nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETFD(new_flags))
        .map(|_| ())
        .map_err(|_| Error::new(Status::GenericFailure, "fcntl failed"))
}
