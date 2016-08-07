extern crate getopts;

use getopts::Options;
use std::env;

fn main () {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut options = Options::new();
  options.optflag("h", "help", "Print this message");
  options.optflag("v", "version", "Print version information");

  let matches = match options.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  // help command
  if matches.opt_present("h") {
    print_usage(&program, options);
    return;
  }

  // version command
  if matches.opt_present("v") {
    println!("v1.0.0");
    return;
  }

  if !matches.free.is_empty() {
    let input = matches.free.clone();
    let command = &input[0];

    match command.trim() {
      "config" => {
        println!("Command not yet implemented")
      },
      "init" => {
        println!("Command not yet implemented")
      },
      "build" => {
        println!("Command not yet implemented")
      },
      "status" => {
        println!("Command not yet implemented")
      },
      "deploy" => {
        println!("Command not yet implemented")
      },
      _ => {
        println!("Error: unknown command '{}'\n", command.trim());
        print_usage(&program, options);
      }
    }
  } else {
    println!("Error: expected <command> to be passed\n");
    print_usage(&program, options);
  }
}

// print CLI usage
fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} <file> [options]", program);
  print!("{}", opts.usage(&brief));
}
