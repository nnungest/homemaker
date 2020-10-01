extern crate console;
use console::style;
use std::fmt;
use std::io;
use std::process::ExitStatus;

pub struct HomemakerError {
  line_number: usize,
  complaint: String,
  encapsulated_error: Option<io::Error>,
}

impl fmt::Display for HomemakerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.line_number, self.complaint)
  }
}

impl fmt::Debug for HomemakerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "HomemakerError {{ line_number: {}, complaint: {}, encapsulated_error: {} }}",
      self.line_number,
      self.complaint,
      self.encapsulated_error.as_ref().unwrap().to_string()
    )
  }
}

impl From<io::Error> for HomemakerError {
  fn from(err: io::Error) -> Self {
    HomemakerError {
      line_number: 0,
      complaint: err.to_string(),
      encapsulated_error: Some(err),
    }
  }
}
#[derive(Debug)]
pub enum HMError {
  Io(io::Error),
  Regular(ErrorKind),
  Other(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
  DependencyUndefinedError,
  CyclicalDependencyError,
  SolutionError,
  ConfigError,
  Other,
}

#[derive(Debug)]
struct ConfigError {
  line_number: i32,
  complaint: String,
}

#[derive(Debug)]
struct SolutionError {
  line_number: i32,
  exit_status: ExitStatus,
  complaint: String,
}

#[derive(Debug)]
struct DependencyUndefinedError {
  line_number: i32,
  dependency: String,
  dependent: String,
}

#[derive(Debug)]
struct CyclicalDependencyError {
  line_number: i32,
  dependency: String,
}

impl From<io::Error> for HMError {
  fn from(err: io::Error) -> HMError {
    HMError::Io(err)
  }
}

impl ErrorKind {
  fn as_str(&self) -> &str {
    match *self {
      ErrorKind::ConfigError => "configuration error",
      ErrorKind::SolutionError => "solution error",
      ErrorKind::DependencyUndefinedError => "dependency undefined",
      ErrorKind::CyclicalDependencyError => "cyclical dependency",
      ErrorKind::Other => "other error",
    }
  }
}

//impl Error for HMError {
//  fn description(&self) -> &str {
//    match *self {
//      HMError::Regular(ref err) => err.as_str(),
//      HMError::Custom(ref err) => err
//    }
//  }
//}

impl fmt::Display for HMError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      HMError::Regular(ref err) => write!(f, "A regular error occurred {:?}", err),
      HMError::Other(ref err) => write!(f, "An error occurred {:?}", err),
      HMError::Io(ref err) => err.fmt(f),
    }
  }
}

pub fn error(complaint: String, er: &str) {
  eprintln!("{}:\n ↳ Error: {}", style(complaint).red().bold(), er)
}

pub type Result<T> = std::result::Result<T, HMError>;
