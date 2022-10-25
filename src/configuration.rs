use std::fs;
use std::env::args;

/*
  TODO
  - Make the Configuration struct have a full fledged argument parser,
    to allow for flags.
  - Pass some of the logic of from_args into a new struct named CLInstance,
    which will handle the file opening, highlighting, and the rest of finding logic
    in the program.
*/

pub struct Configuration {
  pub file: fs::File,
  pub fname: String,
  pub search_term: String,
}

pub enum ConfigurationError {
  NotEnoughArguments,
  FileNotFound(String),
}

impl Configuration {
  pub fn from_args() -> Result<Configuration, ConfigurationError> {
    
    let fname = if let Some(arg) = args().nth(1) {
      arg
    } else {
      return Err(ConfigurationError::NotEnoughArguments);
    };

    let sterm = if let Some(arg) = args().nth(2) {
      arg
    } else {
      return Err(ConfigurationError::NotEnoughArguments);
    };

    if let Ok(file) = fs::File::open(&fname) {
      return Ok(Configuration {
        file,
        fname,
        search_term: sterm,
      });
    } else {
      return Err(ConfigurationError::FileNotFound(fname));
    }
  }

  pub fn instance_is_for_help() -> Result<bool, ConfigurationError> {
    if let Some(arg) = args().nth(1) {
      if arg == "--help" {
        return Ok(true);
      }
      return Ok(false);
    } else {
      return Err(ConfigurationError::NotEnoughArguments);
    }
  }
}
