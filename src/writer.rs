
use std::fs;
use std::path::Path;
use log::info;
use crate::gitlab::{GroupNode};
use crate::common::{Result};

pub struct YamlModelWriter {
    pub output_dir: String,
    pub groups_dir: String,
    pub model_file_name: String,

    pub write_group: bool,
    pub write_model: bool,
    pub handle_children: bool,

    pub model_files: Vec<String>
}

impl YamlModelWriter {
    pub fn on_node(&self, item: &GroupNode) -> Result<()>{
        if self.write_model {
            let file_name = self.build_model_file_name(&item).clone();

            let result = write_yaml(Path::new(&self.output_dir).
                join(&item.relative_root_path).
                join(file_name).as_path(), item);
            if result.is_err() {
                return result
            }
        }

        self.on_group(&item.group)?;

        if self.handle_children {
            for child in &item.children {
                let result = self.on_node(child);
                if result.is_err() {
                    return result
                }
            }
        }
        Ok(())
    }

    fn build_model_file_name(&self, item: &GroupNode) -> String {
        let file_name = if self.model_file_name != "" {
            self.model_file_name.clone()
        } else {
            format!("{:?}.yaml", item.group.id.value().to_string())
        };
        file_name
    }

    fn on_group(&self, item: &gitlab::GroupDetail) -> Result<()>{
        if self.write_group {
            let target_file_path = Path::new(&self.output_dir).
                join( &self.groups_dir).
            join(format!("{:?}.yaml", item.id.value()));

            write_yaml(target_file_path.as_path(), item)?;
        }
        return Ok(())
    }
}

fn write_yaml<T>(target_file_path: &Path, item: T) -> Result<()> where T:serde::Serialize{
    let yaml = serde_yaml::to_string(&item)?;
    fs::create_dir_all(target_file_path.parent().unwrap())?;
    fs::write(target_file_path, yaml)?;
    info!("File {:?} written", target_file_path);
    return Ok(());
}