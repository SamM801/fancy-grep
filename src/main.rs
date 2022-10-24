use fgrep::configuration::*;
use std::io::{prelude::*, BufReader};

fn find_string_and_highlight(text: &String, term: &str) -> Option<String> {
  let mut highlighted = String::new();
  if let Some(index) = text.find(term) {
    highlighted.push_str(&text[..index]);
    highlighted.push_str(&text[index..index+term.len()]);
    highlighted.push_str(&text[index+term.len()..]);
    return Some(highlighted);
  }
  None
}

fn safe_subtract(n1: usize, n2: usize) -> Option<usize> {
  if n1 == 0 {
    return None;
  }
  Some((n1-n2) as usize) 
}

fn standard_spaces(line_nr: &usize) -> String {
  let mut line_nr = line_nr.to_owned() as f64;
  line_nr = 1.0 + line_nr.log10().floor();
  if line_nr == f64::NEG_INFINITY { line_nr = 1.0; }
  " ".repeat( ( ( ( line_nr / 4.0 ).floor() * 4.0 ) + 4.0 - line_nr) as usize )
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
      if let Some(linebefore) = lines.get(safe_subtract(ind, 1).unwrap_or(usize::MAX)) {
        if let Some(lineafter) = lines.get(ind+1) {
          println!("{}{}| {}", ind, standard_spaces(&ind), linebefore);
          println!("{}{}| {}\x1B[39;49m", ind+1, standard_spaces(&ind), line);
          println!("{}{}| {}\n", ind+2, standard_spaces(&ind), lineafter);
        } else {
          println!("{}{}| {}", ind, standard_spaces(&ind), linebefore);
          println!("{}{}| {}\x1B[39;49m\n", ind+1, standard_spaces(&ind), line);
        }
      } else {
        if let Some(lineafter) = lines.get(ind+1) {
          println!("{}{}| {}\x1B[39;49m", ind+1, standard_spaces(&ind), line);
          println!("{}{}| {}\n", ind+2, standard_spaces(&ind), lineafter);
        } else {
          println!("{}{}| {}\x1B[39;49m\n", ind+1, standard_spaces(&ind), line);
        }
      }
    } else {
    }
  }
}