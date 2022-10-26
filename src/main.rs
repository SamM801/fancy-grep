use fgrep::instance::*;

/*
  TODO
  - (Extended from configuration.rs) remove some of the logic from main() and abstract it to a new struct to clean things up
*/


fn main()  {
  /*
    GREP in rust with search term highlighting and
    Regular Expression support.

    Syntax:
      fgrep (File Path) (Pattern) [--regex, --minify]
      At least, this is what it WILL be whenever i actually add flags and regular expression support lel
  */

  let mut inst = CLInstance::init();
  loop {
    match inst.next() {
      Ok(res) => {
        if let Some(res) = res {
          println!("{}\n", res);
        }
      }

      Err(InstanceError::StopIteration) => break,
    }
  }
  
}
