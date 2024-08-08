// parser.rs
use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Display};
use serde::de::Visitor;
use serde_yaml::{from_str, from_value, Value};

#[derive(Debug, Clone, Serialize)]
pub enum VariableValue {
    String(String),
    Int(i32),
    Bool(bool),
}

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

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Extends {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GitlabJob {
    pub script: Option<Vec<String>>,
    pub extends: Option<Extends>,
    pub variables: Option<HashMap<String, VariableValue>>,
    pub stage: Option<String>,
    pub when: Option<String>,
    pub only: Option<HashMap<String, Vec<String>>>,
    pub needs: Option<Vec<String>>,
    pub image: Option<Image>,
    pub before_script: Option<Vec<String>>,
}

impl GitlabJob {
    pub fn variables_as_strings(&self) -> HashMap<String, String> {
        let mut string_vars = HashMap::new();
        if let Some(vars) = &self.variables {
            for (key, value) in vars {
                string_vars.insert(key.clone(), value.to_string());
            }
        }
        string_vars
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Image {
    pub name: String,
    pub entrypoint: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitlabCi {
    pub include: Option<Vec<Include>>,
    pub stages: Option<Vec<String>>,
    pub jobs: HashMap<String, GitlabJob>,
}

impl GitlabCi {
    pub fn insert_job(&mut self, name: String, job: GitlabJob) {
        self.jobs.insert(name, job);
    }

    pub fn resolve_job(&self, job_name: &str) -> Option<GitlabJob> {
        let mut job = self.jobs.get(job_name)?.clone();

        if let Some(extends) = job.extends.clone() {
            match extends {
                Extends::Single(parent) => {
                    if let Some(parent_job) = self.resolve_job(&parent) {
                        job = self.merge_jobs(parent_job, job);
                    }
                }
                Extends::Multiple(parents) => {
                    for parent in parents {
                        if let Some(parent_job) = self.resolve_job(&parent) {
                            job = self.merge_jobs(parent_job, job);
                        }
                    }
                }
            }
        }
        Some(job)
    }

    fn merge_jobs(&self, parent: GitlabJob, mut child: GitlabJob) -> GitlabJob {
        if child.script.is_none() {
            child.script = parent.script.clone();
        } else {
            if let Some(parent_script) = parent.script {
                if let Some(child_script) = &mut child.script {
                    child_script.splice(0..0, parent_script);
                }
            }
        }

        if child.variables.is_none() {
            child.variables = parent.variables.clone();
        } else {
            if let Some(parent_vars) = parent.variables {
                if let Some(child_vars) = &mut child.variables {
                    child_vars.extend(parent_vars);
                }
            }
        }

        child
    }

    pub fn collect_undefined_env_vars(&self) -> Vec<String> {
        let mut defined_vars: HashMap<String, bool> = HashMap::new();
        let mut undefined_vars: Vec<String> = Vec::new();

        for job in self.jobs.values() {
            if let Some(vars) = &job.variables {
                for key in vars.keys() {
                    defined_vars.insert(key.clone(), true);
                }
            }
        }

        for job in self.jobs.values() {
            if let Some(script) = &job.script {
                for command in script {
                    for (var, _) in defined_vars.iter() {
                        if command.contains(&format!("${{{}}}", var)) && !defined_vars.contains_key(var) {
                            undefined_vars.push(var.clone());
                        }
                    }
                }
            }
        }

        undefined_vars
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Include {
    pub project: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub file: Vec<String>,
}

pub fn parse_gitlab_ci(yml_content: &str) -> GitlabCi {
    let yaml_value: Value = from_str(&yml_content).expect("Unable to parse gitlab-ci.yml file");

    let mut gitlab_ci: GitlabCi = GitlabCi {
        include: None,
        stages: Default::default(),
        jobs: Default::default(),
    };

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

    gitlab_ci
}
