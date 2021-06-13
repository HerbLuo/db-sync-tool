use std::path::{Path, PathBuf};

use crate::types::{ZzErrors};

pub fn get_base_dir() -> Result<PathBuf, ZzErrors> {
    Path::new("./").canonicalize().map_err(|e| ZzErrors::GetBaseDirError(e))
}
