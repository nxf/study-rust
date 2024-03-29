use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt");

  let _f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Tried to create file: {:?}", e),
      },
      other_error => panic!("There was a problem opening the file: {:?}", other_error)
    },
  };
}
