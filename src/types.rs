#[derive(Debug)]
pub enum ZzErrors {
    ParseConfigError(serde_json::Error),
    IllegalConfig(String),
    GetBaseDirError(std::io::Error),
    IoError(String),
    ConnectError(String),
    ExecSqlError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    #[serde(alias = "drop-create")]
    DropCreate,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnyStr {
    #[serde(alias = "*")]
    AnyStr
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SyncConfigTables {
    Any(AnyStr),
    Tables(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAddr {
    pub hostname: String,
    pub username: String,
    pub db: String,
    pub port: u16,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DbConfig {
    Path(String),
    ClientAddr(ClientAddr)
}

fn default_buffer_size() -> u32 { 1024 }
fn default_skip_sync_if_table_not_exist() -> bool { true }

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncConfig {
    pub name: Option<String>,
    pub mode: Mode,
    pub tables: SyncConfigTables,
    pub from: DbConfig,
    pub to: DbConfig,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: u32,
    #[serde(default = "default_skip_sync_if_table_not_exist")]
    pub skip_sync_if_table_not_exist: bool,
    // pub transactional: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
  pub name: String,
  pub def: Option<bool>,
  pub syncs: Vec<SyncConfig>,
}

#[derive(Debug)]
pub struct SqlGroup {
    pub schema: String,
    pub sqls: Vec<String>,
}

pub type SqlGroups = Vec<SqlGroup>;
