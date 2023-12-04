use std::{fmt, fmt::Debug, panic::Location};

#[derive(Debug)]
pub struct Error {
  line: u32,
  column: u32,
  file: String
}

impl Error {
  #[track_caller]
  pub(crate) fn new(err: &str) -> Self {
    let caller = Location::caller();
    let line = Location::line(caller);
    let column = Location::column(caller);
    let file = Location::file(caller).to_string();
    Self {line, column, file}
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Unexpected error in HTML Editor")
  }
}

impl std::error::Error for Error {}
