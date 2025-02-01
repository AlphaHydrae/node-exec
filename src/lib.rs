#![deny(clippy::all)]

use std::ffi::{CStr, CString};

use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi]
pub fn execvp(file: String, args: Vec<String>) -> Result<()> {
  let file_cstring = CString::new(file.clone()).unwrap();

  let args: Vec<CString> = args
    .iter()
    .map(|arg| string_ref_to_c_str(arg).unwrap())
    .collect();
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

fn string_ref_to_c_str(string: &str) -> Result<CString> {
  CString::new(string.to_owned()).map_err(|_| Error::new(Status::GenericFailure, "Invalid string"))
}
