use std::path::Path;
use std::fs;
use log::info;
use crate::common::{Result};
use crate::gitlab::GroupNode;
use crate::handler::Handler;

pub struct YamlWriter {
    pub model_file_name: String,
    pub model_files: Vec<String>
}

impl Handler for YamlWriter {
    fn on_node(&self, target_path: &Path, item: &GroupNode) -> Result<()> {
        write_yaml(Path::new(target_path).
            join(self.build_model_file_name(item)).as_path(), item)
    }
}

impl YamlWriter {
    fn build_model_file_name(&self, item: &GroupNode) -> String {
        let file_name = if self.model_file_name != "" {
            self.model_file_name.clone()
        } else {
            format!("{:?}.yaml", item.group.id.value().to_string())
        };
        file_name
    }
}

pub fn write_yaml<T>(target_file_path: &Path, item: T) -> Result<()> where T:serde::Serialize{
    let yaml = serde_yaml::to_string(&item)?;
    fs::write(target_file_path, yaml)?;
    info!("File {:?} written", target_file_path);
    return Ok(());
}