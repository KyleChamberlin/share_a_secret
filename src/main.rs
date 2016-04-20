extern crate getopts;

use getopts::Options;
use getopts::Matches;

use std::io;
use std::env;
use std::path::Path;
use std::num::ParseIntError;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
  let stderr = io::stderr();
  let args: Vec<String> = env::args().collect();
  let program = &args[0];

  let options = set_cli_options(&program);

  let matched_options = match options.parse(&args[1..]) {
    Ok(opts)  =>  { opts }
    Err(error)=>  { panic!(error.to_string()) }
  };

  if args.len() < 2 || matched_options.opt_present("help") {
    print_help(&program, options);
    return;
  }

  if matched_options.opt_present("version") {
    print_version(&program);
    return;
  }

  match parse_cli_args(matched_options) {
    Ok(Action::Encode(k, n)) => println!("encode with k={} and n={}.", k, n),
    Ok(Action::Decode) => println!("decode."),
    Err(e) => println!("ERROR: {}", e.to_string())
  };
}

fn parse_cli_args(matched_options: Matches) -> Result<Action, &'static str> {
  match (matched_options.opt_present("decode"), matched_options.opt_present("n"), matched_options.opt_present("k")) {
      (true, invalid_n, invalid_k) => {
        if invalid_n || invalid_k {
          Err("options for shares and required minimums are not valid with the decode option.")
        } else {
          Ok(Action::Decode)
        }
      }
      (false, n_present, k_present) => {
        if n_present && k_present {
          let k = matched_options.opt_str("k").unwrap();
          let n = matched_options.opt_str("n").unwrap();
          if let Ok(k) = parse_numeric_parameter(&k) {
            if let Ok(n) = parse_numeric_parameter(&n) {
              if k > 0 && k <= n {
                Ok(Action::Encode(k,n))
              } else {
                Err("invalid encoding parameters, k or n.")
              }
            } else {
              Err("n must be a number")
            }
          } else {
            Err("k must be a number")
          }
        } else {
          Err("encoding requires both k and n.")
        }
      }
    }
}

fn parse_numeric_parameter(parameter: &str) -> Result<u8, ParseIntError> {
  let parameter = parameter.trim();
  let number = try!(parameter.parse());
  Ok(number)
}

fn set_cli_options(program: &str) -> Options {
  let mut options = Options::new();
  options.optflag("h", "help", "prints this help message.");
  options.optflag("v", "version", &format!("display current version of {}, {}", program, VERSION));
  options.optflag("d", "decode", "decode a secret from a set of shares (one per line).");
  options.optopt("n", "shares", "the number of shares to generate.", "TOTAL_SHARES");
  options.optopt("k", "required", "the minimum number of shares required to decode the secret, use
  with encoding.", "MINIMUM_REQUIRED_SHARES_FOR_DECODE");
  options.optopt("s", "secret", "the secret file you would like to share. \nIf no secret file is
  defined, input is read from STDIN.", "SECRET_FILE");
  options.optopt("o", "output", "file in which to output shares. \nIf no output file is defined,
  output is written to STDOUT.", "SHARES_OUTPUT_FILE");

  return options;
}

fn print_help(program: &str, opts: Options) {
  println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn print_version(program: &str) {
  println!("{} v{}", program, VERSION);
}

enum Action {
  Encode(u8, u8),
  Decode
}

enum Input {
  File(Path),
  STDIN
}

enum Output {
  File(Path),
  STDOUT
}

