use std::path::Path;
use std::fs;

use crate::common::Result;
use crate::gitlab::GroupNode;
use crate::handler::Handler;

use include_dir::{include_dir, Dir};
use tera::{Tera, Context};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use crate::ci_parser::GitlabCi;

const TEMPLATES_FOLDER: Dir = include_dir!("templates");

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();

        let mut templates: HashMap<String, String> = HashMap::new();
        
        fn load_dir(dir: &Dir, templates: &mut HashMap<String, String>) {
            for entry in dir.entries().into_iter() {
                match entry.as_file() {
                    Some(file) => {
                        let file_name = file.path().to_string_lossy().to_string();
                        let content = file.contents_utf8().expect("Unable to open file");
                        let template_name = file_name.to_string();
                        println!("load template: {}", template_name);
                        templates.insert(template_name, content.to_string());
                    }
                    None => {
                        match entry.as_dir() {
                            Some(dir) => {
                                load_dir(dir, templates);
                            }
                            None => {}
                            
                        }
                    }
                }
            }
        }
        load_dir(&TEMPLATES_FOLDER, &mut templates);

        //println!("register templates first time in order to load templates without dependencies");
        for (name, content) in &templates {
            match tera.add_raw_template(name.as_str() , content.as_str()) {
                Err(_) => {
                    //println!("can't add template, the first time: {}", e);
                },
                _ => {}
            }
        }

        //println!("register templates second time in order to resolve dependencies");
        for (name, content) in &templates {
            match tera.add_raw_template(name.as_str() , content.as_str()) {
                Err(e) => {
                    println!("can't add template, the second time: {}", e);
                }
                _ => {}
            }
        }
        tera
    };
}

pub struct GitScriptGenerator {
    pub templates: Vec<String>,
    pub gitlab_token_file: String
}

impl Handler for GitScriptGenerator {
    fn on_node(&self, target_path: &Path, item: &GroupNode) -> Result<()> {
        let mut context = Context::new();
        context.insert("groupNode", item);
        context.insert("gitlabTokenFile", &self.gitlab_token_file);

        for template in &self.templates {
            let target_file_path_buf = Path::new(target_path).join(template);
            let target_file_path = target_file_path_buf.as_path();

            let target_file = File::create(target_file_path)?;

            make_executable(target_file_path);

            match TERA.render_to(format!("git/{}", &template).as_str(), &context, target_file) {
                Err(e) => {
                    println!("can't render file: {}", e);
                }
                _ => {}
            };
        }
        Ok(())
    }
}

pub struct GitCiScriptGenerator {
    ci: GitlabCi,
    output_dir: String,
}

impl GitCiScriptGenerator {
    pub fn new(ci: GitlabCi, output_dir: &str) -> Self {
        GitCiScriptGenerator {
            ci,
            output_dir: output_dir.to_string(),
        }
    }

    pub fn gen(&self) {
        // Create the output directory if it doesn't exist
        if !Path::new(&self.output_dir).exists() {
            fs::create_dir_all(&self.output_dir)
                .expect("Unable to create output directory");
        }

        // Create empty script files for unresolved jobs
        for job_name in self.ci.collect_unresolved_extends().iter() {
            let target_file_path_buf = Path::new(&self.output_dir).join(format!("{}.sh", job_name));
            if !target_file_path_buf.exists() {
                File::create(&target_file_path_buf)
                    .expect("Unable to create empty script file for unresolved job");
                println!("Empty script file created: {:?}", &target_file_path_buf);
            }
        }

        // Generate script files for all defined jobs
        for (job_name, job) in self.ci.jobs.iter() {
            let mut context = Context::new();
            context.insert("job_name", job_name);
            context.insert("job", job);
            context.insert("is_template", &job_name.starts_with("."));
            context.insert("variables", &job.variables_as_strings());

            let target_file_path_buf = Path::new(&self.output_dir).join(format!("{}.sh", job_name));

            let target_file = File::create(&target_file_path_buf)
                .expect("Unable to create job file");

            make_executable(&target_file_path_buf);

            match TERA.render_to("ci/job.sh", &context, &target_file) {
                Err(e) => {
                    println!("can't render file: {}", e);
                }
                _ => {
                    println!("file generated: {:?}", &target_file_path_buf);
                }
            };
        }
    }
}

pub fn make_executable(target_file_path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match fs::set_permissions(target_file_path.as_os_str(), fs::Permissions::from_mode(0o700)) {
            Err(e) => {
                println!("can't set permissions to file: {}", e);
            }
            _ => {}
        }
    }
}