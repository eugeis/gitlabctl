use std::path::Path;
use std::fs;
use log::{warn};
use crate::common::{Result};
use crate::gitlab::GroupNode;
use crate::handler::Handler;

use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}


pub struct Generator {
}

impl Handler for Generator {
    fn on_node(&self, target_path: &Path, item: &GroupNode) -> Result<()> {
        let mut context = Context::new();
        context.insert("groupNode", item);

        for template in ["clone.sh","pull.sh","status.sh"] {
            let target_file_path_buf = Path::new(target_path).join(template);
            let target_file_path = target_file_path_buf.as_path();

            let target_file = fs::File::create(target_file_path)?;

            match TEMPLATES.render_to(template, &context, target_file) {
                Err(e) => {
                    warn!("Error: {}", e);
                }
                _ => {}
            };
        }
        Ok(())
    }
}