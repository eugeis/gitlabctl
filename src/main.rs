mod gitlab;
mod gitlab_types;
mod handler;
mod common;
mod gen;
mod yaml;

extern crate tera;
extern crate lazy_static;

use clap::{Parser, Subcommand};
use crate::handler::{FsModelHandler, Handler};
use crate::yaml::YamlWriter;
use crate::gen::Generator;
use std::boxed::Box;
use std::fs;
use ::gitlab::api::{Query};
use ::gitlab::api::projects::pipelines::{DeletePipeline, Pipelines};
use ::gitlab::{api, Gitlab};
use resolve_path::PathResolveExt;
use crate::gitlab_types::{PipelineSchema};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Gitlab URL
    #[arg(short, long, default_value = "code.siemens.com")]
    url: String,

    /// Gitlab token file for update_scripts.sh path
    #[arg(short, long, default_value = "~/.ssh/gitlab_token_read")]
    token_file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate Git scripts
    GenerateGitScripts {
        /// Gitlab Groups to read
        #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',', required = true)]
        groups: Vec<String>,

        /// Base dir for generation
        #[arg(short, long, default_value = ".")]
        output_dir: String,

        /// Generate Git scripts
        #[arg(long, default_value = "clone.sh,pull.sh,status.sh,scripts.sh", value_delimiter = ',', num_args = 1..)]
        generate_templates: Vec<String>,
    },
    /// Show Pipelines
    ShowPipelines {
        /// Project path to show pipelines for
        #[arg(short, long)]
        project_path: String,
    },
    /// Delete Pipelines
    DeletePipelines {
        /// Project path to delete pipelines from
        #[arg(short, long)]
        project_path: String,
    },
}

fn main() {
    let args = Args::parse();
    let gitlab_token_file_path = args.token_file.resolve();
    let gitlab_token: String = fs::read_to_string(gitlab_token_file_path)
        .expect("can't read gitlab token file")
        .trim()
        .to_string();

    match &args.command {
        Commands::GenerateGitScripts { groups, output_dir, generate_templates } => {
            let handler = build_script_generator(generate_templates.clone(), args.token_file.clone());

            let fs_handler = FsModelHandler {
                output_dir: output_dir.clone(),
                handle_children: true,
                handler,
            };

            match gitlab::GroupNodeReader::new(&args.url, &gitlab_token) {
                Ok(mut reader) => {
                    for group in groups {
                        println!("read group: {:?}", group);
                        match reader.read(group) {
                            Ok(node) => {
                                fs_handler.on_node(&node).expect("can't write model")
                            },
                            Err(err) => println!("can't read the group '{}': {}", group, err),
                        }
                    }
                }
                Err(err) => panic!("can't connect to Gitlab: {}", err),
            };
        }
        Commands::ShowPipelines { project_path } => {
            match Gitlab::new(&args.url, &gitlab_token) {
                Ok(gitlab) => {
                    let pipelines: Vec<PipelineSchema> = Pipelines::builder()
                        .project(project_path)
                        .build()
                        .unwrap()
                        .query(&gitlab)
                        .unwrap();

                    for pipeline in pipelines.iter() {
                        println!("Pipeline ID: {:?}", pipeline.id);
                    }
                }
                Err(err) => println!("Error connecting to GitLab: {:?}", err),
            }
        }
        Commands::DeletePipelines { project_path } => {
            match Gitlab::new(&args.url, &gitlab_token) {
                Ok(gitlab) => {
                    let pipelines: Vec<PipelineSchema> = Pipelines::builder()
                        .project(project_path)
                        .build()
                        .unwrap()
                        .query(&gitlab)
                        .unwrap();

                    for pipeline in pipelines.iter() {
                        println!("Deleting pipeline {:?}", pipeline.id);

                        let endpoint = DeletePipeline::builder()
                            .project(project_path)
                            .pipeline(pipeline.id)
                            .build()
                            .unwrap();
                        api::ignore(endpoint).query(&gitlab).expect("delete failed");
                    }
                }
                Err(err) => println!("Error connecting to GitLab: {:?}", err),
            }
        }
    }
}

fn build_yaml_writer() -> Box<dyn Handler> {
    Box::new(YamlWriter {
        model_file_name: ".gitlab.yaml".to_string(),
    })
}

fn build_script_generator(templates: Vec<String>, gitlab_token_file: String) -> Box<dyn Handler> {
    Box::new(Generator { templates, gitlab_token_file })
}
