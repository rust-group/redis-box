pub struct CliConnInfo {
    pub host_ip: String,
    pub host_port: i32,
    pub input_db_num: i32,
    pub auth: String,
    pub user: String,
}

impl CliConnInfo {
    pub fn new() -> CliConnInfo {
        CliConnInfo {
            host_ip: "127.0.0.1".to_string(),
            host_port: 6379,
            input_db_num: 0,
            auth: "".to_string(),
            user: "".to_string(),
        }
    }
}