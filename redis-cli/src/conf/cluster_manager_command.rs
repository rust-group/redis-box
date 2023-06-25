use super::consts;

pub struct ClusterManagerCommand {
    pub name: String,
    pub argc: i32,
    pub argv: Vec<String>,
    pub stdin_arg: String,
    pub flags: i32,
    pub replicas: i32,
    pub from: String,
    pub to: String,
    pub weight: Vec<String>,
    pub weight_argc: i32,
    pub master_id: String,
    pub slots: i32,
    pub timeout: i32,
    pub pipeline: i32,
    pub threshold: f32,
    pub backup_dir: String,
    pub from_user: String,
    pub from_pass: String,
    pub from_ask_pass: i32,
}

impl ClusterManagerCommand {
    pub fn new() -> ClusterManagerCommand {
        ClusterManagerCommand {
            name: "".to_string(),
            argc: 0,
            argv: vec![],
            stdin_arg: "".to_string(),
            flags: 0,
            replicas: 0,
            from: "".to_string(),
            to: "".to_string(),
            weight: vec![],
            weight_argc: 0,
            master_id: "".to_string(),
            slots: 0,
            timeout: consts::CLUSTER_MANAGER_MIGRATE_TIMEOUT,
            pipeline: consts::CLUSTER_MANAGER_MIGRATE_PIPELINE,
            threshold: consts::CLUSTER_MANAGER_REBALANCE_THRESHOLD,
            backup_dir: "".to_string(),
            from_user: "".to_string(),
            from_pass: "".to_string(),
            from_ask_pass: 0,
        }
    }
}