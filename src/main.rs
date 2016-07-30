use std::process::Command;

fn main() {
  let output = Command::new("sh")
    .arg("-c")
    .arg("ls")
    .output()
    .expect("failed to execute process");

  println!("{}", String::from_utf8_lossy(&output.stdout));
}
