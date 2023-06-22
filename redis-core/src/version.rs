const REDIS_VERSION: &str = "7.0.11";
const REDIS_GIT_SHA1: &str = "bdb1e6cc";
const REDIS_GIT_DIRTY: u8 = 1;

pub fn check_rdb_version() -> String {
    version("redis-check-rdb")
}

pub fn redis_cli_version() -> String {
    version("redis-cli")
}

fn version(name: &str) -> String {
    let mut version = format!("{name} {REDIS_VERSION}");
    if !REDIS_GIT_SHA1.trim().is_empty() {
        version = format!("{version} (git:{REDIS_GIT_SHA1}");
        if REDIS_GIT_DIRTY > 0 {
            version.push_str("-dirty");
        }
        version.push_str(")");
    }
    version
}