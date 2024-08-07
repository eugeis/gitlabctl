// generator.rs
use std::{fs, io::Write, path::Path};
use std::fs::File;
use clap::Args;
use crate::ci_parser::{GitlabCi, parse_gitlab_ci};

#[derive(Args, Debug)]
pub struct GenerateBashScriptsCommand {
    #[arg(short, long)]
    pub yml_file: String,
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,
}

impl GenerateBashScriptsCommand {
    pub fn run(&self) {
        let yml_content = fs::read_to_string(&self.yml_file)
            .expect("Unable to read gitlab-ci.yml file");

        let gitlab_ci = parse_gitlab_ci(&yml_content);

        gitlab_ci.generate_scripts(&self.output_dir);
    }
}

impl GitlabCi {
    pub fn generate_scripts(&self, output_dir: &str) {
        // Create the output directory if it doesn't exist
        if !Path::new(output_dir).exists() {
            fs::create_dir_all(output_dir)
                .expect("Unable to create output directory");
        }

        // Collect undefined environment variables
        let undefined_vars = self.collect_undefined_env_vars();

        // Create .env.example file with undefined variables
        if !undefined_vars.is_empty() {
            let mut env_example = File::create(format!("{}/.env.example", output_dir))
                .expect("Unable to create .env.example file");
            for var in &undefined_vars {
                writeln!(env_example, "{}=", var)
                    .expect("Unable to write to .env.example file");
            }
        }

        // Generate scripts for each job
        for (job_name, _) in self.jobs.iter() {
            let script_file_path = format!("{}/{}.sh", output_dir, job_name);
            let mut file = File::create(&script_file_path)
                .expect("Unable to create bash script file");

            if let Some(job) = self.resolve_job(job_name) {
                if let Some(script) = &job.script {
                    for command in script {
                        writeln!(file, "{}", command)
                            .expect("Unable to write to bash script file");
                    }
                }

                for var in &undefined_vars {
                    if job.script.iter().any(|commands| commands.iter().any(|cmd| cmd.contains(&format!("${{{}}}", var)))) {
                        writeln!(file, "source .env")
                            .expect("Unable to write to bash script file");
                        break;
                    }
                }
            } else {
                writeln!(file, "echo 'Warning: Job {} could not be resolved'", job_name)
                    .expect("Unable to write to bash script file");
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&script_file_path)
                    .expect("Unable to read script file metadata")
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&script_file_path, perms)
                    .expect("Unable to set script file permissions");
            }

            println!("Generated script for job '{}': {}", job_name, script_file_path);
        }
    }
}

