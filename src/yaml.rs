use std::fs;
use std::path::Path;

use crate::common::Result;
use crate::gitlab::GroupNode;
use crate::handler::Handler;

pub struct YamlWriter {
    pub model_file_name: String,
}

impl Handler for YamlWriter {
    fn on_node(&self, target_path: &Path, item: &GroupNode) -> Result<()> {
        write_yaml(
            Path::new(target_path)
                .join(self.build_model_file_name(item))
                .as_path(),
            item,
        )
    }
}

impl YamlWriter {
    fn build_model_file_name(&self, item: &GroupNode) -> String {
        if !self.model_file_name.is_empty() {
            self.model_file_name.clone()
        } else {
            format!("{:?}.yaml", item.group.id.to_string())
        }
    }
}

pub fn write_yaml<T>(target_file_path: &Path, item: T) -> Result<()>
where
    T: serde::Serialize,
{
    let yaml = serde_yaml::to_string(&item)?;
    fs::write(target_file_path, yaml)?;
    println!("File {:?} written", target_file_path);
    Ok(())
}
