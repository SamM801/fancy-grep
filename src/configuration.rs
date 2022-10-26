use std::fs::File;
use std::env::args;

/*
  TODO
  - Make the Configuration struct have a full fledged argument parser,
    to allow for flags. DONE (but may need more work)
  - Pass some of the logic of from_args into a new struct named CLInstance,
    which will handle the file opening, highlighting, and the rest of finding logic in the program. NOT NEEDED (LOGIC FROM MAIN WILL BE IMPLEMENTED ON THE INST INSTEAD)
*/

pub struct Configuration {
  pub file: File,
  pub fname: String,
  pub search_term: String,
  pub flags: Vec<String>,
}

pub enum ConfigurationError {
  NotEnoughArguments,
  FileNotFound(String),
  InvalidFlag,
}

impl Configuration {
  pub fn from_args() -> Result<Configuration, ConfigurationError> {
    let mut args: Vec<String> = args().collect();
    let fname = args.get(1).ok_or(ConfigurationError::NotEnoughArguments)?.to_owned();
    let search_term = args.get(2).ok_or(ConfigurationError::NotEnoughArguments)?.to_owned();
    let flags: Vec<String> = args.drain(2..).collect::<Vec<String>>().to_owned();

    match File::open(&fname) {
      Err(_) => return Err(ConfigurationError::FileNotFound(fname.to_string())),
      Ok(file) => {
        return Ok(Configuration {
        file,
        fname,
        search_term,
        flags
        })
      }
    }
  }
  
  pub fn instance_is_for_help(&self) -> Result<bool, ConfigurationError> {
    let args = args().collect::<Vec<String>>();
    let help_flag = args.get(1)
      .ok_or(ConfigurationError::NotEnoughArguments)?;

    if help_flag != "--help" { return Ok(false) }
    Ok(true)
    
  }
}
