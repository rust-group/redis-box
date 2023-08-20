mod help;
mod conf;

use std::env;
use std::process::exit;
use redis_core::version;
use conf::CliConfig;
use conf::Pref;
use conf::consts;
use crate::help::print_usage;

const spectrum_palette_color_size: i32 = 19;
const spectrum_palette_color: [i32; 19] = [0, 233, 234, 235, 237, 239, 241, 243, 245, 247, 144, 143, 142, 184, 226, 214, 208, 202, 196];
const spectrum_palette_mono_size: i32 = 13;
const spectrum_palette_mono: [i32; 13] = [0, 233, 234, 235, 237, 239, 241, 243, 245, 247, 249, 251, 253];


fn main() {
    let first_arg = 0;
    let mut config = CliConfig::new();
    let pref = Pref::new();
    let mut spectrum_palette = spectrum_palette_color.to_vec();
    let mut spectrum_palette_size = spectrum_palette_color_size;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage(0);
    }
    let arg1 = &args[1].clone();

    parse_options(args, &mut config, &mut spectrum_palette, &mut spectrum_palette_size);

    if arg1 == "-v" || arg1 == "--version" {
        println!("{}", version::redis_cli_version());
        return;
    }
}

fn parse_options(args: Vec<String>, config: &mut CliConfig, spectrum_palette: &mut Vec<i32>, spectrum_palette_size: &mut i32) {
    for (i, v) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let next = || args.get(i + 1).unwrap().to_string();
        let last_arg = i == args.len() - 1;

        if v == "-h" && !last_arg {
            config.conn_info.host_ip = next();
        } else if v == "-h" && last_arg {
            help::print_usage(0);
        } else if v == "--help" {
            help::print_usage(0);
        } else if v == "-x" {
            config.stdin_last_arg = 1;
        } else if v == "-X" && !last_arg {
            config.stdin_tag_arg = 1;
            config.stdin_tag_name = next();
        } else if v == "-p" && !last_arg {
            config.conn_info.host_port = next().parse().unwrap_or_default();
        } else if v == "-s" && !last_arg {
            config.host_socket = next();
        } else if v == "-r" && !last_arg {
            config.repeat = next().parse().unwrap_or_default();
        } else if v == "-i" && !last_arg {
            config.interval = next().parse::<i64>().unwrap_or_default() * 1000000;
        } else if v == "-n" && !last_arg {
            config.conn_info.input_db_num = next().parse().unwrap_or_default();
        } else if v == "--no-auth-warning" {
            config.no_auth_warning = 1;
        } else if v == "--askpass" {
            config.ask_pass = 1;
        } else if (v == "-a" || v == "--pass") && !last_arg {
            config.conn_info.auth = next();
        } else if v == "--user" && !last_arg {
            config.conn_info.user = next();
        } else if v == "-u" && !last_arg {
            // TODO
        } else if v == "--raw" {
            config.output = consts::OUTPUT_RAW;
        } else if v == "--no-raw" {
            config.output = consts::OUTPUT_STANDARD;
        } else if v == "--quoted-input" {
            config.quoted_input = 1;
        } else if v == "--csv" {
            config.output = consts::OUTPUT_CSV;
        } else if v == "--json" {
            if config.resp3 == 0 {
                config.resp3 = 2;
            }
            config.output = consts::OUTPUT_JSON;
        } else if v == "--quoted-json" {
            if config.resp3 == 0 {
                config.resp3 = 2;
            }
            config.output = consts::OUTPUT_QUOTED_JSON;
        } else if v == "--latency" {
            config.latency_mode = 1;
        } else if v == "--latency-dist" {
            config.latency_dist_mode = 1;
        } else if v == "--mono" {
            *spectrum_palette = spectrum_palette_mono.to_vec();
            *spectrum_palette_size = spectrum_palette_mono_size;
        } else if v == "--latency-history" {
            config.latency_mode = 1;
            config.latency_history = 1;
        } else if v == "--lru-test" && !last_arg {
            config.lru_test_mode = 1;
            config.lru_test_sample_size = next().parse().unwrap_or_default();
        } else if v == "--slave" {
            config.slave_mode = 1;
        } else if v == "--replica" {
            config.slave_mode = 1;
        } else if v == "--stat" {
            config.stat_mode = 1;
        } else if v == "--scan" {
            config.scan_mode = 1;
        } else if v == "--pattern" && !last_arg {
            config.pattern = next();
        } else if v == "--quoted-pattern" && !last_arg {
            // TODO
            config.pattern = next();
            if config.pattern.is_empty() {
                eprintln!("Invalid quoted string specified for --quoted-pattern.");
                exit(1);
            }
        } else if v == "--intrinsic-latency" && !last_arg {
            config.intrinsic_latency_mode = 1;
            config.intrinsic_latency_duration = next().parse().unwrap_or_default();
        } else if v == "--rdb" && !last_arg {
            config.get_rdb_mode = 1;
            config.rdb_filename = next();
        } else if v == "--functions-rdb" && !last_arg {
            config.get_functions_rdb_mode = 1;
            config.rdb_filename = next();
        } else if v == "--pipe" {
            config.pipe_mode = 1;
        } else if v == "--pipe-timeout" && !last_arg {
            config.pipe_timeout = next().parse().unwrap_or_default();
        } else if v == "--bigkeys" {
            config.big_keys = 1;
        } else if v == "--memkeys" {
            config.mem_keys = 1;
            config.mem_keys_samples = 0;
        } else if v == "--memkeys-samples" {
            config.mem_keys = 1;
            config.mem_keys_samples = next().parse().unwrap_or_default();
        } else if v == "--hotkeys" {
            config.hot_keys = 1;
        } else if v == "--eval" && !last_arg {
            config.eval = next();
        } else if v == "--ldb" {
            config.eval_ldb = 1;
            config.output = consts::OUTPUT_RAW;
        } else if v == "--ldb-sync-mode" {
            config.eval_ldb = 1;
            config.eval_ldb_sync = 1;
            config.output = consts::OUTPUT_RAW;
        } else if v == "-c" {
            config.cluster_mode = 1;
        } else if v == "-d" && !last_arg {
            config.mb_delim = next();
        } else if v == "-D" && !last_arg {
            config.cmd_delim = next();
        } else if v == "-e" {
            config.set_errcode = 1;
        } else if v == "--verbose" {
            config.verbose = 1;
        } else if v == "--cluster" && !last_arg {
            if !config.cluster_manager_command.name.is_empty() {
                print_usage(1);
            }
            // TODO
        } else if v == "--cluster" && last_arg {
            print_usage(1);
        } else if v == "--cluster-only-masters" {
            config.cluster_manager_command.flags |= consts::CLUSTER_MANAGER_CMD_FLAG_MASTERS_ONLY;
        } else if v == "--cluster-only-replicas" {
            config.cluster_manager_command.flags |= consts::CLUSTER_MANAGER_CMD_FLAG_SLAVES_ONLY;
        } else if v == "--cluster-replicas" && !last_arg {
            config.cluster_manager_command.replicas = next().parse().unwrap_or_default();
        } else if v == "--cluster-master-id" && !last_arg {
            config.cluster_manager_command.master_id = next();
        }
    }
}