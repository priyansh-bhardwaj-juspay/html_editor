use std::{fmt, fmt::Debug, panic::Location};

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Unexpected error in HTML Editor")
  }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct ErrorDetail {
  line: u32,
  column: u32,
  file: String
}

impl ErrorDetail {
  #[track_caller]
  pub fn new() -> Self {
    let caller = Location::caller();
    let line = Location::line(caller);
    let column = Location::column(caller);
    let file = Location::file(caller).to_string();
    Self {line, column, file}
  }
}

impl<T: std::error::Error + 'static> From<T> for ErrorDetail {
  #[track_caller]
  fn from(value: T) -> Self {
    let caller = Location::caller();
    let line = Location::line(caller);
    let column = Location::column(caller);
    let file = Location::file(caller).to_string();
    Self {line, column, file}
  }
}
