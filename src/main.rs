use fgrep::configuration::*;
use fgrep::colourize::Colourize::{self, BgBrightWhite, Black, BrightRed, BgGray, RESET};
use std::io::{prelude::*, BufReader};

fn find_string_and_highlight(text: &String, term: &str) -> Option<String> {
  let mut highlighted = String::new();
  if let Some(index) = text.find(term) {

    let middle_string = &text[index..index+term.len()];
    let middle_string = format!("{}{middle_string}{RESET}", Colourize::from_rgb(true, 20, 180, 20));
    
    highlighted.push_str(&text[..index]);
    highlighted.push_str(&middle_string[..]);
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
      At least, this is what it WILL be whenever i actually add flags and regular expression support lel
  */
  
  let config: Configuration;
  match Configuration::from_args() {
    Ok(cfg) => config = cfg,
    Err(ConfigurationError::NotEnoughArguments) => {
      eprintln!("{BrightRed}The program encountered an error!");
      eprintln!("Too few arguments were supplied when calling this program!");
      eprintln!("\nHINT: you call fgrep with the following syntax:");
      eprintln!("fgrep (File Path) (Search Term) [--regex, --decolor]{RESET}");
      return;
    },
    Err(ConfigurationError::FileNotFound(fname)) => {
      eprintln!("{BrightRed}The program encountered an error!");
      eprintln!("Your file ({}) could not be found!", fname);
      eprintln!("\nHINT: check what directory you are calling");
      eprintln!("me from, and use a relative path from there!{RESET}");
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
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}", ind, standard_spaces(&ind), linebefore);
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}", ind+1, standard_spaces(&ind), line);
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}\n", ind+2, standard_spaces(&ind), lineafter);
        } else {
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}", ind, standard_spaces(&ind), linebefore);
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}\n", ind+1, standard_spaces(&ind), line);
        }
      } else {
        if let Some(lineafter) = lines.get(ind+1) {
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}", ind+1, standard_spaces(&ind), line);
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}\n", ind+2, standard_spaces(&ind), lineafter);
        } else {
          println!("{BgBrightWhite}{Black}{}{}{BgGray} {RESET} {}\n", ind+1, standard_spaces(&ind), line);
        }
      }
    } else {
    }
  }
}