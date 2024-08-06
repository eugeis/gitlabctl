use clap::Parser;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::{fmt, fs};
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::de::Visitor;
use serde_yaml::{from_str, from_value, Value};

#[derive(Debug, Clone)]
enum VariableValue {
    String(String),
    Int(i32),
    Bool(bool),
}

// Implement custom deserialization for VariableValue
impl<'de> Deserialize<'de> for VariableValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct VariableValueVisitor;

        impl<'de> Visitor<'de> for VariableValueVisitor {
            type Value = VariableValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string, an integer, or a boolean")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                Ok(VariableValue::String(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                Ok(VariableValue::String(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                Ok(VariableValue::Int(value as i32))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                Ok(VariableValue::Bool(value))
            }
        }

        deserializer.deserialize_any(VariableValueVisitor)
    }
}

// Implement ToString for VariableValue
impl Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            VariableValue::String(value) => value.clone(),
            VariableValue::Int(value) => value.to_string(),
            VariableValue::Bool(value) => value.to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Extends {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
struct GitlabJob {
    script: Option<Vec<String>>,
    extends: Option<Extends>, // Changed to Option<Extends>
    variables: Option<HashMap<String, VariableValue>>,
    stage: Option<String>,
    when: Option<String>,
    only: Option<HashMap<String, Vec<String>>>,
    needs: Option<Vec<String>>,
    image: Option<Image>,
    before_script: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Image {
    name: String,
    entrypoint: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct GitlabCi {
    include: Option<Vec<Include>>,
    stages: Option<Vec<String>>,
    jobs: HashMap<String, GitlabJob>,
}

impl GitlabCi {
    pub fn insert_job(&mut self, name: String, job: GitlabJob) {
        self.jobs.insert(name, job);
    }

    pub fn resolve_job(&self, job_name: String) -> Option<GitlabJob> {
        let mut job = self.jobs.get(&job_name)?.clone(); // Return None if job does not exist

        // Substitute variables in the script
        if let Some(variables) = &job.variables {
            if let Some(script) = &mut job.script {
                for command in script.iter_mut() {
                    for (var, value) in variables {
                        *command = command.replace(&format!("${{{}}}", var), &*value.to_string());
                    }
                }
            }
        }

        Some(job)
    }
}

#[derive(Debug, Deserialize)]
struct Include {
    project: String,
    #[serde(rename = "ref")]
    ref_: String,
    file: Vec<String>, // Simplified structure
}

#[derive(Parser, Debug)]
pub struct GenerateBashScriptsCommand {
    /// Path to the gitlab-ci.yml file
    #[arg(short, long)]
    pub yml_file: String,

    /// Output directory for generated bash scripts
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,
}
impl GenerateBashScriptsCommand {
    pub fn run(&self) {
        // Read and parse the gitlab-ci.yml file
        let yml_content = fs::read_to_string(&self.yml_file)
            .expect("Unable to read gitlab-ci.yml file");

        let yaml_value: Value = from_str(&yml_content).expect("Unable to parse gitlab-ci.yml file");

        let mut gitlab_ci: GitlabCi = GitlabCi {
            include: None,
            stages: Default::default(),
            jobs: Default::default(),
        };

        // Parse dynamic jobs and store them in a hashmap
        let mut jobs: HashMap<String, GitlabJob> = HashMap::new();
        if let Some(mapping) = yaml_value.as_mapping() {
            for (key, value) in mapping {
                if let Some(key_str) = key.as_str() {
                    if key_str == "include" {
                        gitlab_ci.include = from_value(value.clone()).expect("Failed to deserialize 'include' field");
                    } else if key_str == "stages" {
                        gitlab_ci.stages = Some(value.as_sequence()
                            .expect("Invalid 'stages' format")
                            .iter()
                            .map(|v| v.as_str().expect("Invalid stage name").to_owned())
                            .collect());
                    } else {
                        let job: GitlabJob = from_value(value.clone()).expect("Failed to deserialize job");
                        gitlab_ci.insert_job(key_str.to_owned(), job);
                    }
                }
            }
        } else {
            panic!("Invalid YAML format");
        }

        // Create the output directory if it doesn't exist
        if !Path::new(&self.output_dir).exists() {
            fs::create_dir_all(&self.output_dir)
                .expect("Unable to create output directory");
        }

        // Generate bash scripts for each job
        for (job_name, job) in gitlab_ci.jobs.iter() {
            if let Some(script) = &job.script {
                let script_file_path = format!("{}/{}.sh", &self.output_dir, job_name);
                let mut file = File::create(&script_file_path)
                    .expect("Unable to create bash script file");

                // Write the script commands to the file
                for command in script {
                    writeln!(file, "{}", command)
                        .expect("Unable to write to bash script file");
                }

                // Make the script executable
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
}
