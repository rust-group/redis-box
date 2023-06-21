use std::process::exit;
use redis_core::version;

pub(crate) fn print_usage(code: i32) {
    const REDIS_CLI_AUTH_ENV: &str = "REDISCLI_AUTH";
    const REDIS_CLI_DEFAULT_PIPE_TIMEOUT: i32 = 30;
    const USE_OPENSSL: bool = false;
    const TLS1_3_VERSION: bool = false;
    let redis_cli_version = version::redis_cli_version();

    let mut usage = format!("{redis_cli_version}

Usage: redis-cli [OPTIONS] [cmd [arg [arg ...]]]
  -h <hostname>      Server hostname (default: 127.0.0.1).
  -p <port>          Server port (default: 6379).
  -s <socket>        Server socket (overrides hostname and port).
  -a <password>      Password to use when connecting to the server.
                     You can also use the {REDIS_CLI_AUTH_ENV} environment
                     variable to pass this password more safely
                     (if both are used, this argument takes precedence).
  --user <username>  Used to send ACL style 'AUTH username pass'. Needs -a.
  --pass <password>  Alias of -a for consistency with the new --user option.
  --askpass          Force user to input password with mask from STDIN.
                     If this argument is used, '-a' and {REDIS_CLI_AUTH_ENV}
                     environment variable will be ignored.
  -u <uri>           Server URI.
  -r <repeat>        Execute specified command N times.
  -i <interval>      When -r is used, waits <interval> seconds per command.
                     It is possible to specify sub-second times like -i 0.1.
                     This interval is also used in --scan and --stat per cycle.
                     and in --bigkeys, --memkeys, and --hotkeys per 100 cycles.
  -n <db>            Database number.
  -2                 Start session in RESP2 protocol mode.
  -3                 Start session in RESP3 protocol mode.
  -x                 Read last argument from STDIN (see example below).
  -X                 Read <tag> argument from STDIN (see example below).
  -d <delimiter>     Delimiter between response bulks for raw formatting (default: \\n).
  -D <delimiter>     Delimiter between responses for raw formatting (default: \\n).
  -c                 Enable cluster mode (follow -ASK and -MOVED redirections).
  -e                 Return exit error code when command execution fails.");
    if USE_OPENSSL {
        usage += "
  --tls              Establish a secure TLS connection.
  --sni <host>       Server name indication for TLS.
  --cacert <file>    CA Certificate file to verify with.
  --cacertdir <dir>  Directory where trusted CA certificates are stored.
                     If neither cacert nor cacertdir are specified, the default
                     system-wide trusted root certs configuration will apply.
  --insecure         Allow insecure TLS connection by skipping cert validation.
  --cert <file>      Client certificate to authenticate with.
  --key <file>       Private key file to authenticate with.
  --tls-ciphers <list> Sets the list of preferred ciphers (TLSv1.2 and below)
                     in order of preference from highest to lowest separated by colon (\":\").
                     See the ciphers(1ssl) manpage for more information about the syntax of this string.";

        if TLS1_3_VERSION {
            usage += "
  --tls-ciphersuites <list> Sets the list of preferred ciphersuites (TLSv1.3)
                     in order of preference from highest to lowest separated by colon (\":\").
                     See the ciphers(1ssl) manpage for more information about the syntax of this string,
                     and specifically for TLSv1.3 ciphersuites."
        }
    }

    usage.push_str(format!("
  --raw              Use raw formatting for replies (default when STDOUT is
                     not a tty).
  --no-raw           Force formatted output even when STDOUT is not a tty.
  --quoted-input     Force input to be handled as quoted strings.
  --csv              Output in CSV format.
  --json             Output in JSON format (default RESP3, use -2 if you want to use with RESP2).
  --quoted-json      Same as --json, but produce ASCII-safe quoted strings, not Unicode.
  --show-pushes <yn> Whether to print RESP3 PUSH messages.  Enabled by default when
                     STDOUT is a tty but can be overridden with --show-pushes no.
  --stat             Print rolling stats about server: mem, clients, ...
  --latency          Enter a special mode continuously sampling latency.
                     If you use this mode in an interactive session it runs
                     forever displaying real-time stats. Otherwise if --raw or
                     --csv is specified, or if you redirect the output to a non
                     TTY, it samples the latency for 1 second (you can use
                     -i to change the interval), then produces a single output
                     and exits.
  --latency-history  Like --latency but tracking latency changes over time.
                     Default time interval is 15 sec. Change it using -i.
  --latency-dist     Shows latency as a spectrum, requires xterm 256 colors.
                     Default time interval is 1 sec. Change it using -i.
  --lru-test <keys>  Simulate a cache workload with an 80-20 distribution.
  --replica          Simulate a replica showing commands received from the master.
  --rdb <filename>   Transfer an RDB dump from remote server to local file.
                     Use filename of \"-\" to write to stdout.
  --functions-rdb <filename> Like --rdb but only get the functions (not the keys)
                     when getting the RDB dump file.
  --pipe             Transfer raw Redis protocol from stdin to server.
  --pipe-timeout <n> In --pipe mode, abort with error if after sending all data.
                     no reply is received within <n> seconds.
                     Default timeout: {REDIS_CLI_DEFAULT_PIPE_TIMEOUT}. Use 0 to wait forever.
  --bigkeys          Sample Redis keys looking for keys with many elements (complexity).
  --memkeys          Sample Redis keys looking for keys consuming a lot of memory.
  --memkeys-samples <n> Sample Redis keys looking for keys consuming a lot of memory.
                     And define number of key elements to sample
  --hotkeys          Sample Redis keys looking for hot keys.
                     only works when maxmemory-policy is *lfu.
  --scan             List all keys using the SCAN command.
  --pattern <pat>    Keys pattern when using the --scan, --bigkeys or --hotkeys
                     options (default: *).
  --quoted-pattern <pat> Same as --pattern, but the specified string can be
                         quoted, in order to pass an otherwise non binary-safe string.
  --intrinsic-latency <sec> Run a test to measure intrinsic system latency.
                     The test will run for the specified amount of seconds.
  --eval <file>      Send an EVAL command using the Lua script at <file>.
  --ldb              Used with --eval enable the Redis Lua debugger.
  --ldb-sync-mode    Like --ldb but uses the synchronous Lua debugger, in
                     this mode the server is blocked and script changes are
                     not rolled back from the server memory.
  --cluster <command> [args...] [opts...]
                     Cluster Manager command and arguments (see below).
  --verbose          Verbose mode.
  --no-auth-warning  Don't show warning message when using password on command
                     line interface.
  --help             Output this help and exit.
  --version          Output version and exit.
").as_str());

    /* Using another fprintf call to avoid -Woverlength-strings compile warning */
    usage += "
Cluster Manager Commands:
  Use --cluster help to list all available cluster manager commands.

Examples:
  cat /etc/passwd | redis-cli -x set mypasswd
  redis-cli -D \"\" --raw dump key > key.dump && redis-cli -X dump_tag restore key2 0 dump_tag replace < key.dump
  redis-cli -r 100 lpush mylist x
  redis-cli -r 100 -i 1 info | grep used_memory_human:
  redis-cli --quoted-input set '\"null-\\x00-separated\"' value
  redis-cli --eval myscript.lua key1 key2 , arg1 arg2 arg3
  redis-cli --scan --pattern '*:12345*'

  (Note: when using --eval the comma separates KEYS[] from ARGV[] items)

When no command is given, redis-cli starts in interactive mode.
Type \"help\" in interactive mode for information on available commands
and settings.
";

    if code == 0 {
        println!("{}", usage);
    } else {
        eprint!("{}", usage);
    }
    exit(code);
}