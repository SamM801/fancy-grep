/*
  NOT COMPLETED DO NOT PUSH TO REPO
*/

pub enum Colourize {
  Black,
  White,
  Red,
  Yellow,
  Green,
}

impl Colourize {
  pub fn from_rgb(foreground: bool, r: u8, g: u8, b: u8) -> String{
    format!("\x1b[{};2;{r};{g};{b}m", foreground : if foreground { 38 } else { 48 })
  }
}