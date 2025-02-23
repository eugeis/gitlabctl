use std::fs;
use std::path::Path;

use crate::common::Result;
use crate::gitlab::GroupNode;
use crate::handler::Handler;

use crate::ci_parser::{variables_as_string_fill, GitlabCi, GitlabJob, ParallelField, VariableValue};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use tera::{Context, Tera};

const TEMPLATES_FOLDER: Dir = include_dir!("templates");

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = Tera::default();

        let mut templates: HashMap<String, String> = HashMap::new();

        fn load_dir(dir: &Dir, templates: &mut HashMap<String, String>) {
            for entry in dir.entries().iter() {
                match entry.as_file() {
                    Some(file) => {
                        let file_name = file.path().to_string_lossy().to_string();
                        let content = file.contents_utf8().unwrap_or("");

                        if !content.is_empty() {
                            let template_name = file_name.to_string();
                            //println!("load template: {}", template_name);
                            templates.insert(template_name, content.to_string());
                        }
                    }
                    None => {
                         if let Some(dir) = entry.as_dir() {
                            load_dir(dir, templates);
                         }
                    }
                }
            }
        }
        load_dir(&TEMPLATES_FOLDER, &mut templates);

        //println!("register templates first time in order to load templates without dependencies");
        for (name, content) in &templates {
             if tera.add_raw_template(name.as_str() , content.as_str()).is_err() {
                 //println!("can't add template, the first time: {}", e);
             }
        }

        //println!("register templates second time in order to resolve dependencies");
        for (name, content) in &templates {
            if let Err(e) = tera.add_raw_template(name.as_str() , content.as_str()) {
               println!("can't add template, the second time: {}", e);
             }
        }
        tera
    };
}

pub struct GitScriptGenerator {
    pub templates: Vec<String>,
    pub gitlab_token_file: String,
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

            if let Err(e) =
                TERA.render_to(format!("git/{}", &template).as_str(), &context, target_file)
            {
                println!("can't render file: {}", e);
            };
        }
        Ok(())
    }
}

pub struct GitlabCiScriptGenerator {
    ci: GitlabCi,
    output_dir: String,
}

impl GitlabCiScriptGenerator {
    pub fn new(ci: GitlabCi, output_dir: &str) -> Self {
        GitlabCiScriptGenerator {
            ci,
            output_dir: output_dir.to_string(),
        }
    }

    pub fn gen(&self) {
        // Create the output directory if it doesn't exist
        if !Path::new(&self.output_dir).exists() {
            fs::create_dir_all(&self.output_dir).expect("Unable to create output directory");
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

        if let Some(matrices) = &self.ci.matrices {
            // Generate script files for all defined jobs
            for (matrix_name, matrix) in matrices.iter() {
                for (index, variables) in matrix.iter().enumerate() {
                    self.gen_matrix(&format!("{}_{}", matrix_name, index), &variables);
                }
            }
        }

        // Generate script files for all defined jobs
        for (job_name, job) in self.ci.jobs.iter() {
            if let Some(parallel) = &job.parallel {
                let (matrix_name, matrix_length) = match parallel {
                    ParallelField::Reference(reference) => {
                        // Only first matrix is supported for now
                        let matrix_name = reference.matrix.first().unwrap();
                        if let Some(matrix) = self.ci.matrices.as_ref().and_then(|matrices| {
                            matrices.get(matrix_name)
                        }) {
                            (matrix_name.clone(), matrix.len())
                        } else {
                            (matrix_name.clone(), 0)
                        }
                    }
                    ParallelField::Matrix(parallel_matrix) => {
                        let matrix_name = format!("{}_matrix", job_name);
                        for (index, variables) in parallel_matrix.matrix.iter().enumerate() {
                            self.gen_matrix(&format!("{}_{}", matrix_name, index), &variables);
                        }
                        (matrix_name, parallel_matrix.matrix.len())
                    }
                };

                for index in 0..matrix_length {
                    let job_name_parallel = format!("{}_{}", job_name, index);
                    self.gen_job(&job_name_parallel, job, format!("{}_{}", matrix_name, index).as_str());
                }
            } else {
                self.gen_job(&job_name, job, "");
            }
        }
    }

    fn gen_matrix(&self, matrix_name: &String, matrix: &&BTreeMap<String, VariableValue>) {
        let mut context = Context::new();
        context.insert("matrix_name", &matrix_name);
        context.insert("is_template", &true);

        let mut string_vars = Vec::new();
        variables_as_string_fill(&mut string_vars, &matrix);
        context.insert("variables", &string_vars);

        let target_file_path_buf = Path::new(&self.output_dir).join(format!("{}.sh", matrix_name));

        let target_file =
            File::create(&target_file_path_buf).expect("Unable to create job file");

        make_executable(&target_file_path_buf);

        match TERA.render_to("ci/matrix.sh", &context, &target_file) {
            Err(e) => {
                println!("can't render file: {}", e);
            }
            _ => {
                println!("file generated: {:?}", &target_file_path_buf);
            }
        };
    }

    fn gen_job(&self, job_name: &str, job: &GitlabJob, parallel: &str) {
        let mut context = Context::new();
        context.insert("job_name", job_name);
        context.insert("job", job);
        context.insert("is_template", &job_name.starts_with("."));
        context.insert("variables", &job.variables_as_strings());
        context.insert("parallel", &parallel);

        let target_file_path_buf = Path::new(&self.output_dir).join(format!("{}.sh", job_name));

        let target_file =
            File::create(&target_file_path_buf).expect("Unable to create job file");

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

pub fn make_executable(target_file_path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = fs::set_permissions(
            target_file_path.as_os_str(),
            fs::Permissions::from_mode(0o700),
        ) {
            println!("can't set permissions to file: {}", e);
        }
    }
}
