mod help;

use std::env;
use std::process;
use redis_core::version;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help::print_usage(0);
    }
    let arg1 = &args[1];
    if arg1 == "-v" || arg1 == "--version" {
        println!("{}", version::redis_cli_version());
        process::exit(0);
    }
}