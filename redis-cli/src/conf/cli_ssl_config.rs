pub struct CliSSLConfig {
    pub sni: String,
    pub ca_cert: String,
    pub ca_cert_dir: String,
    pub skip_cert_verify: i32,
    pub cert: String,
    pub key: String,
    pub ciphers: String,
    pub cipher_suites: String,
}

impl CliSSLConfig {
    pub fn new() -> CliSSLConfig {
        CliSSLConfig {
            sni: "".to_string(),
            ca_cert: "".to_string(),
            ca_cert_dir: "".to_string(),
            skip_cert_verify: 0,
            cert: "".to_string(),
            key: "".to_string(),
            ciphers: "".to_string(),
            cipher_suites: "".to_string(),
        }
    }
}