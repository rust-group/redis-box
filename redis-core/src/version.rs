const REDIS_VERSION: &str = "7.0.5";
const REDIS_GIT_SHA1: &str = "";
const REDIS_GIT_DIRTY: u8 = 0;

pub fn check_rdb_version() -> String {
    let mut version = format!("redis-check-rdb {REDIS_VERSION}");
    if !REDIS_GIT_SHA1.trim().is_empty() {
        version = format!("{version} (git:{REDIS_GIT_SHA1}");
        if REDIS_GIT_DIRTY > 0 {
            version.push_str("-dirty");
        }
        version.push_str(")");
    }
    version
}