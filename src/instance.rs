use super::configuration::*;
use super::colourize::Colourize::{self, RESET, BrightRed, BgBrightWhite, BgGray, Black, Red};
use std::process::exit;
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

fn standard_spaces(line_nr: &usize) -> String {
  let mut line_nr = line_nr.to_owned() as f64;
  line_nr = 1.0 + line_nr.log10().floor();
  if line_nr == f64::NEG_INFINITY { line_nr = 1.0; }
  " ".repeat( ( ( ( line_nr / 4.0 ).floor() * 4.0 ) + 4.0 - line_nr) as usize )
}

pub enum InstanceError {
  StopIteration,
}

pub struct CLInstance {
  config: Configuration,
  iter: usize,
  lines: Vec<String>,
  state: State,
}

enum State {
  BEGINNING,
  MIDDLE,
  END,
}

impl CLInstance {
  pub fn init() -> CLInstance {
    let config = match Configuration::from_args() {
      Ok(config) => config,
      Err(ConfigurationError::NotEnoughArguments) => {
        eprintln!("{BrightRed}The program encountered an error!");
        eprintln!("Too few arguments were supplied when calling this program!");
        eprintln!("\nHINT: you call fgrep with the following syntax:");
        eprintln!("fgrep (File Path) (Pattern) [--regex, --minify]{RESET}");
        exit(1);
      },
      Err(ConfigurationError::FileNotFound(fname)) => {
        eprintln!("{BrightRed}The program encountered an error!");
        eprintln!("Your file ({}) could not be found!", fname);
        eprintln!("\nHINT: check what directory you are calling");
        eprintln!("me from, and use a relative path from there!{RESET}");
        exit(1);
      },
      Err(ConfigurationError::InvalidFlag) => {
        eprintln!("{BrightRed}The program encountered an error!");
        eprintln!("Some flags provided were invalid!");
        eprintln!("\nHINT: The only supported flags right now are");
        eprintln!("'--regex' and '--minify'!{RESET}");
        exit(1);
      }
    };

    if match config.instance_is_for_help() {
      Ok(r) => r,
      Err(ConfigurationError::NotEnoughArguments) => {
        eprintln!("{BrightRed}The program encountered an error!");
        eprintln!("Too few arguments were supplied when calling this program!");
        eprintln!("\nHINT: you call fgrep with the following syntax:");
        eprintln!("fgrep (File Path) (Search Term) [--regex, --decolor]{RESET}");
        exit(1);
      }
      Err(_) => panic!("{BrightRed}THIS SHOULD NOT HAVE HAPPENED IN ANY CIRCUMSTANCE{RESET}"),
    } {
      println!("    PROJECT {BgBrightWhite}{Black}fgrep v0.2.0{RESET}");
      println!("fgrep is a command line tool to find patterns in files.");
      println!("\nThe Syntax:");
      println!("  fgrep (File Path) (Pattern) [--regex --minify]\n");
      println!("Attempting to use a regular expression in the pattern field");
      println!("is {Red}erroneous{RESET} and will treat the regex as a literal string!");
      exit(0)
    }

    let lines = BufReader::new(&config.file)
      .lines()
      .map(|l| l.unwrap_or(String::new()));
    let lines: Vec<String> = lines.collect();
      
    CLInstance {
      config,
      iter: 0,
      lines,
      state: State::BEGINNING
    }
  }

  fn update_state(&mut self) {
    if self.iter == 0 {
      self.state = State::BEGINNING;
    } else if self.iter == self.lines.len() - 1 {
      self.state = State::END;
    } else {
      self.state = State::MIDDLE;
    }
  }
  
  pub fn next(&mut self) -> Result<Option<String>, InstanceError> {
    self.update_state();
    if self.iter == self.lines.len() {
      return Err(InstanceError::StopIteration);
    }
    
    match &self.state {
      State::BEGINNING => {
        let current_line = &self.lines[self.iter];
        let ctx_next = &self.lines[self.iter+1];
        let spaces = standard_spaces(&(self.iter+1));
        let line_nr = self.iter+1;
        let next_line_nr = self.iter+2;
        self.iter += 1;

        if let Some(found_str) = find_string_and_highlight(current_line, &self.config.search_term) {
          return Ok(Some(format!("{BgBrightWhite}{Black}{line_nr}{spaces}{BgGray} {RESET} {found_str}\n{BgBrightWhite}{Black}{next_line_nr}{spaces}{BgGray} {RESET} {ctx_next}")));
        }
        return Ok(None);
      }

      State::MIDDLE => {
        let current_line = &self.lines[self.iter];
        let ctx_prev = &self.lines[self.iter-1];
        let ctx_next = &self.lines[self.iter+1];
        let spaces = standard_spaces(&(self.iter+1));
        let prev_line_nr = self.iter;
        let line_nr = self.iter+1;
        let next_line_nr = self.iter+2;
        self.iter += 1;

        if let Some(found_str) = find_string_and_highlight(current_line, &self.config.search_term) {
          return Ok(Some(format!("{BgBrightWhite}{Black}{prev_line_nr}{spaces}{BgGray} {RESET} {ctx_prev}\n{BgBrightWhite}{Black}{line_nr}{spaces}{BgGray} {RESET} {found_str}\n{BgBrightWhite}{Black}{next_line_nr}{spaces}{BgGray} {RESET} {ctx_next}")));
        }
        return Ok(None);
      }

      State::END => {
        let current_line = &self.lines[self.iter];
        let ctx_prev = &self.lines[self.iter-1];
        let spaces = standard_spaces(&(self.iter));
        let prev_line_nr = self.iter;
        let line_nr = self.iter+1;
        self.iter += 1;

        if let Some(found_str) = find_string_and_highlight(current_line, &self.config.search_term) {
          return Ok(Some(format!("{BgBrightWhite}{Black}{prev_line_nr}{spaces}{BgGray} {RESET} {ctx_prev}\n{BgBrightWhite}{Black}{line_nr}{spaces}{BgGray} {RESET} {found_str}")));
        }
        return Ok(None);
      }
    }
  }
}