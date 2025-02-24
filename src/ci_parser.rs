use indexmap::IndexMap;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_yaml::{from_str, from_value, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{self, Display};

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

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VariableValue::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VariableValue::Int(value as i32))
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
pub struct Variable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Extends {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, Serialize)]
pub struct GitlabJob {
    pub script: Option<Vec<String>>,
    pub extends: Option<Extends>,
    pub variables: Option<BTreeMap<String, VariableValue>>,
    pub stage: Option<String>,
    pub when: Option<String>,
    pub only: Option<HashMap<String, Vec<String>>>,
    pub needs: Option<Vec<String>>,
    pub image: Option<Image>,
    pub before_script: Option<Vec<String>>,
    pub parallel: Option<ParallelField>,
}

impl<'de> Deserialize<'de> for GitlabJob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawGitlabJob {
            script: Option<Vec<String>>,
            extends: Option<Extends>,
            variables: Option<BTreeMap<String, VariableValue>>,
            stage: Option<String>,
            when: Option<String>,
            only: Option<HashMap<String, Vec<String>>>,
            needs: Option<Vec<String>>,
            image: Option<Image>,
            before_script: Option<Vec<String>>,
            parallel: Option<Value>, // Deserialize parallel as a Value first
        }

        let raw_job = RawGitlabJob::deserialize(deserializer)?;

        let parallel_field = raw_job.parallel.map(|parallel_value| {
            from_value::<ParallelField>(parallel_value.clone())
                .or_else(|_| from_value::<ParallelReference>(parallel_value.clone())
                    .map(ParallelField::Reference))
                .or_else(|_| from_value::<ParallelMatrix>(parallel_value.clone())
                    .map(ParallelField::Matrix))
                .map_err(de::Error::custom)
        }).transpose()?;

        Ok(GitlabJob {
            script: raw_job.script,
            extends: raw_job.extends,
            variables: raw_job.variables,
            stage: raw_job.stage,
            when: raw_job.when,
            only: raw_job.only,
            needs: raw_job.needs,
            image: raw_job.image,
            before_script: raw_job.before_script,
            parallel: parallel_field,
        })
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum ParallelField {
    Reference(ParallelReference),
    Matrix(ParallelMatrix),
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ParallelReference {
    pub matrix: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ParallelMatrix {
    pub matrix: Vec<BTreeMap<String, VariableValue>>,
}

impl GitlabJob {
    pub fn variables_as_strings(&self) -> Vec<Variable> {
        let mut string_vars = Vec::new();
        if let Some(vars) = &self.variables {
            for (key, value) in vars {
                string_vars.push(Variable {
                    name: key.clone(),
                    value: value.to_string(),
                });
            }
        }
        string_vars
    }
}

pub fn variables_as_string_fill(string_vars: &mut Vec<Variable>, vars: &BTreeMap<String, VariableValue>) {
    for (key, value) in vars {
        string_vars.push(Variable {
            name: key.clone(),
            value: value.to_string(),
        });
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
    pub matrices: Option<IndexMap<String, Vec<BTreeMap<String, VariableValue>>>>,
    pub jobs: BTreeMap<String, GitlabJob>,
}

impl GitlabCi {
    pub fn insert_job(&mut self, name: String, job: GitlabJob) {
        self.jobs.insert(name, job);
    }

    pub fn collect_unresolved_extends(&self) -> HashSet<String> {
        let mut unresolved_extends = HashSet::new();
        let all_job_names: HashSet<_> = self.jobs.keys().cloned().collect();

        for job in self.jobs.values() {
            if let Some(extends) = &job.extends {
                match extends {
                    Extends::Single(job_name) => {
                        if !all_job_names.contains(job_name) {
                            unresolved_extends.insert(job_name.clone());
                        }
                    }
                    Extends::Multiple(job_names) => {
                        for job_name in job_names {
                            if !all_job_names.contains(job_name) {
                                unresolved_extends.insert(job_name.clone());
                            }
                        }
                    }
                }
            }
        }

        unresolved_extends
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
    let yaml_value: Value = from_str(yml_content).expect("Unable to parse gitlab-ci.yml file");

    let mut gitlab_ci: GitlabCi = GitlabCi {
        include: None,
        stages: Default::default(),
        matrices: Default::default(),
        jobs: Default::default(),
    };

    if let Some(mapping) = yaml_value.as_mapping() {
        for (key, value) in mapping {
            if let Some(key_str) = key.as_str() {
                if key_str == "include" {
                    gitlab_ci.include =
                        from_value(value.clone()).expect("Failed to deserialize 'include' field");
                } else if key_str == "stages" {
                    gitlab_ci.stages = Some(
                        value
                            .as_sequence()
                            .expect("Invalid 'stages' format")
                            .iter()
                            .map(|v| v.as_str().expect("Invalid stage name").to_owned())
                            .collect(),
                    );
                } else {
                    match from_value::<GitlabJob>(value.clone()) {
                        Ok(job) => gitlab_ci.insert_job(key_str.to_owned(), job),
                        Err(_) => match from_value::<Vec<BTreeMap<String, VariableValue>>>(value.clone()) {
                            Ok(matrix_vec) => {
                                if let Some(ref mut matrices) = gitlab_ci.matrices {
                                    matrices.insert(key_str.to_owned(), matrix_vec);
                                } else {
                                    let mut matrices = IndexMap::new();
                                    matrices.insert(key_str.to_owned(), matrix_vec);
                                    gitlab_ci.matrices = Some(matrices);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize job or matrix: {}", e)
                            },
                        },
                    }
                }
            }
        }
    } else {
        panic!("Invalid YAML format");
    }

    gitlab_ci
}
