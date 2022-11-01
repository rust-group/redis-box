use redis_core::version;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let exec_name = &args[0];
    if args.len() != 2 {
        println!("Usage: {exec_name} <rdb-file-name>");
        process::exit(1);
    }
    let arg1 = &args[1];
    if arg1 == "-v" || arg1 == "--version" {
        println!("{}", version::check_rdb_version());
        process::exit(0);
    }

    println!("[offset 0] Checking RDB file {arg1}");
}
