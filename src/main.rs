use fgrep::configuration::*;
use std::io::{prelude::*, BufReader};

fn find_string_and_highlight(text: &String, term: &str) -> Option<String> {
  let mut highlighted = String::new();
  while let Some(index) = text.find(term) {
    highlighted.push_str(&text[]);
  }
  Some(highlighted)
}

fn main()  {
  /*
    GREP in rust with search term highlighting and
    Regular Expression support.

    Syntax:
      fgrep (File Path) (Search Term) [--regex, --minify]
  */
  
  let config: Configuration;
  match Configuration::from_args() {
    Ok(cfg) => config = cfg,
    Err(ConfigurationError::NotEnoughArguments) => {
      eprintln!("\x1B[38;2;220;40;40mThe program encountered an error!");
      eprintln!("Too few arguments were supplied when calling this program!");
      eprintln!("\nHINT: you call fgrep with the following syntax:");
      eprintln!("fgrep (File Path) (Search Term) [--regex, --decolor]\x1B[39;49m");
      return;
    },
    Err(ConfigurationError::FileNotFound(fname)) => {
      eprintln!("\x1B[38;2;220;40;40mThe program encountered an error!");
      eprintln!("Your file ({}) could not be found!", fname);
      eprintln!("\nHINT: check what directory you are calling");
      eprintln!("me from, and use a relative path from there!\x1B[39;49m");
      return;
    },
  }

  /*
    The grep part of the program
  */
  let lines = BufReader::new(config.file).lines().map(|l| l.unwrap_or(String::new())).collect::<Vec<String>>(); // TURBOFISH SYNTAX :O
  for ind in 0..lines.len() {
    if let Some(line) = find_string_and_highlight(&lines[ind], &config.search_term) {
      println!("{}\t| {line}", ind+1);
    }
  }
}