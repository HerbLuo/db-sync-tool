#[derive(Debug)]
pub enum ZzErrors {
    ParseConfigError(serde_json::Error),
    IllegalConfig(String),
    GetBaseDirError(std::io::Error),
    IoError(String),
    ConnectError(String)
}

#[derive(Deserialize, Debug)]
pub enum Mode {
    #[serde(alias = "drop-create")]
    DropCreate,
}

#[derive(Deserialize, Debug)]
pub enum AnyStr {
    #[serde(alias = "*")]
    AnyStr
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SyncConfigTables {
    Any(AnyStr),
    Tables(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct ClientAddr {
    pub hostname: String,
    pub username: String,
    pub db: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum DbConfig {
    Path(String),
    ClientAddr(ClientAddr)
}

fn default_buffer_size() -> u32 { 104 }

#[derive(Deserialize, Debug)]
pub struct SyncConfig {
    pub mode: Mode,
    pub tables: SyncConfigTables,
    pub from: DbConfig,
    pub to: DbConfig,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: u32,
}

#[derive(Debug)]
pub struct SqlGroup {
    pub schema: String,
    pub sqls: Vec<String>,
}

pub type SqlGroups = Vec<SqlGroup>;
