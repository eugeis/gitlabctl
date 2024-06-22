use std::path::Path;
use std::fs;

use crate::common::Result;
use crate::gitlab::GroupNode;
use crate::handler::Handler;

use tera::{Context, Tera};

const CLONE_SH: &str = include_str!("../templates/clone.sh");
const PULL_SH: &str = include_str!("../templates/pull.sh");
const STATUS_SH: &str = include_str!("../templates/status.sh");
const SCRIPTS_SH: &str = include_str!("../templates/scripts.sh");
const MACROS_GIT_SCRIPT_SH: &str = include_str!("../templates/macros/git_script.sh");
const MACROS_GIT_SCRIPT_REPO_SH: &str = include_str!("../templates/macros/git_script_repo.sh");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        let _ = tera.add_raw_template("macros/git_script.sh", MACROS_GIT_SCRIPT_SH).expect("can't parse template");
        let _ = tera.add_raw_template("macros/git_script_repo.sh", MACROS_GIT_SCRIPT_REPO_SH).expect("can't parse template");
        let _ = tera.add_raw_template("clone.sh", CLONE_SH).expect("can't parse template");
        let _ = tera.add_raw_template("pull.sh", PULL_SH).expect("can't parse template");
        let _ = tera.add_raw_template("status.sh", STATUS_SH).expect("can't parse template");
        let _ = tera.add_raw_template("scripts.sh", SCRIPTS_SH).expect("can't parse template");

        /*
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => {
                println!("template registered: {:?}", t);
                t
            },
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };*/

        tera
    };
}


pub struct Generator {
    pub gitlab_token_file: String
}

impl Handler for Generator {
    fn on_node(&self, target_path: &Path, item: &GroupNode) -> Result<()> {
        let mut context = Context::new();
        context.insert("groupNode", item);
        context.insert("gitlabTokenFile", &self.gitlab_token_file);

        for template in ["clone.sh","pull.sh","status.sh","scripts.sh"] {
            let target_file_path_buf = Path::new(target_path).join(template);
            let target_file_path = target_file_path_buf.as_path();

            let target_file = fs::File::create(target_file_path)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                match fs::set_permissions(target_file_path.as_os_str(), fs::Permissions::from_mode(0o700)) {
                    Err(e)  => {
                        println!("can't set permissions to file: {}", e);
                    }
                    _ => {}
                }
            }

            match TEMPLATES.render_to(template, &context, target_file) {
                Err(e) => {
                    println!("can't render file: {}", e);
                }
                _ => {}
            };
        }
        Ok(())
    }
}