use std::{fmt::Debug, panic::Location};

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

impl<T: std::error::Error + 'static> From<T> for Error {
  #[track_caller]
  fn from(value: T) -> Self {
    let caller = Location::caller();
    let line = Location::line(caller);
    let column = Location::column(caller);
    let file = Location::file(caller).to_string();
    Self {line, column, file}
  }
}
