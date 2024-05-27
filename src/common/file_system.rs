/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::collections::HashMap;

use crate::error::RResult;
use tokio::io::AsyncReadExt;

#[derive(Debug, Default, Clone)]
pub(crate) struct FileSystem {
    file_type: FileSystemType,
    base_path: String,
    is_root_dir: bool,
}
#[derive(Debug, Clone)]
enum FileSystemType {
    Dir {
        file_path_map: HashMap<String, std::fs::FileType>,
        index_file: String,
    },
    File {
        file_name: String,
    }
}
impl Default for FileSystemType {
    fn default() -> Self {
        FileSystemType::File { file_name: "/".to_string() }
    }
}
impl FileSystem {
    pub(crate) fn new(base_path: &str, index_file: &str) -> RResult<Self> {
        let mut file_path_map = HashMap::new();
        let file_type = if std::fs::metadata(base_path)?.is_dir() {
            for entry in std::fs::read_dir(base_path)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                let path = entry.path();
                let is_dir = file_type.is_dir();
                let path = path.to_str().unwrap();
                file_path_map.insert(path.to_string(), file_type);
                if is_dir {
                    FileSystem::recursive_read(path, &mut file_path_map)?
                }
            }
            let index_file = base_path.to_string() + "/" + index_file;
            FileSystemType::Dir { file_path_map, index_file }
        } else {
            FileSystemType::File { file_name: base_path.to_string() }
        };
        Ok(Self { 
            file_type, 
            base_path: base_path.to_string(),
            is_root_dir: false,
        })
    }
    fn recursive_read(path: &str, map: &mut HashMap<String, std::fs::FileType>) -> RResult<()> {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();
            let is_dir = file_type.is_dir();
            let path = path.to_str().unwrap();
            map.insert(path.to_string(), file_type);
            if is_dir {
                FileSystem::recursive_read(path, map)?
            }
        }
        Ok(())
    }
    pub(crate) fn is_root_dir(&self) -> bool {
        self.is_root_dir
    }
    pub(crate) fn base_path(&self) -> &str {
        &self.base_path
    }
    pub(crate) async fn read_file(&self, file_path: &str) -> RResult<Option<Vec<u8>>> {
        match &self.file_type {
            FileSystemType::File { file_name } => {
                let mut buffer = Vec::new();
                let mut file = tokio::fs::File::open(file_name).await?;
                file.read_to_end(&mut buffer).await?;
                return Ok(Some(buffer));
            }
            FileSystemType::Dir { file_path_map, index_file } => {
                let is_dir = match file_path_map.get(file_path) {
                    Some(file_type) => {
                        file_type.is_dir()
                    }
                    None => {
                        if file_path == self.base_path {
                            return Box::pin(self.read_file(&index_file)).await;
                        }
                        return Ok(None);
                    }
                };
                if is_dir {
                    let mut dir = tokio::fs::read_dir(file_path).await?;
                    let mut buffer = String::new();
                    loop {
                        match dir.next_entry().await? {
                            Some(sub_f_d) => {
                                let path_buf = sub_f_d.path();
                                let path = path_buf.to_str().unwrap();
                                buffer += path;
                                buffer += ",";
                            }
                            None => { break; }
                        }
                    }
                    if !buffer.is_empty() {
                        buffer = (&buffer[0..buffer.len()-1]).to_string();
                    }
                    return Ok(Some(buffer.as_bytes().to_vec()));
                } else {
                    let mut buffer = Vec::new();
                    let mut file = tokio::fs::File::open(file_path).await?;
                    file.read_to_end(&mut buffer).await?;
                    return Ok(Some(buffer));
                }
            }
        }
        
    }
}