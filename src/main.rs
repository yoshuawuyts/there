use std::process::Command;

pub fn config () {
  println!("Config is not yet implemented");
}

pub fn init () {
  let script_init = include_str!("./init");
  run(script_init);
}

pub fn build () {
  println!("Build is not yet implemented");
}

pub fn status () {
  println!("Status is not yet implemented");
}

pub fn deploy () {
  println!("Deploy is not yet implemented");
}

fn run (script: &str) {
  let output = Command::new("sh")
    .arg("-c")
    .arg(script)
    .output()
    .expect("failed to execute process");

  println!("status: {}", output.status);
  println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
  println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
