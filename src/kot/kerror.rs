/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Error module for dotfiles manager kot                               ##
##   This module supports converting errors to custom types using ? operator  ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::fmt::{Debug, Display, Formatter};

// Error types for kot application
#[derive(Debug)]
pub enum ErrorKind {
  ConfigError(String),
  GitError(String),
  IOError(String),
  FileError(String),
  DirError(String),
  Other(String),
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

#[derive(Debug)]
pub struct Error {
  pub kind: ErrorKind,
  message: String,
}

// Implement Display trait for printing found errors
impl std::fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Kot {:?}", self)
  }
}

impl std::error::Error for Error { }

impl Error {
  pub fn new(kind: ErrorKind, message: String) -> Error {
    Error {
      kind: kind,
      message: message.to_string(),
    }
  }
}

// Implement From<T> for each error type T that we want to handle
// These implementations handle converting from T to kot::kerror::Error using ?

// Converting from std::io::Error to kot::kerror::Error::GitError
impl std::convert::From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    return Error::new(ErrorKind::IOError(error.to_string()),
                      "(std::io error)".to_owned());
  }
}

// Converting from fs_extra::error::Error to kot::kerror::Error::GitError
impl std::convert::From<fs_extra::error::Error> for Error {
  fn from(error: fs_extra::error::Error) -> Self {
    return Error::new(ErrorKind::FileError(error.to_string()),
                      "(fs_extra error)".to_owned());
  }
}

// -----------------------------------------------------------------------------


