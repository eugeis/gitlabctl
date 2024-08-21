use std::fs;
use std::path::Path;

use crate::common::Result;
use crate::gitlab::GroupNode;

pub struct FsModelHandler {
    pub output_dir: String,

    pub handle_children: bool,
    pub handler: Box<dyn Handler>,
}

impl FsModelHandler {
    pub fn on_node(&self, item: &GroupNode) -> Result<()> {
        let target_path_buf = Path::new(&self.output_dir).join(&item.relative_root_path);

        let target_path = target_path_buf.as_path();
        fs::create_dir_all(target_path)?;

        let result = self.handler.on_node(target_path, item);

        if result.is_err() {
            return result;
        }

        if self.handle_children {
            for child in &item.children {
                let result = self.on_node(child);
                if result.is_err() {
                    return result;
                }
            }
        }
        Ok(())
    }
}

pub trait Handler {
    fn on_node(&self, target_path: &Path, node: &GroupNode) -> Result<()>;
}

pub struct CompositeHandler {
    pub handlers: Vec<Box<dyn Handler>>,
}

impl Handler for CompositeHandler {
    fn on_node(&self, target_path: &Path, node: &GroupNode) -> Result<()> {
        for handler in &self.handlers {
            handler.on_node(target_path, node)?
        }
        Ok(())
    }
}
